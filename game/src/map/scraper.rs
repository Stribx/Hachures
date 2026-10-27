//! The Gratter's piece for the tool bar: a grattoir, a scraping knife lying on the ground.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

/// A wooden handle and a flat pale blade, lying diagonally on the block's top at the origin.
pub(crate) fn scraper() -> Vec<(Mesh, Color)> {
    let spin = Quat::from_rotation_y(0.6);
    let handle = Mesh::from(Cylinder { radius: 0.05, half_height: 0.2 })
        .rotated_by(Quat::from_rotation_z(FRAC_PI_2))
        .translated_by(Vec3::new(-0.2, 0.05, 0.0));
    let blade = Mesh::from(Cuboid::new(0.4, 0.02, 0.14)).translated_by(Vec3::new(0.2, 0.03, 0.0));
    vec![
        (handle.rotated_by(spin), Color::srgb(0.6, 0.44, 0.27)),
        (blade.rotated_by(spin), Color::srgb(0.88, 0.82, 0.68)),
    ]
}
