//! The props of the decor Gratter scrapes off: the éboulis's stones, the palissade's stakes.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::grid::{Grid, Tile};
use super::props::{PropPart, merge};

/// Fallen stone: blocks of varied sizes, each turned and partly sunk in the ground, in a pale and a
/// dark tone.
pub(crate) fn rubble() -> Vec<PropPart> {
    // Size, turn about y, tilt about x, centre: below 0, so the block sits in the ground.
    let blocks = [
        (Vec3::new(0.3, 0.2, 0.24), 0.4, 0.1, Vec3::new(-0.1, -0.02, 0.05), true),
        (Vec3::new(0.22, 0.16, 0.28), 1.3, -0.2, Vec3::new(0.18, -0.03, -0.12), false),
        (Vec3::new(0.18, 0.14, 0.16), 2.2, 0.3, Vec3::new(0.05, -0.02, 0.22), true),
        (Vec3::new(0.26, 0.18, 0.14), -0.6, -0.1, Vec3::new(-0.2, -0.04, -0.2), false),
        (Vec3::new(0.14, 0.12, 0.2), 0.9, 0.25, Vec3::new(0.2, -0.02, 0.15), true),
        (Vec3::new(0.12, 0.1, 0.12), 1.8, 0.4, Vec3::new(-0.02, 0.0, -0.05), false),
    ];
    let (mut pale, mut dark) = (Vec::new(), Vec::new());
    for (size, turn, tilt, at, light) in blocks {
        let block = Mesh::from(Cuboid::new(size.x, size.y, size.z))
            .rotated_by(Quat::from_rotation_y(turn) * Quat::from_rotation_x(tilt))
            .translated_by(at);
        if light { &mut pale } else { &mut dark }.push(block);
    }
    vec![
        PropPart {
            mesh: merge(pale).expect("a prop has parts"),
            color: Color::srgb(0.62, 0.6, 0.55),
        },
        PropPart {
            mesh: merge(dark).expect("a prop has parts"),
            color: Color::srgb(0.5, 0.48, 0.44),
        },
    ]
}

/// 1 when a palissade stands north or south of cell `i` and none east or west, so the wall runs
/// along z; else 0.
pub(crate) fn palisade_shape(grid: &Grid, i: usize) -> u8 {
    let (x, _) = grid.coords(i);
    let (mut along_z, mut along_x) = (false, false);
    for n in grid.neighbours(i).filter(|&n| grid.tiles[n] == Tile::Palissade) {
        if grid.coords(n).0 == x { along_z = true } else { along_x = true }
    }
    (along_z && !along_x) as u8
}

/// A palisade: a row of pointed stakes in dark stained wood, bound by two rails, the tips dipped in
/// the Chancellerie's vermilion. It runs along x, or along z when `shape` is 1.
pub(crate) fn palisade(shape: u8) -> Vec<PropPart> {
    let turn = Quat::from_rotation_y(if shape == 1 { FRAC_PI_2 } else { 0.0 });
    let (mut wood, mut tips) = (Vec::new(), Vec::new());
    for k in 0..5 {
        let x = -0.4 + 0.2 * k as f32;
        let height = 0.5 + 0.025 * ((k * 3) % 5) as f32;
        wood.push(
            Mesh::from(Cylinder { radius: 0.05, half_height: height / 2.0 })
                .translated_by(Vec3::new(x, height / 2.0, 0.0)),
        );
        tips.push(Mesh::from(Cone { radius: 0.05, height: 0.1 }).translated_by(Vec3::new(
            x,
            height + 0.05,
            0.0,
        )));
    }
    // The rails run edge to edge, so a line of palissades is one fence.
    for y in [0.15, 0.4] {
        let rail = Mesh::from(Cylinder { radius: 0.03, half_height: 0.5 })
            .rotated_by(Quat::from_rotation_z(FRAC_PI_2));
        wood.push(rail.translated_by(Vec3::new(0.0, y, 0.0)));
    }
    let paint = |parts: Vec<Mesh>, color| {
        let mesh = merge(parts.into_iter().map(|m| m.rotated_by(turn))).expect("a prop has parts");
        PropPart { mesh, color }
    };
    vec![paint(wood, Color::srgb(0.36, 0.26, 0.2)), paint(tips, Color::srgb(0.7, 0.2, 0.15))]
}
