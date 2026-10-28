//! The look of the world: the camera, the light, the paper fog, and `InkMaterial`, which draws every surface as an
//! engraving (`assets/shaders/hachure.wesl`). Kenney models get their materials swapped for ink when they spawn.

mod camera;
mod material;
mod models;
mod paper;

use bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin;
use bevy::prelude::*;

use crate::theme::{AMBIENT, SHEET};

#[cfg(not(target_arch = "wasm32"))]
pub use camera::occlusion;
pub use camera::{Frame, Sun, WorldCamera, lens};
pub use material::{Hachure, InkMaterial, ink, shore_image, varnished, water};
pub use models::{Carved, Swaying};
use models::{Inked, ink_models};
pub use paper::Paper;

pub fn plugin(app: &mut App) {
    app.add_plugins((MaterialPlugin::<InkMaterial>::default(), FullscreenMaterialPlugin::<Paper>::default()))
        .insert_resource(ClearColor(SHEET))
        .insert_resource(GlobalAmbientLight { color: AMBIENT, brightness: 900.0, ..default() })
        .init_resource::<Inked>()
        .add_systems(Startup, camera::setup)
        .add_observer(ink_models);
}
