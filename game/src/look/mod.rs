//! The look of the world: the camera, the light, the paper fog, and `InkMaterial`, which draws
//! every surface as an engraving (`assets/shaders/hachure.wesl`). Kenney models get their materials
//! swapped for ink when they spawn.

mod camera;
mod material;
mod models;
mod paper;
mod ready;
mod timing;

use bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin;
use bevy::prelude::*;

use crate::theme::{AMBIENT, SHEET};

#[cfg(not(target_arch = "wasm32"))]
pub use camera::occlusion;
pub use camera::{Frame, Sun, WorldCamera, lens};
pub use material::{Hachure, InkMaterial, fine, ink, shore_image, varnished, water, weighed};
pub use models::{Carved, Swaying, scene};
use models::{Inked, ink_models};
pub use paper::Paper;
pub use ready::PipelinesReady;
pub use timing::{frame_time, gpu_time};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        MaterialPlugin::<InkMaterial>::default(),
        FullscreenMaterialPlugin::<Paper>::default(),
        ready::plugin,
    ))
    .insert_resource(ClearColor(SHEET))
    .insert_resource(GlobalAmbientLight { color: AMBIENT, brightness: 900.0, ..default() })
    .init_resource::<Inked>()
    .add_systems(Startup, camera::setup)
    .add_observer(ink_models);
}
