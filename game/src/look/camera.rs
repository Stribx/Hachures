//! The board's camera and sun: the render-target frame, the lens, the fog and the ambient occlusion.

use bevy::camera::{ImageRenderTarget, RenderTarget};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::image::ImageSampler;
use bevy::light::{CascadeShadowConfigBuilder, ShadowFilteringMethod};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::post_process::dof::DepthOfField;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;

use super::Paper;
use crate::theme::{SHEET, SUNLIGHT};

/// The camera filming the board, as opposed to the icon studio's (`ui::studio`).
#[derive(Component)]
pub struct WorldCamera;

/// The board's sun, whose shadows the settings turn on and off.
#[derive(Component)]
pub struct Sun;

/// Filmed as a model on a table: sharp where the camera looks, soft before and beyond (`camera::drive` sets the focus
/// and the aperture). Gaussian blur, so it also runs on WebGL2. The settings can take it off.
pub fn lens() -> DepthOfField {
    DepthOfField { max_circle_of_confusion_diameter: 16.0, max_depth: 200.0, ..default() }
}

/// On the desktop, the light finds the creases: ambient occlusion at `level` (0 none, 1 low, 2 high), which
/// `hachure.wesl` turns into strokes where pieces meet the ground. Not on WebGL2. Measured with `HACHURES_FPS` at
/// 2880x1800 on an integrated GPU: Low 2.6 ms a frame, High 5.7 (its noise the strokes hide anyway), both with a
/// 1.2 ms depth prepass. No contact shadows: 5.3 ms, for the contacts the occlusion already marks.
#[cfg(not(target_arch = "wasm32"))]
pub fn occlusion(level: u8) -> Option<bevy::pbr::ScreenSpaceAmbientOcclusion> {
    use bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::{High, Low};
    let quality_level = match level {
        0 => return None,
        1 => Low,
        _ => High,
    };
    Some(bevy::pbr::ScreenSpaceAmbientOcclusion { quality_level, ..default() })
}

/// The board camera on the desktop: MSAA off, which ambient occlusion needs. The settings add the occlusion and
/// the anti-aliasing (`settings::apply`).
#[cfg(not(target_arch = "wasm32"))]
fn desktop() -> impl Bundle {
    Msaa::Off
}

#[cfg(target_arch = "wasm32")]
fn desktop() -> impl Bundle {}

/// The board's picture: the world camera draws into it at the render scale (`settings::apply` sizes it), and
/// the window's camera lays it under the interface, which stays sharp at the window's own resolution.
#[derive(Resource)]
pub struct Frame(pub Handle<Image>);

pub(super) fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut frame = Image::new_target_texture(1, 1, TextureFormat::Rgba8UnormSrgb, None);
    // Smooth when stretched over the window.
    frame.sampler = ImageSampler::linear();
    let frame = images.add(frame);
    commands.insert_resource(Frame(frame.clone()));
    // The window's camera: after the board's, it draws the picture, then the interface over it.
    commands.spawn((Camera2d, Camera { order: 1, ..default() }));
    commands.spawn((
        ImageNode::new(frame.clone()),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(i32::MIN),
        Pickable::IGNORE,
    ));
    commands.spawn((
        WorldCamera,
        Camera3d::default(),
        RenderTarget::Image(ImageRenderTarget { handle: frame, scale_factor: 1.0 }),
        // The shader picks the final colours itself; a tonemapping curve would grey them. Linear keeps them and,
        // unlike None, still dithers: the fog, the land's soft blends and the paper's corners would band in 8 bits.
        Tonemapping::Linear,
        DebandDither::Enabled,
        Paper::default(),
        DistanceFog {
            color: SHEET,
            falloff: FogFalloff::Linear { start: 45.0, end: 120.0 },
            ..default()
        },
        lens(),
        // Soft-edged shadows.
        ShadowFilteringMethod::Gaussian,
        desktop(),
    ));
    // The sun from the north-west, where the Empire lies. `hachure.wesl` draws the varnish glints from here too.
    commands.spawn((
        Sun,
        DirectionalLight {
            color: SUNLIGHT,
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 9.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder { num_cascades: 2, maximum_distance: 60.0, ..default() }.build(),
    ));
}
