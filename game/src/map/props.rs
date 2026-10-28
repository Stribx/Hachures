//! The props that stand on a tool's cell (barricade, mud, brambles): their shapes, meshes and setting down.

use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::prelude::*;

use super::decor::{palisade, rubble};
use super::grid::{Grid, Tile};
use super::land::{Land, landing_sound};
use crate::look::{self, Hachure, InkMaterial};

/// One mesh of a prop and the colour it is inked in.
pub(crate) struct PropPart {
    pub mesh: Mesh,
    pub color: Color,
}

/// Prop meshes and materials made so far, by tile and shape (`barricade_shape`): one pair per part.
pub(super) type Props = HashMap<(Tile, u8), Vec<(Handle<Mesh>, Handle<InkMaterial>)>>;

/// Shape bits of a barricade: an arm toward each barricade beside it, and the fascines once the skill is learnt.
const WEST: u8 = 1;
const EAST: u8 = 2;
const NORTH: u8 = 4;
const SOUTH: u8 = 8;
const FASCINES: u8 = 16;

/// Which arms a barricade at `i` reaches out with, so a line of them is one rampart with its corners and crossings.
pub(crate) fn barricade_shape(grid: &Grid, i: usize, fascines: bool) -> u8 {
    let (x, z) = grid.coords(i);
    let arms = grid.neighbours(i).filter(|&n| grid.tiles[n] == Tile::Barricade).fold(0, |arms, n| {
        let (nx, nz) = grid.coords(n);
        arms | match (nx.cmp(&x), nz.cmp(&z)) {
            (std::cmp::Ordering::Less, _) => WEST,
            (std::cmp::Ordering::Greater, _) => EAST,
            (_, std::cmp::Ordering::Less) => NORTH,
            _ => SOUTH,
        }
    });
    arms | if fascines { FASCINES } else { 0 }
}

/// The arm on the other side.
fn opposite(bit: u8) -> u8 {
    match bit {
        WEST => EAST,
        EAST => WEST,
        NORTH => SOUTH,
        _ => NORTH,
    }
}

/// What stands on a cell, on the block's top at the origin: its parts, each with its colour. The shape says
/// the effect: the barricade is a wall, the mud a trap sunk in the ground, the brambles a thicket that bites.
/// `shape` is the barricade's (`barricade_shape`) or the palissade's (`palisade_shape`), 0 for the others.
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
pub(super) fn merge(parts: Vec<Mesh>) -> Option<Mesh> {
    parts.into_iter().reduce(|mut all, part| {
        all.merge(&part).expect("primitive meshes share their attributes");
        all
    })
}

/// A stake: a rod pointed at both ends.
fn stake() -> Mesh {
    let tip = Mesh::from(Cone { radius: 0.035, height: 0.1 });
    merge(vec![
        Mesh::from(Cylinder { radius: 0.035, half_height: 0.3 }),
        tip.clone().translated_by(Vec3::Y * 0.35),
        tip.rotated_by(Quat::from_rotation_x(PI)).translated_by(Vec3::Y * -0.35),
    ])
    .expect("a stake has parts")
}

/// The parts of a barricade's beams, by what they are made of, and whether they carry fascines.
struct Beams {
    wood: Vec<Mesh>,
    string: Vec<Mesh>,
    sticks: Vec<Mesh>,
    fascines: bool,
}

impl Beams {
    /// A beam along +x from `from` to `to`, with crossed stakes at `stations` lashed on with string, and under it a
    /// bundle of fascines once the skill is learnt.
    fn add(&mut self, from: f32, to: f32, stations: &[f32], turn: f32) {
        // The stakes' low tips just touch the ground.
        let lift = 0.4 * FRAC_PI_4.cos();
        let spin = Quat::from_rotation_y(turn);
        let length = to - from;
        // Turns a rod lying along y to lie along x.
        let roll = Quat::from_rotation_z(FRAC_PI_2);
        let along = |mesh: Mesh, x: f32, y: f32, z: f32| mesh.translated_by(Vec3::new(x, y, z)).rotated_by(spin);
        let rod = Mesh::from(Cylinder { radius: 0.05, half_height: length / 2.0 }).rotated_by(roll);
        self.wood.push(along(rod, from + length / 2.0, lift, 0.0));
        for &x in stations {
            for dx in [-0.03, 0.03] {
                let lashing = Mesh::from(Torus { minor_radius: 0.016, major_radius: 0.058 }).rotated_by(roll);
                self.string.push(along(lashing, x + dx, lift, 0.0));
            }
            for tilt in [FRAC_PI_4, -FRAC_PI_4] {
                for turn in [Quat::from_rotation_x(tilt), Quat::from_rotation_z(tilt)] {
                    self.wood.push(along(stake().rotated_by(turn), x, lift, 0.0));
                }
            }
        }
        if self.fascines {
            for (y, z) in [(0.03, -0.05), (0.03, 0.0), (0.03, 0.05), (0.075, -0.025), (0.075, 0.025)] {
                let stick = Mesh::from(Cylinder { radius: 0.024, half_height: length / 2.0 - 0.02 }).rotated_by(roll);
                self.sticks.push(along(stick, from + length / 2.0, y, z));
            }
        }
    }
}

/// Un cheval de frise: beams run through by stakes crossed in X both along them and across them, so it bristles from
/// every side the camera sees it. Alone, one beam across the cell; in a line, an arm out to each neighbour, meeting
/// theirs at the cell's edge.
fn barricade(shape: u8) -> Vec<PropPart> {
    let mut beams = Beams { wood: Vec::new(), string: Vec::new(), sticks: Vec::new(), fascines: shape & FASCINES != 0 };
    // Turn about y taking +x to each direction: east, north (-z), west, south (+z).
    let arms = [(EAST, 0.0), (NORTH, FRAC_PI_2), (WEST, PI), (SOUTH, -FRAC_PI_2)];
    match arms.iter().filter(|(bit, _)| shape & bit != 0).count() {
        0 => beams.add(-0.46, 0.46, &[-0.24, 0.24], 0.0),
        count => {
            for &(bit, turn) in &arms {
                if shape & bit != 0 {
                    beams.add(0.0, 0.5, &[0.25], turn);
                } else if count == 1 && shape & opposite(bit) != 0 {
                    // The end of a line: the beam goes on through the cell and stops short of its edge.
                    beams.add(0.0, 0.4, &[0.22], turn);
                }
            }
        }
    }
    // Model-maker's stuff: whittled sticks of pale wood, cream string, the fascines in darker twigs.
    let mut parts = vec![
        PropPart { mesh: merge(beams.wood).expect("a prop has parts"), color: Color::srgb(0.6, 0.44, 0.27) },
        PropPart { mesh: merge(beams.string).expect("a prop has parts"), color: Color::srgb(0.88, 0.82, 0.68) },
    ];
    if let Some(sticks) = merge(beams.sticks) {
        parts.push(PropPart { mesh: sticks, color: Color::srgb(0.4, 0.28, 0.16) });
    }
    parts
}

/// Mud: two ripple rings on the sunken slab.
fn mud() -> Vec<PropPart> {
    let ring = |major_radius: f32, at: Vec3| {
        Mesh::from(Torus { minor_radius: 0.014, major_radius }).scaled_by(Vec3::new(1.0, 0.5, 1.0)).translated_by(at)
    };
    let parts = vec![ring(0.13, Vec3::new(0.08, 0.0, 0.05)), ring(0.26, Vec3::new(0.04, 0.0, 0.02))];
    vec![PropPart { mesh: merge(parts).expect("a prop has parts"), color: Color::srgb(0.7, 0.58, 0.4) }]
}

/// Brambles: a low mound of moss, dark arches half sunk in it, thorns pointing out along their backs.
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
        PropPart { mesh: merge(parts).expect("a prop has parts"), color: Color::srgb(0.38, 0.2, 0.17) },
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
                .map(|part| {
                    let mut paint = look::ink(part.color);
                    paint.extension = Hachure::fine();
                    (meshes.add(part.mesh), materials.add(paint))
                })
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
