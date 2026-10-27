//! Dev tool: F12 saves a screenshot to `screenshots/`. With `HACHURES_SHOT=out.png` the game jumps into play
//! (`HACHURES_SCREEN=title|plan|paused|settings|controls|credits`, `HACHURES_LANG=en`,
//! `HACHURES_LEVEL=n`, `HACHURES_DEMO=1` for one tower of each kind and a few barricades, `HACHURES_FASCINES=1` with that skill, `HACHURES_WAVE=1` to send the first wave), saves one screenshot after
//! `HACHURES_AT` seconds (default 4) and quits: before/after comparisons without clicking. `HACHURES_FPS=1` logs,
//! each second, the mean and the worst frame time (a stutter is a worst frame far above the mean), and the GPU time
//! of each render pass (`render/<pass>/elapsed_gpu`, Vulkan and DX12 only): frame times alone can sit on the
//! compositor's 60 Hz whatever the GPU does.

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk};

use crate::campaign::LoadLevel;
use crate::campaign::skills::Skill;
use crate::combat::enemies::send_wave;
use crate::combat::towers::{Weapon, spawn_tower};
use crate::level::Level;
use crate::look::WorldCamera;
use crate::map::grid::Tile;
use crate::map::terrain::Edit;
use crate::overworld::plan::WalkTo;
use crate::overworld::travel::{Travel, Trip};
use crate::schedule::Screen;
use crate::settings::Settings;

/// Where to save the one screenshot, and the sign to jump into play.
const SHOT: &str = "HACHURES_SHOT";

pub fn plugin(app: &mut App) {
    app.add_systems(Update, f12);
    if std::env::var("HACHURES_FPS").is_ok() {
        app.add_systems(Last, frame_times).add_plugins((
            bevy::render::diagnostic::RenderDiagnosticsPlugin,
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
        ));
    }
    if std::env::var(SHOT).is_ok() {
        app.add_systems(Startup, jump).add_systems(Update, (demo, act, timed));
        app.add_systems(PostUpdate, camera.before(TransformSystems::Propagate));
    }
}

fn f12(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::F12) {
        let path = format!("screenshots/shot-{:.0}.png", time.elapsed_secs() * 1000.0);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
}

/// Frames this second: their count, their total and the longest, in seconds.
fn frame_times(time: Res<Time<Real>>, trip: Option<Res<Trip>>, mut second: Local<(u32, f32, f32)>) {
    let dt = time.delta_secs();
    // A hitch, with when it fell: during a trip, the clouds hide it only while they are closed.
    if dt > 0.05 {
        let phase = trip.map(|t| format!("{:?}", t.phase));
        info!("slow frame: {:.0} ms at {:.2} s, trip {phase:?}", dt * 1000.0, time.elapsed_secs());
    }
    let (count, total, worst) = &mut *second;
    (*count, *total, *worst) = (*count + 1, *total + dt, worst.max(dt));
    if *total >= 1.0 {
        info!("frames: {count}, mean {:.1} ms, worst {:.1} ms", *total / *count as f32 * 1000.0, *worst * 1000.0);
        *second = (0, 0.0, 0.0);
    }
}

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

fn jump(
    mut next: ResMut<NextState<Screen>>,
    mut load: MessageWriter<LoadLevel>,
    mut settings: ResMut<Settings>,
    mut commands: Commands,
) {
    let screen = env("HACHURES_SCREEN").unwrap_or_else(|| "playing".into());
    next.set(match screen.as_str() {
        "title" => Screen::Title,
        "plan" => Screen::Plan,
        "paused" => Screen::Paused,
        "settings" => Screen::Settings,
        "controls" => Screen::Controls,
        "credits" => Screen::Credits,
        _ => Screen::Playing,
    });
    // The pages opened from the title come back to it.
    commands.insert_resource(crate::menu::BackTo(Screen::Title));
    // The shot is taken in a window that never has the focus: play must not pause for it.
    settings.pause_unfocused = false;
    // `HACHURES_LANG=en` (any code of `i18n::LANGUAGES`): shown in that language, the save left as it is on disk.
    if let Some(code) = env("HACHURES_LANG") {
        // As `settings::plugin` does with the saved language, before the first page is written.
        crate::i18n::set(&code);
        settings.language = code;
    }
    if let Some(n) = env("HACHURES_LEVEL").and_then(|n| n.parse().ok()) {
        load.write(LoadLevel(n));
    }
}

/// One tower of each weapon on the middle row's first open cells, each turned a different way.
fn demo(
    mut done: Local<u32>,
    mut level: ResMut<Level>,
    settings: Res<Settings>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    // Wait for the level asked for to be loaded.
    *done += 1;
    if *done != 3 || env("HACHURES_DEMO").is_none() {
        return;
    }
    if env("HACHURES_WAVE").is_some() {
        send_wave(&mut level, &mut commands, &assets, &settings);
    }
    let grid = level.terrain.grid();
    let open = (0..grid.tiles.len()).filter(|&i| grid.tiles[i] == Tile::Sol && grid.coords(i).1 == 2);
    let weapons = [Weapon::Arbalete, Weapon::Mortier, Weapon::Harpon, Weapon::Lanterne];
    for (i, weapon) in open.step_by(3).zip(weapons) {
        spawn_tower(&mut commands, &assets, grid, i, weapon);
    }
    // And barricades to see their props: a wall of three with a branch down (a T), and one alone further on.
    // `HACHURES_FASCINES=1` learns the skill that shows on them.
    let width = grid.width;
    let wall = |i: usize| [i, i + 1, i + 2, i + 1 + width, i + 5];
    let start = (0..grid.tiles.len()).find(|&i| {
        let (x, z) = grid.coords(i);
        z == 1 && x + 5 < width && wall(i).iter().all(|&j| grid.tiles[j] == Tile::Sol)
    });
    let walls: Vec<(usize, Edit)> = start.into_iter().flat_map(wall).map(|j| (j, Edit::Ink(Tile::Barricade))).collect();
    if env("HACHURES_FASCINES").is_some() {
        level.skills.push(Skill::Fascines);
    }
    // Even with no wall, the cells are drawn again: the skill shows on them.
    level.alter(walls);
}

/// `HACHURES_CAM=x,y,z,tx,ty,tz`: the camera there, looking at the target.
fn camera(mut camera: Single<&mut Transform, With<WorldCamera>>) {
    let Some(v) = env("HACHURES_CAM").map(|s| s.split(',').filter_map(|n| n.parse().ok()).collect::<Vec<f32>>()) else {
        return;
    };
    **camera = Transform::from_xyz(v[0], v[1], v[2]).looking_at(Vec3::new(v[3], v[4], v[5]), Vec3::Y);
}

/// `HACHURES_WALK=n@t` and `HACHURES_TRAVEL=n@t`: at `t` seconds, walk Nève to sheet `n`, or go and play it.
/// `HACHURES_WALK=n@t` and `HACHURES_TRAVEL=n@t` (above); `HACHURES_INK=t`: at `t` seconds, ink a barricade and an
/// arbalète on the third row, as a click would, to see them land.
fn act(
    time: Res<Time>,
    mut done: Local<[bool; 3]>,
    mut walks: MessageWriter<WalkTo>,
    mut travel: MessageWriter<Travel>,
    mut level: ResMut<Level>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    let due = |name: &str| {
        let (n, t) = env(name)?.split_once('@').map(|(n, t)| (n.parse().ok(), t.parse::<f32>().ok()))?;
        (time.elapsed_secs() >= t?).then_some(n?)
    };
    if let (false, Some(n)) = (done[0], due("HACHURES_WALK")) {
        done[0] = true;
        walks.write(WalkTo(n));
    }
    if let (false, Some(n)) = (done[1], due("HACHURES_TRAVEL")) {
        done[1] = true;
        travel.write(Travel { to: n, again: false });
    }
    let ink_at = env("HACHURES_INK").and_then(|t| t.parse::<f32>().ok());
    if !done[2] && ink_at.is_some_and(|t| time.elapsed_secs() >= t) {
        done[2] = true;
        let grid = level.terrain.grid();
        let open: Vec<usize> =
            (0..grid.tiles.len()).filter(|&i| grid.tiles[i] == Tile::Sol && grid.coords(i).1 == 3).collect();
        for (tool, &i) in [0, 3].into_iter().zip(open.iter().step_by(4)) {
            level.ink(i, tool);
            if let Some(weapon) = crate::ink::TOOLS[tool].act.weapon() {
                spawn_tower(&mut commands, &assets, level.terrain.grid(), i, weapon);
            }
        }
    }
}

/// `HACHURES_AT=t` or `t1,t2,…`: one screenshot at each time (`out-1.png`, … when several), then quit.
fn timed(time: Res<Time>, mut taken: Local<usize>, mut commands: Commands) {
    let times: Vec<f32> =
        env("HACHURES_AT").map_or(vec![4.0], |s| s.split(',').filter_map(|t| t.parse().ok()).collect());
    if *taken >= times.len() || time.elapsed_secs() < times[*taken] {
        return;
    }
    *taken += 1;
    let path = env(SHOT).unwrap();
    let path = if times.len() > 1 { path.replace(".png", &format!("-{}.png", *taken)) } else { path };
    let mut shot = commands.spawn(Screenshot::primary_window());
    shot.observe(save_to_disk(path));
    if *taken == times.len() {
        shot.observe(|_: On<ScreenshotCaptured>, mut exit: MessageWriter<AppExit>| {
            exit.write(AppExit::Success);
        });
    }
}
