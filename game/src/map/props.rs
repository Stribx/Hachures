//! The props that stand on a tool's cell: mud and brambles, the meshes made once, and setting them
//! down.

use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use super::barricade::barricade;
use super::decor::{palisade, rubble};
use super::grid::Tile;
use super::land::{Land, landing_sound};
use crate::look::{self, InkMaterial};

/// One mesh of a prop and the colour it is inked in.
pub(crate) struct PropPart {
    pub mesh: Mesh,
    pub color: Color,
}

/// Prop meshes and materials made so far, by tile and shape (`barricade::barricade_shape`): one
/// pair per part.
pub(super) type Props = HashMap<(Tile, u8), Vec<(Handle<Mesh>, Handle<InkMaterial>)>>;

/// What stands on a cell, on the block's top at the origin: its parts, each with its colour. The
/// shape says the effect: the barricade is a wall, the mud a trap sunk in the ground, the brambles
/// a thicket that bites. `shape` is the barricade's (`barricade::barricade_shape`) or the
/// palissade's (`palisade_shape`), 0 for the others.
pub(crate) fn prop(tile: Tile, shape: u8) -> Vec<PropPart> {
    match tile {
        Tile::Barricade => barricade(shape),
        Tile::Boue => mud(),
        Tile::Ronces => brambles(),
        Tile::Eboulis => rubble(),
        Tile::Palissade => palisade(shape),
        _ => Vec::new(),
    }
}

/// The meshes made into one, if there are any.
pub(crate) fn merge(parts: impl IntoIterator<Item = Mesh>) -> Option<Mesh> {
    parts.into_iter().reduce(|mut all, part| {
        all.merge(&part).expect("primitive meshes share their attributes");
        all
    })
}

/// The wood of a small prop's handle or leg, and the pale of its string and blade.
pub(crate) const PROP_WOOD: Color = Color::srgb(0.6, 0.44, 0.27);
pub(crate) const PROP_PALE: Color = Color::srgb(0.88, 0.82, 0.68);

/// Mud: two ripple rings on the sunken slab.
fn mud() -> Vec<PropPart> {
    let ring = |major_radius: f32, at: Vec3| {
        Mesh::from(Torus { minor_radius: 0.014, major_radius })
            .scaled_by(Vec3::new(1.0, 0.5, 1.0))
            .translated_by(at)
    };
    let parts =
        vec![ring(0.13, Vec3::new(0.08, 0.0, 0.05)), ring(0.26, Vec3::new(0.04, 0.0, 0.02))];
    vec![PropPart {
        mesh: merge(parts).expect("a prop has parts"),
        color: Color::srgb(0.7, 0.58, 0.4),
    }]
}

/// Brambles: a low mound of moss, dark arches half sunk in it, thorns pointing out along their
/// backs.
fn brambles() -> Vec<PropPart> {
    let mound = Mesh::from(Sphere::new(0.42)).scaled_by(Vec3::new(1.0, 0.18, 1.0));
    let mut parts = Vec::new();
    for (major, turn, at) in [
        (0.3, 0.0, Vec3::new(0.0, 0.0, -0.12)),
        (0.26, 1.2, Vec3::new(0.12, 0.0, 0.1)),
        (0.22, -0.7, Vec3::new(-0.16, 0.0, 0.12)),
        (0.2, 2.2, Vec3::new(-0.05, 0.0, -0.02)),
    ] {
        let spin = Quat::from_rotation_y(turn);
        parts.push(
            Mesh::from(Torus { minor_radius: 0.055, major_radius: major })
                .rotated_by(spin * Quat::from_rotation_x(FRAC_PI_2))
                .translated_by(at),
        );
        for angle in [0.25 * PI, 0.5 * PI, 0.75 * PI] {
            let out = spin * Vec3::new(angle.cos(), angle.sin(), 0.0);
            parts.push(
                Mesh::from(Cone { radius: 0.025, height: 0.09 })
                    .rotated_by(Quat::from_rotation_arc(Vec3::Y, out))
                    .translated_by(at + out * (major + 0.06)),
            );
        }
    }
    // A felt mound and wire stems, painted the bramble's dull red.
    vec![
        PropPart { mesh: mound, color: Color::srgb(0.5, 0.52, 0.36) },
        PropPart {
            mesh: merge(parts).expect("a prop has parts"),
            color: Color::srgb(0.38, 0.2, 0.17),
        },
    ]
}

/// A tool's prop (`prop`), its meshes and fine-lined materials made once per tile and shape.
pub(super) fn prop_parts(
    props: &mut Props,
    tile: Tile,
    shape: u8,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<InkMaterial>,
) -> Vec<(Handle<Mesh>, Handle<InkMaterial>)> {
    props
        .entry((tile, shape))
        .or_insert_with(|| {
            prop(tile, shape)
                .into_iter()
                .map(|part| (meshes.add(part.mesh), materials.add(look::fine(part.color))))
                .collect()
        })
        .clone()
}

/// Sets `parts` on the top of cell `i`'s block; `new`, just inked by hand, it lands there.
pub(super) fn spawn_prop(
    commands: &mut Commands,
    cell: Entity,
    i: usize,
    tile: Tile,
    height: f32,
    parts: Vec<(Handle<Mesh>, Handle<InkMaterial>)>,
    new: bool,
) {
    if parts.is_empty() {
        return;
    }
    // The same cell always gets the same turn and size, from a hash of its index (as the hachures).
    let h = (i as u32).wrapping_mul(2_654_435_761);
    let jitter = (h >> 8 & 255) as f32 / 255.0 - 0.5;
    let (turn, size) = match tile {
        // Their orientation comes from their shape: no turn of their own.
        Tile::Barricade | Tile::Palissade => (0.0, 1.0),
        _ => ((h >> 16 & 255) as f32 / 255.0 * 2.0 * PI, 1.0 + jitter * 0.2),
    };
    let transform = Transform::from_xyz(0.0, height / 2.0, 0.0)
        .with_rotation(Quat::from_rotation_y(turn))
        .with_scale(Vec3::splat(size));
    let mut prop = commands.spawn((ChildOf(cell), transform, Visibility::default()));
    for (mesh, material) in parts {
        prop.with_child((Mesh3d(mesh), MeshMaterial3d(material)));
    }
    if new {
        prop.insert(Land::new(transform, landing_sound(tile)));
    }
}
