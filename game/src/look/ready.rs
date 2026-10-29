//! Whether the GPU has compiled every pipeline asked for so far, read back from the render world:
//! the splash waits on it, so no shader is first compiled in a menu or in play.

use bevy::prelude::*;
use bevy::render::render_resource::PipelineCache;
use bevy::render::{ExtractSchedule, MainWorld, RenderApp};

/// No pipeline waiting to be compiled, as of the last frame drawn.
#[derive(Resource, Default, PartialEq)]
pub struct PipelinesReady(pub bool);

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PipelinesReady>();
    // Without a renderer (the tests), nothing is ever compiled.
    if let Some(render) = app.get_sub_app_mut(RenderApp) {
        render.add_systems(ExtractSchedule, read_back);
    }
}

fn read_back(mut main: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
    let ready = pipelines.waiting_pipelines().next().is_none();
    if let Some(mut seen) = main.get_resource_mut::<PipelinesReady>() {
        seen.set_if_neq(PipelinesReady(ready));
    }
}
