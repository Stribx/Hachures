//! World 3's gallery blackout: the board dims for two waves while the Calque stays legible.

use bevy::prelude::*;

use crate::ink::Selection;
use crate::level::Level;
use crate::look::InkMaterial;

const NORMAL: f32 = 900.0;
const DARK: f32 = 140.0;

pub(super) fn apply(
    level: Res<Level>,
    selection: Res<Selection>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    let target = if level.darkness_on() && !selection.calque { DARK } else { NORMAL };
    if (ambient.brightness - target).abs() > f32::EPSILON {
        ambient.brightness = target;
    }
    let ink = if level.darkness_on() && !selection.calque { 0.35 } else { 1.0 };
    for (_, material) in materials.iter_mut() {
        material.extension.uniform.cloud_shadows = ink;
    }
}
