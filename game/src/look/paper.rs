//! The paper pass: a still grain and a sepia vignette laid over the board's picture (`assets/shaders/paper.wesl`).

use bevy::core_pipeline::Core3dSystems;
use bevy::core_pipeline::fullscreen_material::FullscreenMaterial;
use bevy::core_pipeline::tonemapping::tonemapping;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::BoxedSystem;
use bevy::post_process::bloom::bloom;
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;
use bevy::shader::ShaderRef;

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
