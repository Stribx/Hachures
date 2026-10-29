//! The performance overlay: the FPS, or in detail the frame time, the GPU's time and its name, on
//! a slip under the level's label. Off, FPS or detailed in the settings; F3 goes round them.

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::ui::ComputedNode;

use super::hud::HudLabel;
use super::widgets::text;
use crate::i18n::{number, tf};
use crate::look::gpu_time;
use crate::settings::Settings;
use crate::theme::{PAPER, SMALL_SIZE};

/// What the overlay shows, from `Settings::performance`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overlay {
    Off,
    Fps,
    Detailed,
}

impl Overlay {
    fn of(level: u8) -> Overlay {
        match level {
            0 => Overlay::Off,
            1 => Overlay::Fps,
            _ => Overlay::Detailed,
        }
    }
}

/// The level after `level`, round: what F3 does.
fn next(level: u8) -> u8 {
    (level.min(2) + 1) % 3
}

/// The numbers the overlay shows: frames a second, the frame time's mean and worst (ms) over the
/// last frames kept, the GPU's time a frame and its name, when the platform tells them.
struct Reading {
    fps: f64,
    mean_ms: f64,
    worst_ms: f64,
    gpu_ms: Option<f64>,
    adapter: Option<String>,
}

/// The slip the overlay is written on.
#[derive(Component)]
struct PerfSlip;

/// How often the numbers are written again, in seconds: often enough to follow, slow enough to
/// read.
const REFRESH: f32 = 0.25;
/// Where the slip stands from the window's corner, and its gap under the label.
const MARGIN: f32 = 16.0;
const GAP: f32 = 8.0;

pub(super) fn plugin(app: &mut App) {
    // Bevy's `trace` (the `profile` feature) adds the render timings itself.
    if !app.is_plugin_added::<RenderDiagnosticsPlugin>() {
        app.add_plugins(RenderDiagnosticsPlugin);
    }
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, spawn)
        .add_systems(Update, (f3, show).chain());
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        PerfSlip,
        text(&assets, "", SMALL_SIZE),
        Node {
            position_type: PositionType::Absolute,
            top: px(MARGIN),
            left: px(MARGIN),
            padding: UiRect::axes(px(10), px(6)),
            display: Display::None,
            ..default()
        },
        BackgroundColor(PAPER),
        // Over everything, the travel clouds included.
        GlobalZIndex(1000),
    ));
}

/// F3: the next level, kept in the settings file.
fn f3(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>) {
    if keys.just_pressed(KeyCode::F3) {
        settings.performance = next(settings.performance);
        settings.store();
    }
}

fn read(store: &DiagnosticsStore, adapter: Option<&RenderAdapterInfo>) -> Reading {
    let fps = store.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed());
    let times: Vec<f64> = store
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .map(|d| d.values().copied().collect())
        .unwrap_or_default();
    let mean_ms = times.iter().sum::<f64>() / times.len().max(1) as f64;
    Reading {
        fps: fps.unwrap_or(0.0),
        mean_ms,
        worst_ms: times.iter().copied().fold(0.0, f64::max),
        gpu_ms: gpu_time(store),
        adapter: adapter.map(|a| format!("{} ({:?})", a.name, a.backend)),
    }
}

/// The overlay's lines at `overlay`. The GPU's name is the driver's, not translated.
fn lines(overlay: Overlay, reading: &Reading) -> Vec<String> {
    let ms = |x: f64| number(x as f32);
    let mut lines = Vec::new();
    if overlay == Overlay::Off {
        return lines;
    }
    lines.push(tf("perf.fps", &[&(reading.fps.round() as u32)]));
    if overlay == Overlay::Detailed {
        lines.push(tf("perf.frame", &[&ms(reading.mean_ms), &ms(reading.worst_ms)]));
        if let Some(gpu) = reading.gpu_ms {
            lines.push(tf("perf.gpu", &[&ms(gpu)]));
        }
        lines.extend(reading.adapter.clone());
    }
    lines
}

/// Writes the slip a few times a second (at once when the level changes), under the label while it
/// shows, in the corner otherwise.
fn show(
    time: Res<Time<Real>>,
    mut since: Local<f32>,
    settings: Res<Settings>,
    store: Res<DiagnosticsStore>,
    adapter: Option<Res<RenderAdapterInfo>>,
    label: Query<(&ComputedNode, &InheritedVisibility), With<HudLabel>>,
    slip: Single<(&mut Text, &mut Node), With<PerfSlip>>,
) {
    *since += time.delta_secs();
    if *since < REFRESH && !settings.is_changed() {
        return;
    }
    *since = 0.0;
    let (mut words, mut node) = slip.into_inner();
    let lines = lines(Overlay::of(settings.performance), &read(&store, adapter.as_deref()));
    let display = if lines.is_empty() { Display::None } else { Display::Flex };
    let below = label.iter().find(|(_, shown)| shown.get());
    let top = below.map_or(MARGIN, |(l, _)| MARGIN + l.size().y * l.inverse_scale_factor() + GAP);
    let new = lines.join("\n");
    if words.0 != new {
        words.0 = new;
    }
    if node.display != display || node.top != px(top) {
        (node.display, node.top) = (display, px(top));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{number, tf};

    fn reading(gpu_ms: Option<f64>) -> Reading {
        Reading {
            fps: 59.6,
            mean_ms: 16.7,
            worst_ms: 21.04,
            gpu_ms,
            adapter: Some("Intel Arc (Vulkan)".into()),
        }
    }

    #[test]
    fn each_level_says_more() {
        let full = reading(Some(7.25));
        assert!(lines(Overlay::Off, &full).is_empty());
        assert_eq!(lines(Overlay::Fps, &full), [tf("perf.fps", &[&60])]);
        let detailed = lines(Overlay::Detailed, &full);
        assert_eq!(detailed.len(), 4, "{detailed:?}");
        assert_eq!(detailed[1], tf("perf.frame", &[&number(16.7), &number(21.0)]));
        assert_eq!(detailed[2], tf("perf.gpu", &[&number(7.25)]));
        assert_eq!(detailed[3], "Intel Arc (Vulkan)");
    }

    #[test]
    fn without_gpu_timings_the_line_is_left_out() {
        let detailed = lines(Overlay::Detailed, &reading(None));
        assert_eq!(detailed.len(), 3, "{detailed:?}");
        assert_eq!(detailed[2], "Intel Arc (Vulkan)");
    }

    #[test]
    fn f3_goes_off_fps_detailed_and_round() {
        assert_eq!((next(0), next(1), next(2)), (1, 2, 0));
        assert_eq!(Overlay::of(1), Overlay::Fps);
        assert_eq!(Overlay::of(7), Overlay::Detailed, "an odd file reads as the most");
    }
}
