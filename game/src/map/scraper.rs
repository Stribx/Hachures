//! The Gratter's piece for the tool bar: a grattoir, a scraping knife lying on the ground.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::props::{PROP_PALE, PROP_WOOD, PropPart};

/// A wooden handle and a flat pale blade, lying diagonally on the block's top at the origin.
pub(crate) fn scraper() -> Vec<PropPart> {
    let spin = Quat::from_rotation_y(0.6);
    let handle = Mesh::from(Cylinder { radius: 0.05, half_height: 0.2 })
        .rotated_by(Quat::from_rotation_z(FRAC_PI_2))
        .translated_by(Vec3::new(-0.2, 0.05, 0.0));
    let blade = Mesh::from(Cuboid::new(0.4, 0.02, 0.14)).translated_by(Vec3::new(0.2, 0.03, 0.0));
    vec![
        PropPart { mesh: handle.rotated_by(spin), color: PROP_WOOD },
        PropPart { mesh: blade.rotated_by(spin), color: PROP_PALE },
    ]
}
