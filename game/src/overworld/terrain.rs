//! The land itself: procedural height, colour and wood, and the meshes of the land and of the road across it.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use super::parcels::{WASHES, openness, parcel};
use super::{ROAD, SHEETS};
use crate::theme::{HEDGE, PASTURE, WHEAT};

/// Where the land is drawn.
pub(super) const WEST: f32 = -40.0;
pub(super) const EAST: f32 = 240.0;
pub(super) const NORTH: f32 = -75.0;
pub(super) const SOUTH: f32 = 40.0;
/// Below this, the land is under water.
pub const WATER: f32 = -0.35;

pub(super) fn hash(p: Vec2) -> f32 {
    ((p.dot(Vec2::new(127.1, 311.7))).sin() * 43758.547).fract().abs()
}

fn value_noise(p: Vec2) -> f32 {
    let (i, f) = (p.floor(), p.fract_gl());
    let u = f * f * (3.0 - 2.0 * f);
    let corner = |dx, dy| hash(i + Vec2::new(dx, dy));
    let top = corner(0.0, 0.0) + (corner(1.0, 0.0) - corner(0.0, 0.0)) * u.x;
    let bottom = corner(0.0, 1.0) + (corner(1.0, 1.0) - corner(0.0, 1.0)) * u.x;
    top + (bottom - top) * u.y
}

/// Four octaves of value noise, from 0 to 1.
fn fbm(p: Vec2) -> f32 {
    (0..4).map(|o| value_noise(p * 2f32.powi(o) + o as f32 * 17.0) / 2f32.powi(o + 1)).sum::<f32>() / 0.9375
}

/// How far into world 2's mountains `x` lies, from 0 to 1.
pub(super) fn mountain_share(x: f32) -> f32 {
    ((x - 112.0) / 16.0).clamp(0.0, 1.0)
}

/// Distance from `p` to the road.
pub(super) fn road_distance(p: Vec2) -> f32 {
    ROAD.iter()
        .flat_map(|leg| leg.windows(2))
        .map(|s| {
            let (a, b) = (s[0], s[1]);
            let t = ((p - a).dot(b - a) / (b - a).length_squared()).clamp(0.0, 1.0);
            p.distance(a + (b - a) * t)
        })
        .fold(f32::MAX, f32::min)
}

pub(super) fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Height of the land at `p`: rolling hills in world 1, mountains in world 2, flat around every sheet.
pub fn height(p: Vec2) -> f32 {
    let amplitude = 4.0 + 7.0 * mountain_share(p.x);
    let mut h = (fbm(p * 0.05) - 0.34) * amplitude * 1.6;
    let sheet = SHEETS.iter().map(|s| s.distance(p)).fold(f32::MAX, f32::min);
    h = h * smoothstep(0.0, 1.0, (sheet - 0.5) / 6.0) - 0.05 * (1.0 - smoothstep(0.0, 6.0, sheet));
    // The road follows the land, and fords the lakes on a causeway.
    let road = 1.0 - smoothstep(1.0, 3.0, road_distance(p));
    h + (h.max(WATER + 0.3) - h) * road
}

/// How wooded the land is at `p`: the woods stand where it is over 0.52, and their floor darkens the ground.
pub(super) fn wood(p: Vec2) -> f32 {
    fbm(p * 0.035 + 40.0)
}

/// How snowy the heights at `p`, `h` high, are, from 0 to 1.
pub(super) fn snow(p: Vec2, h: f32) -> f32 {
    mountain_share(p.x) * smoothstep(2.0, 4.0, h)
}

/// Colour of the land at `p`, `h` high, painted like the plaster of a plan-relief: the bocage's parcels on the open
/// land, broad swathes of meadow and straw elsewhere, the woods' darker floor, sand by the water, stone and snow up
/// high. Every change is a blend, but for the parcels' seams.
fn land_color(p: Vec2, h: f32) -> Color {
    let swathes = PASTURE.mix(&WHEAT, smoothstep(0.35, 0.65, fbm(p * 0.04 + 7.0)));
    let meadow = swathes.mix(&WASHES[parcel(p).wash], openness(p));
    let mut color = meadow.mix(&HEDGE, smoothstep(0.47, 0.55, wood(p)));
    color = color.mix(&Color::srgb(0.62, 0.66, 0.7), mountain_share(p.x) * 0.8);
    color = color.mix(&Color::srgb(0.86, 0.78, 0.58), 1.0 - smoothstep(WATER + 0.05, WATER + 0.25, h));
    let stone = Color::srgb(0.72, 0.68, 0.6).mix(&Color::srgb(0.93, 0.93, 0.92), mountain_share(p.x));
    color.mix(&stone, smoothstep(2.0, 4.0, h))
}

/// The land as one smooth mesh, coloured at its corners so colours blend across it.
pub(super) fn land_mesh() -> Mesh {
    const STEP: f32 = 0.5;
    let (columns, rows) = (((EAST - WEST) / STEP) as usize, ((SOUTH - NORTH) / STEP) as usize);
    let corners: Vec<Vec3> = (0..=rows)
        .flat_map(|r| (0..=columns).map(move |c| Vec2::new(WEST + c as f32 * STEP, NORTH + r as f32 * STEP)))
        .map(|p| Vec3::new(p.x, height(p), p.y))
        .collect();
    let colors: Vec<[f32; 4]> = corners.iter().map(|c| land_color(c.xz(), c.y).to_linear().to_f32_array()).collect();
    let width = columns as u32 + 1;
    let indices: Vec<u32> = (0..rows as u32)
        .flat_map(|r| {
            (0..columns as u32).flat_map(move |c| {
                let (a, d) = (r * width + c, (r + 1) * width + c);
                [a, d, a + 1, a + 1, d, d + 1]
            })
        })
        .collect();
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, corners)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(Indices::U32(indices))
        .with_computed_smooth_normals()
}

/// The road as a ribbon of beaten earth just above the land.
pub(super) fn road_mesh() -> Mesh {
    const HALF_WIDTH: f32 = 0.8;
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    for leg in ROAD.iter() {
        for (i, w) in leg.windows(2).enumerate() {
            let (a, b) = (w[0], w[1]);
            let side = (b - a).normalize().perp() * HALF_WIDTH;
            // Over the highest of the land under it, so no facet pokes through.
            let lie = |p: Vec2| Vec3::new(p.x, height(p).max(height(p - side)).max(height(p + side)) + 0.06, p.y);
            if i == 0 {
                positions.extend([lie(a - side), lie(a + side)]);
            }
            positions.extend([lie(b - side), lie(b + side)]);
            let n = positions.len() as u32;
            indices.extend([n - 4, n - 3, n - 2, n - 3, n - 1, n - 2]);
        }
    }
    let count = positions.len();
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::Y; count])
        .with_inserted_indices(Indices::U32(indices))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The land's paint blends everywhere but at the parcels' seams: a cut would stay as big however close the points,
    /// a blend shrinks.
    #[test]
    fn land_colour_cuts_only_at_seams() {
        for z in [-60.0, -20.0, 0.0, 10.0, 30.0] {
            let mut x = WEST;
            let at = |x: f32| {
                let p = Vec2::new(x, z);
                land_color(p, height(p)).to_srgba().to_f32_array()
            };
            let mut before = at(x);
            while x < EAST {
                x += 0.01;
                let now = at(x);
                let jump = before.iter().zip(now).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
                let seam = parcel(Vec2::new(x - 0.01, z)).id != parcel(Vec2::new(x, z)).id;
                assert!(jump < 0.03 || seam, "a cut of {jump} in the land's colour at ({x}, {z})");
                before = now;
            }
        }
    }
}
