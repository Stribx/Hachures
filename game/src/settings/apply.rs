//! Puts the settings into effect: the language, the sound, the window, the light, the lens and the UI scale.
//! Each system runs when the settings change, and when the window gains or loses focus (for the sound) or changes
//! size (for the scale).

use bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening;
use bevy::audio::AudioSinkPlayback;
use bevy::camera::{ImageRenderTarget, RenderTarget};
use bevy::ecs::system::SystemParam;
use bevy::post_process::dof::DepthOfField;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::window::{MonitorSelection, PresentMode, PrimaryWindow, WindowMode};

use super::{Music, Settings};
use crate::i18n;
#[cfg(not(target_arch = "wasm32"))]
use crate::look::occlusion;
use crate::look::{Frame, Sun, WorldCamera, lens};
use crate::schedule::Screen;
use crate::theme::{FELL, READABLE};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (apply_audio, apply_window, apply_render, apply_ui_scale, swap_fonts, pause_unfocused));
}

/// The settings, and when to read them again: they changed, or the window's focus or size did.
#[derive(SystemParam)]
struct Due<'w, 's> {
    settings: Res<'w, Settings>,
    seen: Local<'s, Option<(bool, Vec2, UVec2)>>,
}

impl Due<'_, '_> {
    fn check(&mut self, window: &Window) -> Option<&Settings> {
        let now = (window.focused, window.size(), window.physical_size());
        // Every frame, so the window's last state is always the one seen.
        let moved = self.seen.replace(now) != Some(now);
        (moved || self.settings.is_changed()).then(|| &*self.settings)
    }
}

/// The language and the music's volume, which follows the focus.
fn apply_audio(
    mut due: Due,
    window: Single<&Window, With<PrimaryWindow>>,
    mut music: Query<&mut AudioSink, With<Music>>,
) {
    let Some(s) = due.check(&window) else { return };
    i18n::set(&s.language);
    for mut sink in &mut music {
        sink.set_volume(s.music_volume(window.focused));
    }
}

fn apply_window(mut due: Due, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    let Some(s) = due.check(&window) else { return };
    let mode =
        if s.fullscreen { WindowMode::BorderlessFullscreen(MonitorSelection::Current) } else { WindowMode::Windowed };
    let present = if s.vsync { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync };
    if window.mode != mode {
        window.mode = mode;
    }
    if window.present_mode != present {
        window.present_mode = present;
    }
}

/// The board's picture and what is drawn into it: the render scale, the shadows, the occlusion, the anti-aliasing
/// and the lens.
fn apply_render(
    mut due: Due,
    window: Single<&Window, With<PrimaryWindow>>,
    mut suns: Query<&mut DirectionalLight, With<Sun>>,
    camera: Single<(Entity, Has<DepthOfField>), With<WorldCamera>>,
    frame: Res<Frame>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let Some(s) = due.check(&window) else { return };
    // The board's picture at the render scale. Its own scale factor shrinks with it, so it keeps the window's
    // logical size: picking and the place labels need no conversion.
    let scale = s.render_scale as f32 / 100.0;
    let wanted = (window.physical_size().as_vec2() * scale).as_uvec2().max(UVec2::ONE);
    // `get_mut` marks the image changed, so only when it does.
    if images.get(&frame.0).is_some_and(|image| image.size() != wanted) {
        let size = Extent3d { width: wanted.x, height: wanted.y, depth_or_array_layers: 1 };
        images.get_mut(&frame.0).expect("read just above").resize(size);
    }
    let target = ImageRenderTarget { handle: frame.0.clone(), scale_factor: window.scale_factor() * scale };
    commands.entity(camera.0).insert(RenderTarget::Image(target));
    // Stretched over the window, a smaller picture blurs: sharpened back, as a game's upscaler would.
    if s.render_scale < 100 {
        commands.entity(camera.0).insert(ContrastAdaptiveSharpening::default());
    } else {
        commands.entity(camera.0).remove::<ContrastAdaptiveSharpening>();
    }
    for mut sun in &mut suns {
        if sun.shadow_maps_enabled != s.shadows {
            sun.shadow_maps_enabled = s.shadows;
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use bevy::anti_alias::fxaa::Fxaa;
        use bevy::anti_alias::smaa::Smaa;
        use bevy::pbr::ScreenSpaceAmbientOcclusion;
        let mut camera = commands.entity(camera.0);
        match occlusion(s.occlusion) {
            Some(ssao) => camera.insert(ssao),
            None => camera.remove::<ScreenSpaceAmbientOcclusion>(),
        };
        match s.anti_aliasing {
            0 => camera.remove::<(Fxaa, Smaa)>(),
            1 => camera.remove::<Smaa>().insert(Fxaa::default()),
            _ => camera.remove::<Fxaa>().insert(Smaa::default()),
        };
    }
    // `camera::drive` focuses the lens again from the next frame.
    match (*camera, s.depth_of_field_shown()) {
        ((camera, false), true) => {
            commands.entity(camera).insert(lens());
        }
        ((camera, true), false) => {
            commands.entity(camera).remove::<DepthOfField>();
        }
        _ => {}
    }
}

fn apply_ui_scale(mut due: Due, window: Single<&Window, With<PrimaryWindow>>, mut ui_scale: ResMut<UiScale>) {
    let Some(s) = due.check(&window) else { return };
    let scale = fitted(s.ui_scale as f32 / 100.0, window.size());
    if ui_scale.0 != scale {
        ui_scale.0 = scale;
    }
}

/// The room the largest page (the settings) takes at a UI scale of 1, in logical pixels.
const FIT: Vec2 = Vec2::new(1100.0, 760.0);

/// The UI as large as the player asked, as long as the pages fit the window (a small browser tab, a tiled window).
fn fitted(asked: f32, window: Vec2) -> f32 {
    asked.min(window.x / FIT.x).min(window.y / FIT.y)
}

/// With the readable font on, every text set in IM Fell takes Atkinson Hyperlegible instead, and back. Texts are
/// written in IM Fell everywhere; this swaps them as they come and when the setting changes.
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

    #[test]
    fn the_ui_shrinks_only_to_fit() {
        assert_eq!(fitted(1.0, Vec2::new(1920.0, 1080.0)), 1.0);
        assert_eq!(fitted(1.5, Vec2::new(1920.0, 1080.0)), 1080.0 / 760.0);
        assert_eq!(fitted(1.0, Vec2::new(550.0, 1000.0)), 0.5);
    }
}
