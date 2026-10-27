//! Where the woods, lone trees and rocks stand on the Marches, and which model each is.

use bevy::prelude::*;

use super::SHEETS;
use super::terrain::{EAST, NORTH, SOUTH, WATER, WEST, hash, height, road_distance, snow, wood};

/// Props kept off: closer than this to a sheet or the road.
pub(super) const CLEAR: f32 = 1.5;

/// Where the woods, lone trees and rocks stand, and which model each is.
pub(super) fn props() -> Vec<(&'static str, Transform)> {
    let mut props = Vec::new();
    let mut z = NORTH;
    while z < SOUTH {
        let mut x = WEST;
        while x < EAST {
            let cell = Vec2::new(x, z);
            let p = cell + (Vec2::new(hash(cell), hash(cell + 3.1)) - 0.5) * 1.6;
            let h = height(p);
            let roll = hash(p * 1.7);
            let wood = wood(p) > 0.52;
            let clear = SHEETS.iter().all(|s| s.distance(p) > CLEAR) && road_distance(p) > CLEAR;
            let steep = (height(p + Vec2::X) - h).abs().max((height(p + Vec2::Y) - h).abs());
            // On the snow the land is painted with.
            let snow = snow(p, h) > 0.35;
            let model = match () {
                _ if !clear || h < WATER + 0.2 => None,
                _ if steep > 0.6 && roll < 0.25 => Some(if snow { "snow-detail-rocks" } else { "detail-rocks-large" }),
                _ if (wood && roll < 0.8) || roll < 0.06 => Some(match (snow, roll < 0.35) {
                    (true, true) => "snow-detail-tree-large",
                    (true, false) => "snow-detail-tree",
                    (false, true) => "detail-tree-large",
                    (false, false) => "detail-tree",
                }),
                _ if roll > 0.985 => Some("detail-rocks"),
                _ => None,
            };
            if let Some(model) = model {
                let transform = Transform::from_xyz(p.x, h - 0.05, p.y)
                    .with_rotation(Quat::from_rotation_y(roll * 40.0))
                    .with_scale(Vec3::splat(2.0 + hash(p + 9.0) * 1.4));
                props.push((model, transform));
            }
            x += 2.0;
        }
        z += 2.0;
    }
    props
}
