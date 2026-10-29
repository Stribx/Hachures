//! What play will draw, drawn once behind the splash: a tower of each weapon, a figure of each
//! species, the ghost's pale ink, a panel's shadow and the gizmos' lines. The GPU compiles their pipelines then, not
//! on the first tower inked or the first wave.

use bevy::prelude::*;

use crate::combat::figure::{EnemyLook, spawn_figure};
use crate::combat::stand::{BASE_WOOD, TOWER_BASE, WEAPON_WOOD, weapon_model};
use crate::combat::towers::Weapon;
use crate::level::Level;
use crate::look::{self, Hachure, InkMaterial};
use crate::map;
use crate::schedule::Screen;
use crate::theme::PANEL_SHADOW;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn, gizmos).run_if(in_state(Screen::Splash)))
        .add_systems(OnExit(Screen::Splash), clear);
}

/// A piece drawn only for the warm-up.
#[derive(Component)]
struct Warmup;

/// Once the level and its figures are ready, every piece at the board's middle, where the
/// camera already looks behind the splash.
fn spawn(
    level: Res<Level>,
    look: Option<Res<EnemyLook>>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
    mut done: Local<bool>,
    mut commands: Commands,
) {
    let Some(look) = look.filter(|_| !*done) else { return };
    *done = true;
    let grid = level.terrain.grid();
    let at = map::cell_pos(grid, grid.tiles.len() / 2);
    let scene =
        |path: &'static str| WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(path)));
    commands.spawn((Warmup, scene(TOWER_BASE), BASE_WOOD, Transform::from_translation(at)));
    let models = [Weapon::Arbalete, Weapon::Mortier, Weapon::Harpon, Weapon::Lanterne]
        .into_iter()
        .filter_map(weapon_model);
    for path in models {
        let top = Transform::from_translation(at + Vec3::Y * 0.6);
        commands.spawn((Warmup, scene(path), WEAPON_WOOD, top));
    }
    for index in 0..look.figures.len() {
        let figure = spawn_figure(&mut commands, &look, index, at, 1.0);
        commands.entity(figure).insert(Warmup);
    }
    // The ghost's piece: see-through, its colours on its vertices, as `ink::ghost` makes it.
    let mut pale = look::ink(Color::WHITE.with_alpha(0.45));
    pale.base.alpha_mode = AlphaMode::Blend;
    pale.extension = Hachure::fine();
    let cube = Mesh::from(Cuboid::from_length(0.3));
    let n = cube.count_vertices();
    let cube = cube.with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1.0; 4]; n]);
    commands.spawn((
        Warmup,
        Mesh3d(meshes.add(cube)),
        MeshMaterial3d(materials.add(pale)),
        Transform::from_translation(at),
    ));
    // A panel's shadow, as the title's menu casts it: under the splash's card, unseen.
    commands.spawn((
        Warmup,
        Node { width: px(1), height: px(1), ..default() },
        BoxShadow::new(PANEL_SHADOW, px(0), px(3), px(0), px(12)),
        GlobalZIndex(i32::MIN + 1),
    ));
}

/// A line and a ring, as the range circles and the overlay draw them.
fn gizmos(level: Res<Level>, mut gizmos: Gizmos) {
    let at = map::cell_pos(level.terrain.grid(), level.terrain.grid().tiles.len() / 2);
    gizmos.line(at, at + Vec3::Y, Color::BLACK);
    gizmos.circle(
        Isometry3d::new(at, Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        0.5,
        Color::BLACK,
    );
}

fn clear(pieces: Query<Entity, With<Warmup>>, mut commands: Commands) {
    for piece in &pieces {
        commands.entity(piece).despawn();
    }
}
