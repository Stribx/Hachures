//! The look of the world: the camera, the light, the paper fog, and `InkMaterial`, which draws every surface as an
//! engraving (`assets/shaders/hachure.wesl`). Kenney models get their materials swapped for ink when they spawn.

mod material;
mod models;

use bevy::camera::{ImageRenderTarget, RenderTarget};
use bevy::core_pipeline::Core3dSystems;
use bevy::core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping, tonemapping};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::BoxedSystem;
use bevy::image::ImageSampler;
use bevy::light::{CascadeShadowConfigBuilder, ShadowFilteringMethod};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::post_process::bloom::bloom;
use bevy::post_process::dof::DepthOfField;
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{ShaderType, TextureFormat};
use bevy::shader::ShaderRef;

use crate::theme::{AMBIENT, SHEET, SUNLIGHT};

pub use material::{Hachure, InkMaterial, ink, shore_image, varnished, water};
pub use models::{Carved, Swaying};
use models::{Inked, ink_models};

pub fn plugin(app: &mut App) {
    app.add_plugins((MaterialPlugin::<InkMaterial>::default(), FullscreenMaterialPlugin::<Paper>::default()))
        .insert_resource(ClearColor(SHEET))
        .insert_resource(GlobalAmbientLight { color: AMBIENT, brightness: 900.0, ..default() })
        .init_resource::<Inked>()
        .add_systems(Startup, setup)
        .add_observer(ink_models);
}

/// The board filmed as a photograph of a model on the table (`assets/shaders/paper.wesl`): a still paper grain,
/// and the corners sinking into sepia. On the board's camera only; the interface is drawn after it, untouched.
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
#[extract_app(RenderApp)]
pub struct Paper {
    /// How far the corners sink toward sepia, 0 for none.
    pub vignette: f32,
    /// How strong the paper's grain is, 0 for none.
    pub grain: f32,
    _padding: Vec2,
}

impl Default for Paper {
    fn default() -> Self {
        Paper { vignette: 0.55, grain: 0.05, _padding: Vec2::ZERO }
    }
}

impl FullscreenMaterial for Paper {
    fn fragment_shader() -> ShaderRef {
        "shaders/paper.wesl".into()
    }

    // Before the lens (which runs after bloom): both swap the frame's two textures, and left unordered one read
    // the other's last frame, so the grain piled up frame after frame. Before the tonemapping, so it is dithered.
    fn schedule_configs(system: ScheduleConfigs<BoxedSystem>) -> ScheduleConfigs<BoxedSystem> {
        system.in_set(Core3dSystems::PostProcess).before(bloom).before(tonemapping)
    }
}

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

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut frame = Image::new_target_texture(1, 1, TextureFormat::Rgba8UnormSrgb, None);
    // Smooth when stretched over the window.
    frame.sampler = ImageSampler::linear();
    let frame = images.add(frame);
    commands.insert_resource(Frame(frame.clone()));
    // The window's camera: after the board's, it draws the picture, then the interface over it.
    commands.spawn((Camera2d, Camera { order: 1, ..default() }));
    commands.spawn((
        ImageNode::new(frame.clone()),
        Node { position_type: PositionType::Absolute, width: percent(100), height: percent(100), ..default() },
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
        DistanceFog { color: SHEET, falloff: FogFalloff::Linear { start: 45.0, end: 120.0 }, ..default() },
        lens(),
        // Soft-edged shadows.
        ShadowFilteringMethod::Gaussian,
        desktop(),
    ));
    // The sun from the north-west, where the Empire lies. `hachure.wesl` draws the varnish glints from here too.
    commands.spawn((
        Sun,
        DirectionalLight { color: SUNLIGHT, illuminance: 10_000.0, shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(-4.0, 9.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder { num_cascades: 2, maximum_distance: 60.0, ..default() }.build(),
    ));
}
