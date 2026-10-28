//! The bocage's hedgerows along the parcels' seams, and the wheat standing in its wheat parcels,
//! swaying.

use bevy::prelude::*;

use super::parcels::{WHEAT_WASH, openness, parcel, seam};
use super::props::CLEAR;
use super::terrain::{EAST, NORTH, SOUTH, WEST, hash, height, road_distance};
use super::{OverworldRoot, SHEETS};
use crate::look::{Hachure, InkMaterial, ink};
use crate::theme::{HEDGE, WHEAT};

/// How far a hedge's top and a wheat ear lean in the wind (`Hachure::wind`), in world units.
const HEDGE_WIND: f32 = 0.06;
const WHEAT_WIND: f32 = 0.16;
/// The grid the bocage is grown on, in world units: fine enough that the hedges along a seam touch.
const STEP: f32 = 0.6;
/// The land is grown in strips this wide, so the camera skips those out of view.
const STRIP: f32 = 40.0;

/// What grows on the Marches: a hedge lump, or a clump of wheat, standing at a point.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Growth {
    Hedge(Vec2),
    Wheat(Vec2),
}

/// Every hedge lump along the seams and every clump of wheat in the wheat parcels, on open land
/// only, clear of the sheets and the road.
pub(super) fn growth() -> Vec<Growth> {
    let mut growth = Vec::new();
    let mut z = NORTH;
    while z < SOUTH {
        let mut x = WEST;
        while x < EAST {
            let cell = Vec2::new(x, z);
            let p = cell + (Vec2::new(hash(cell + 2.7), hash(cell + 5.9)) - 0.5) * 0.3;
            let clear = SHEETS.iter().all(|s| s.distance(p) > CLEAR) && road_distance(p) > CLEAR;
            if clear && openness(p) > 0.6 {
                if seam(p) < 0.3 {
                    growth.push(Growth::Hedge(p));
                } else if parcel(p).wash == WHEAT_WASH && seam(p) > 0.8 && hash(p * 3.3) < 0.25 {
                    growth.push(Growth::Wheat(p));
                }
            }
            x += STEP;
        }
        z += STEP;
    }
    growth
}

/// The bocage's hedges and wheat, a few strips of merged meshes, swaying.
pub(super) fn spawn_bocage(
    root: Single<Entity, With<OverworldRoot>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    let material = |wind: f32, materials: &mut Assets<InkMaterial>| {
        let mut felt = ink(Color::WHITE);
        felt.extension = Hachure::fine();
        felt.extension.uniform.wind = wind;
        materials.add(felt)
    };
    let (hedge, wheat) =
        (material(HEDGE_WIND, &mut materials), material(WHEAT_WIND, &mut materials));
    let strips = ((EAST - WEST) / STRIP).ceil() as usize;
    let mut parts: Vec<(Vec<Mesh>, Vec<Mesh>)> = (0..strips).map(|_| Default::default()).collect();
    for g in growth() {
        let (Growth::Hedge(p) | Growth::Wheat(p)) = g;
        let strip = &mut parts[((p.x - WEST) / STRIP) as usize];
        let foot = Vec3::new(p.x, height(p) - 0.05, p.y);
        match g {
            Growth::Hedge(_) => {
                strip.0.extend(hedge_lumps(p).into_iter().map(|m| m.translated_by(foot)))
            }
            Growth::Wheat(_) => {
                strip.1.extend(wheat_stalks(p).into_iter().map(|m| m.translated_by(foot)))
            }
        }
    }
    for (hedges, stalks) in parts {
        for (pieces, material) in [(hedges, &hedge), (stalks, &wheat)] {
            let Some(mesh) = pieces.into_iter().reduce(|mut all, m| {
                all.merge(&m).expect("the bocage's pieces share their attributes");
                all
            }) else {
                continue;
            };
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                ChildOf(*root),
            ));
        }
    }
}

/// A hedge lump: two rounded masses of foliage, a little different each, their tops taking the
/// wind.
fn hedge_lumps(p: Vec2) -> Vec<Mesh> {
    let roll = hash(p * 7.1);
    let green = HEDGE.mix(&Color::srgb(0.42, 0.56, 0.3), roll);
    [(0.0, 0.42), (0.35 * roll + 0.15, 0.32)]
        .into_iter()
        .map(|(shift, r)| {
            let mesh = Sphere::new(r).mesh().ico(1).expect("a small ico sphere");
            let mesh = mesh.scaled_by(Vec3::new(1.0, 0.85, 1.0)).translated_by(Vec3::new(
                shift,
                r * 0.8,
                -shift * 0.6,
            ));
            weighed(mesh, green, 0.75)
        })
        .collect()
}

/// A clump of wheat: a few tall stalks, each with its ear.
fn wheat_stalks(p: Vec2) -> Vec<Mesh> {
    let ear = WHEAT.mix(&Color::srgb(0.7, 0.55, 0.28), 0.4);
    (0..4)
        .flat_map(|k| {
            let at = Vec3::new(hash(p + k as f32) - 0.5, 0.0, hash(p - k as f32) - 0.5) * 0.6;
            let h = 0.45 + hash(p * 2.0 + k as f32) * 0.15;
            let stalk =
                Mesh::from(Cuboid::new(0.05, h, 0.05)).translated_by(at + Vec3::Y * h / 2.0);
            let head = Mesh::from(Cuboid::new(0.09, 0.16, 0.09)).translated_by(at + Vec3::Y * h);
            [weighed(stalk, WHEAT, 0.7), weighed(head, ear, 0.7)]
        })
        .collect()
}

/// `mesh` coloured, its vertices' alpha the share of the wind they take: 0 at the ground, 1 at
/// `tallest`.
fn weighed(mesh: Mesh, color: Color, tallest: f32) -> Mesh {
    let [r, g, b, _] = color.to_linear().to_f32_array();
    let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|p| p.as_float3())
    else {
        return mesh;
    };
    let colors: Vec<[f32; 4]> =
        positions.iter().map(|p| [r, g, b, (p[1] / tallest).clamp(0.0, 1.0)]).collect();
    mesh.with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overworld::terrain::WATER;

    /// Hedges and wheat stand on dry land, clear of every sheet and of the road, and there are
    /// plenty of both.
    #[test]
    fn the_bocage_keeps_off_the_sheets_and_the_road() {
        let growth = growth();
        let (hedges, wheat) =
            (growth.iter().filter(|g| matches!(g, Growth::Hedge(_))).count(), growth.len());
        assert!(hedges > 300 && wheat > hedges, "{hedges} hedges of {wheat}");
        for g in growth {
            let (Growth::Hedge(p) | Growth::Wheat(p)) = g;
            assert!(SHEETS.iter().all(|s| s.distance(p) > CLEAR), "on a sheet at {p}");
            assert!(road_distance(p) > CLEAR, "on the road at {p}");
            assert!(height(p) > WATER, "in the water at {p}");
        }
    }
}
