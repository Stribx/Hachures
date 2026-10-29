//! A barricade's figure: the arms it reaches toward the barricades beside it, and its beams, stakes
//! and string.

use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::prelude::*;

use super::grid::{Grid, Tile};
use super::props::{PROP_PALE, PROP_WOOD, PropPart, merge};

/// Shape bits of a barricade: an arm toward each barricade beside it, and the fascines once the
/// skill is learnt.
const WEST: u8 = 1;
const EAST: u8 = 2;
const NORTH: u8 = 4;
const SOUTH: u8 = 8;
const FASCINES: u8 = 16;

/// Which arms a barricade at `i` reaches out with, so a line of them is one rampart with its
/// corners and crossings.
pub(crate) fn barricade_shape(grid: &Grid, i: usize, fascines: bool) -> u8 {
    let (x, z) = grid.coords(i);
    let arms =
        grid.neighbours(i).filter(|&n| grid.tiles[n] == Tile::Barricade).fold(0, |arms, n| {
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
    /// A beam along +x from `from` to `to`, with crossed stakes at `stations` lashed on with
    /// string, and under it a bundle of fascines once the skill is learnt.
    fn add(&mut self, from: f32, to: f32, stations: &[f32], turn: f32) {
        // The stakes' low tips just touch the ground.
        let lift = 0.4 * FRAC_PI_4.cos();
        let spin = Quat::from_rotation_y(turn);
        let length = to - from;
        // Turns a rod lying along y to lie along x.
        let roll = Quat::from_rotation_z(FRAC_PI_2);
        let along = |mesh: Mesh, x: f32, y: f32, z: f32| {
            mesh.translated_by(Vec3::new(x, y, z)).rotated_by(spin)
        };
        let rod = Mesh::from(Cylinder { radius: 0.05, half_height: length / 2.0 }).rotated_by(roll);
        self.wood.push(along(rod, from + length / 2.0, lift, 0.0));
        for &x in stations {
            for dx in [-0.03, 0.03] {
                let lashing =
                    Mesh::from(Torus { minor_radius: 0.016, major_radius: 0.058 }).rotated_by(roll);
                self.string.push(along(lashing, x + dx, lift, 0.0));
            }
            for tilt in [FRAC_PI_4, -FRAC_PI_4] {
                for turn in [Quat::from_rotation_x(tilt), Quat::from_rotation_z(tilt)] {
                    self.wood.push(along(stake().rotated_by(turn), x, lift, 0.0));
                }
            }
        }
        if self.fascines {
            for (y, z) in
                [(0.03, -0.05), (0.03, 0.0), (0.03, 0.05), (0.075, -0.025), (0.075, 0.025)]
            {
                let stick =
                    Mesh::from(Cylinder { radius: 0.024, half_height: length / 2.0 - 0.02 })
                        .rotated_by(roll);
                self.sticks.push(along(stick, from + length / 2.0, y, z));
            }
        }
    }
}

/// Un cheval de frise: beams run through by stakes crossed in X both along them and across them, so
/// it bristles from every side the camera sees it. Alone, one beam across the cell; in a line, an
/// arm out to each neighbour, meeting theirs at the cell's edge.
pub(super) fn barricade(shape: u8) -> Vec<PropPart> {
    let mut beams = Beams {
        wood: Vec::new(),
        string: Vec::new(),
        sticks: Vec::new(),
        fascines: shape & FASCINES != 0,
    };
    // Turn about y taking +x to each direction: east, north (-z), west, south (+z).
    let arms = [(EAST, 0.0), (NORTH, FRAC_PI_2), (WEST, PI), (SOUTH, -FRAC_PI_2)];
    match arms.iter().filter(|(bit, _)| shape & bit != 0).count() {
        0 => beams.add(-0.46, 0.46, &[-0.24, 0.24], 0.0),
        count => {
            for &(bit, turn) in &arms {
                if shape & bit != 0 {
                    beams.add(0.0, 0.5, &[0.25], turn);
                } else if count == 1 && shape & opposite(bit) != 0 {
                    // The end of a line: the beam goes on through the cell and stops short of its
                    // edge.
                    beams.add(0.0, 0.4, &[0.22], turn);
                }
            }
        }
    }
    // Model-maker's stuff: whittled sticks of pale wood, cream string, the fascines in darker
    // twigs.
    let mut parts = vec![
        PropPart { mesh: merge(beams.wood).expect("a prop has parts"), color: PROP_WOOD },
        PropPart { mesh: merge(beams.string).expect("a prop has parts"), color: PROP_PALE },
    ];
    if let Some(sticks) = merge(beams.sticks) {
        parts.push(PropPart { mesh: sticks, color: Color::srgb(0.4, 0.28, 0.16) });
    }
    parts
}
