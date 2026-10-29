//! Puts the settings into effect: the language, the sound, the window, the light, the lens and the
//! UI scale. Each system runs when the settings change, and when the window gains or loses focus
//! (for the sound) or changes size (for the scale).

use bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening;
use bevy::audio::AudioSinkPlayback;
use bevy::camera::{ImageRenderTarget, RenderTarget};
use bevy::ecs::system::SystemParam;
use bevy::post_process::dof::DepthOfField;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::window::{MonitorSelection, PresentMode, PrimaryWindow, WindowMode};

use super::{DynamicScale, Music, Settings};
use crate::i18n;
#[cfg(not(target_arch = "wasm32"))]
use crate::look::occlusion;
use crate::look::{Frame, InkMaterial, Sun, WorldCamera, lens};
use crate::schedule::Screen;
use crate::theme::{FELL, READABLE};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            apply_audio,
            apply_window,
            apply_render,
            apply_cloud_shadows,
            apply_ui_scale,
            swap_fonts,
            pause_unfocused,
        ),
    );
}

/// The settings, and when to read them again: they changed, or the window's focus or size did, or
/// the screen (the title draws a smaller world), or the dynamic resolution's share.
#[derive(SystemParam)]
struct Due<'w, 's> {
    settings: Res<'w, Settings>,
    screen: Res<'w, State<Screen>>,
    dynamic: Res<'w, DynamicScale>,
    seen: Local<'s, Option<(bool, Vec2, UVec2)>>,
}

impl Due<'_, '_> {
    fn check(&mut self, window: &Window) -> Option<&Settings> {
        let now = (window.focused, window.size(), window.physical_size());
        // Every frame, so the window's last state is always the one seen.
        let moved = self.seen.replace(now) != Some(now);
        let due =
            self.settings.is_changed() || self.screen.is_changed() || self.dynamic.is_changed();
        (moved || due).then(|| &*self.settings)
    }
}

/// The language and the music's volume, which follows the focus.
fn apply_audio(
    mut due: Due,
    window: Single<&Window, With<PrimaryWindow>>,
    mut music: Query<&mut AudioSink, With<Music>>,
) {
    let Some(settings) = due.check(&window) else { return };
    i18n::set(&settings.language);
    for mut sink in &mut music {
        sink.set_volume(settings.music_volume(window.focused));
    }
}

fn apply_window(mut due: Due, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    let Some(settings) = due.check(&window) else { return };
    let mode = if settings.fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
    let present = if settings.vsync { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync };
    if window.mode != mode {
        window.mode = mode;
    }
    if window.present_mode != present {
        window.present_mode = present;
    }
}

/// The world's picture on the title, at most, as a share of the window across: the lens blurs it
/// there anyway. A render choice, not a rule: it takes a third off the title's GPU time, and the
/// hatching still reads (at 0.5 it smeared into dashes).
const TITLE_SCALE: f32 = 0.7;

/// The world's picture, as a share of the window: no more than `TITLE_SCALE` on the screens the
/// title's blurred Marches stand behind; elsewhere the player's render scale, times the dynamic
/// resolution's share (`settings::dynamic`).
fn world_scale(render_scale: u32, dynamic: DynamicScale, screen: Screen) -> f32 {
    let asked = render_scale as f32 / 100.0;
    match screen {
        Screen::Splash | Screen::Title | Screen::ConfirmNew => asked.min(TITLE_SCALE),
        _ => asked * dynamic.0 as f32 / 100.0,
    }
}

/// The board's picture and what is drawn into it: the render scale, the shadows, the occlusion, the
/// anti-aliasing and the lens.
fn apply_render(
    mut due: Due,
    window: Single<&Window, With<PrimaryWindow>>,
    mut suns: Query<&mut DirectionalLight, With<Sun>>,
    camera: Single<(Entity, Has<DepthOfField>), With<WorldCamera>>,
    frame: Res<Frame>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let (screen, dynamic) = (*due.screen.get(), *due.dynamic);
    let Some(settings) = due.check(&window) else { return };
    // The board's picture at the render scale. Its own scale factor shrinks with it, so it keeps
    // the window's logical size: picking and the place labels need no conversion.
    let scale = world_scale(settings.render_scale, dynamic, screen);
    let wanted = (window.physical_size().as_vec2() * scale).as_uvec2().max(UVec2::ONE);
    // `get_mut` marks the image changed, so only when it does.
    if images.get(&frame.0).is_some_and(|image| image.size() != wanted) {
        let size = Extent3d { width: wanted.x, height: wanted.y, depth_or_array_layers: 1 };
        images.get_mut(&frame.0).expect("read just above").resize(size);
    }

    let target =
        ImageRenderTarget { handle: frame.0.clone(), scale_factor: window.scale_factor() * scale };
    commands.entity(camera.0).insert(RenderTarget::Image(target));
    // Stretched over the window, a smaller picture blurs: sharpened back, as a game's upscaler
    // would. The title's too, so the splash compiles the sharpening, not a first drop in play.
    if scale < 1.0 {
        commands.entity(camera.0).insert(ContrastAdaptiveSharpening::default());
    } else {
        commands.entity(camera.0).remove::<ContrastAdaptiveSharpening>();
    }
    for mut sun in &mut suns {
        if sun.shadow_maps_enabled != settings.shadows {
            sun.shadow_maps_enabled = settings.shadows;
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use bevy::anti_alias::fxaa::Fxaa;
        use bevy::anti_alias::smaa::Smaa;
        use bevy::pbr::ScreenSpaceAmbientOcclusion;
        let mut camera = commands.entity(camera.0);
        match occlusion(settings.occlusion) {
            Some(ssao) => camera.insert(ssao),
            None => camera.remove::<ScreenSpaceAmbientOcclusion>(),
        };
        match settings.anti_aliasing {
            0 => camera.remove::<(Fxaa, Smaa)>(),
            1 => camera.remove::<Smaa>().insert(Fxaa::default()),
            _ => camera.remove::<Fxaa>().insert(Smaa::default()),
        };
    }
    // `camera::drive` focuses the lens again from the next frame.
    match (*camera, settings.depth_of_field_shown()) {
        ((camera, false), true) => {
            commands.entity(camera).insert(lens());
        }
        ((camera, true), false) => {
            commands.entity(camera).remove::<DepthOfField>();
        }
        _ => {}
    }
}

/// Keeps procedural cloud shading aligned with the graphics setting, including materials created
/// after the setting changed.
fn apply_cloud_shadows(settings: Res<Settings>, mut materials: ResMut<Assets<InkMaterial>>) {
    let cloud_shadows = if settings.shadows { 1.0 } else { 0.0 };
    for (_, material) in materials.iter_mut() {
        if material.extension.uniform.cloud_shadows != cloud_shadows {
            material.extension.uniform.cloud_shadows = cloud_shadows;
        }
    }
}

fn apply_ui_scale(
    mut due: Due,
    window: Single<&Window, With<PrimaryWindow>>,
    mut ui_scale: ResMut<UiScale>,
) {
    let Some(settings) = due.check(&window) else { return };
    let scale = fitted(settings.ui_scale as f32 / 100.0, window.size());
    if ui_scale.0 != scale {
        ui_scale.0 = scale;
    }
}

/// The room the largest page (the settings) takes at a UI scale of 1, in logical pixels.
const FIT: Vec2 = Vec2::new(1100.0, 760.0);

/// The UI as large as the player asked, as long as the pages fit the window (a small browser tab, a
/// tiled window).
fn fitted(asked: f32, window: Vec2) -> f32 {
    asked.min(window.x / FIT.x).min(window.y / FIT.y)
}

/// With the readable font on, every text set in IM Fell takes Atkinson Hyperlegible instead, and
/// back. Texts are written in IM Fell everywhere; this swaps them as they come and when the setting
/// changes.
fn swap_fonts(settings: Res<Settings>, assets: Res<AssetServer>, mut texts: Query<&mut TextFont>) {
    let (from, to) = if settings.readable_font { (FELL, READABLE) } else { (READABLE, FELL) };
    for mut font in &mut texts {
        if !settings.is_changed() && !font.is_added() {
            continue;
        }
        let bevy::text::FontSource::Handle(handle) = &font.font else { continue };
        let Some(path) = assets.get_path(handle.id()) else { continue };
        if let Some(k) = from.iter().position(|&f| path.path() == std::path::Path::new(f)) {
            font.font = assets.load(to[k]).into();
        }
    }
}

/// The window or the tab loses focus during play: the game waits under the pause page.
fn pause_unfocused(
    settings: Res<Settings>,
    window: Single<&Window, (With<PrimaryWindow>, Changed<Window>)>,
    screen: Res<State<Screen>>,
    mut next: ResMut<NextState<Screen>>,
) {
    if !window.focused && settings.pause_unfocused && *screen.get() == Screen::Playing {
        next.set(Screen::Paused);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The title's picture is smaller, never enlarged; the board's is the player's.
    #[test]
    fn the_title_draws_a_smaller_world() {
        let all = DynamicScale(100);
        assert_eq!(world_scale(100, all, Screen::Title), TITLE_SCALE);
        assert_eq!(world_scale(40, all, Screen::Title), 0.4);
        assert_eq!(world_scale(100, all, Screen::Playing), 1.0);
        assert_eq!(world_scale(70, all, Screen::Settings), 0.7);
        // The dynamic share shrinks the board's, never the title's.
        assert_eq!(world_scale(100, DynamicScale(90), Screen::Playing), 0.9);
        assert_eq!(world_scale(100, DynamicScale(90), Screen::Title), TITLE_SCALE);
    }

    #[test]
    fn the_ui_shrinks_only_to_fit() {
        assert_eq!(fitted(1.0, Vec2::new(1920.0, 1080.0)), 1.0);
        assert_eq!(fitted(1.5, Vec2::new(1920.0, 1080.0)), 1080.0 / 760.0);
        assert_eq!(fitted(1.0, Vec2::new(550.0, 1000.0)), 0.5);
    }
}
