//! The props a figurine carries: where each one sits on a body, from the body's anchors.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::kit::Paint::{Coat, Felt, Hide, Iron, Lamp, Linen, Slate, Wood};
use super::kit::{Anchors, Form, Part, Role, body};
use crate::map::species::{Carry, Shape};

/// How much smaller than the Harde an ox is.
const OX: f32 = 0.55;

/// Where the oxen's yoke lies in front of the chest, and the oxen abreast of it.
const YOKE: f32 = 0.567;

/// An ox: the Harde's body in hide, with a horn each side of the head, its middle `(x, z)` from the
/// origin.
fn ox(x: f32, z: f32) -> Vec<Part> {
    let at = Part::at;
    let hide = |p: Part| if p.paint == Coat { p.painted(Hide) } else { p };
    let mut ox: Vec<Part> = (body(Shape::Bete).0.into_iter())
        .filter(|p| p.paint != Felt)
        .map(|p| hide(p.scaled(OX)).shifted(Vec3::new(x, 0.0, z)))
        .collect();
    for side in [1.0, -1.0] {
        let horn = at(
            Form::Cylinder { radius: 0.008, height: 0.05 },
            x + 0.25 * OX,
            0.215,
            z + side * 0.035,
            Linen,
        );
        ox.push(horn.turned(Quat::from_rotation_x(side * 0.5)));
    }
    ox
}

/// The parts of the Grande Chaîne's own `carry`s: the oxen and the yoke, Gast and the sighting
/// lantern.
fn assembly(carry: Carry, a: &Anchors) -> Vec<Part> {
    let at = Part::at;
    let cx = a.chest.x;
    match carry {
        // A pole from the cart's front to the yoke, between two oxen walking abreast.
        Carry::Boeufs => {
            let mut parts = vec![at(
                Form::Cuboid(Vec3::new(YOKE - 0.25, 0.02, 0.02)),
                cx + (YOKE + 0.25) / 2.0,
                0.17,
                0.0,
                Wood,
            )];
            parts.extend(ox(cx + YOKE - 0.1, 0.085));
            parts.extend(ox(cx + YOKE - 0.1, -0.085));
            parts
        }
        Carry::Joug => {
            vec![
                at(Form::Cuboid(Vec3::new(0.03, 0.025, 0.24)), cx + YOKE, 0.19, 0.0, Wood)
                    .role(Role::Yoke),
            ]
        }
        // The Pied, smaller, in slate cloth with a bicorne, standing on the box at its back.
        Carry::Gast => {
            let figure = (body(Shape::Pied).0.into_iter()).filter(|p| p.paint != Felt);
            let up = Vec3::new(cx - 0.06, a.crown - 0.03 * 0.7, 0.0);
            let cloth = |p: Part| if p.paint == Coat { p.painted(Slate) } else { p };
            let mut parts: Vec<Part> = figure.map(|p| cloth(p).scaled(0.7).shifted(up)).collect();
            parts.push(at(
                Form::Cuboid(Vec3::new(0.05, 0.025, 0.12)),
                cx - 0.06,
                up.y + 0.415 * 0.7 + 0.005,
                0.0,
                Wood,
            ));
            parts
        }
        // A post at the front of the box, a glowing lantern on it.
        Carry::Visee => vec![
            at(Form::Cylinder { radius: 0.01, height: 0.14 }, cx + 0.2, a.crown + 0.07, 0.0, Wood),
            at(Form::Sphere(0.035), cx + 0.2, a.crown + 0.17, 0.0, Lamp),
        ],
        _ => Vec::new(),
    }
}

/// The parts of one `carry` on a body with `a`.
pub(super) fn props(carry: Carry, a: &Anchors) -> Vec<Part> {
    let at = Part::at;
    let (cx, cy) = (a.chest.x, a.chest.y);
    let (hx, hy) = (a.head.x, a.head.y);
    match carry {
        // Athwart, sunk a little into the crown.
        Carry::Bicorne => {
            vec![at(Form::Cuboid(Vec3::new(0.07, 0.035, 0.17)), hx, a.crown + 0.0075, 0.0, Wood)]
        }
        // A bandolier: a ring a little wider than the coat, leaning across the chest.
        Carry::Chaine => vec![
            at(Form::Torus { minor: 0.014, major: 0.085 }, cx, cy, 0.0, Iron)
                .turned(Quat::from_rotation_y(0.2) * Quat::from_rotation_x(0.5)),
        ],
        // Held at the side, on the ground: a linen pole with three iron bands.
        Carry::Jalon => {
            let mut parts = vec![at(
                Form::Cylinder { radius: 0.015, height: 0.55 },
                cx + 0.02,
                0.275,
                0.11,
                Linen,
            )];
            parts.extend([0.10, 0.275, 0.45].map(|y| {
                at(Form::Cylinder { radius: 0.017, height: 0.06 }, cx + 0.02, y, 0.11, Iron)
            }));
            parts
        }
        // The brim around the crown, the crown of the hat above it.
        Carry::Paille => vec![
            at(Form::Cylinder { radius: 0.13, height: 0.015 }, hx, a.crown - 0.02, 0.0, Linen),
            at(
                Form::Frustum { bottom: 0.06, top: 0.035, height: 0.05 },
                hx,
                a.crown + 0.0125,
                0.0,
                Linen,
            ),
        ],
        Carry::Fichu => {
            vec![at(
                Form::Frustum { bottom: 0.07, top: 0.04, height: 0.05 },
                hx,
                a.crown - 0.02,
                0.0,
                Linen,
            )]
        }
        Carry::Capuche => vec![at(
            Form::Frustum { bottom: 0.075, top: 0.01, height: 0.12 },
            hx,
            hy + 0.02,
            0.0,
            Linen,
        )],
        Carry::Ballot
        | Carry::Enfant
        | Carry::Corde
        | Carry::Piolet
        | Carry::Bats
        | Carry::Bache
        | Carry::Boeufs
        | Carry::Joug
        | Carry::Gast
        | Carry::Visee
        | Carry::Meneur => carried(carry, a),
    }
}

/// The parts of a `carry` held or worn on the body's back and sides, or standing beside it, on a
/// body with `a`.
fn carried(carry: Carry, a: &Anchors) -> Vec<Part> {
    let at = Part::at;
    let cx = a.chest.x;
    match carry {
        Carry::Ballot => {
            vec![at(Form::Cuboid(Vec3::new(0.09, 0.10, 0.11)), cx - 0.09, 0.24, 0.0, Linen)]
        }
        Carry::Enfant => vec![
            at(
                Form::Frustum { bottom: 0.04, top: 0.025, height: 0.07 },
                cx + 0.02,
                0.20,
                0.09,
                Coat,
            ),
            at(Form::Sphere(0.03), cx + 0.02, 0.26, 0.09, Wood),
        ],
        // A coil standing on its edge against the back.
        Carry::Corde => vec![
            at(Form::Torus { minor: 0.012, major: 0.05 }, cx - 0.075, 0.22, 0.0, Linen)
                .turned(Quat::from_rotation_z(FRAC_PI_2)),
        ],
        Carry::Piolet => vec![
            at(Form::Cylinder { radius: 0.012, height: 0.32 }, cx + 0.03, 0.16, -0.11, Wood),
            at(Form::Cuboid(Vec3::new(0.07, 0.015, 0.015)), cx + 0.03, 0.32, -0.11, Iron),
        ],
        Carry::Bats => [0.15, -0.15]
            .map(|z| at(Form::Cuboid(Vec3::new(0.16, 0.12, 0.06)), cx, 0.27, z, Linen))
            .to_vec(),
        // Half sunk in the box, it reads as a hood.
        Carry::Bache => {
            vec![
                at(Form::Cylinder { radius: 0.14, height: 0.46 }, cx, 0.36, 0.0, Linen)
                    .turned(Quat::from_rotation_z(FRAC_PI_2)),
            ]
        }
        // A small figurine standing on the ground beside the front of the cart.
        Carry::Boeufs | Carry::Joug | Carry::Gast | Carry::Visee => assembly(carry, a),
        Carry::Meneur => vec![
            at(Form::Frustum { bottom: 0.07, top: 0.04, height: 0.18 }, cx + 0.2, 0.09, 0.26, Coat),
            at(Form::Sphere(0.045), cx + 0.2, 0.215, 0.26, Wood),
        ],
        // Named, not `_`: a new `Carry` must be placed in one of the two matches.
        Carry::Bicorne
        | Carry::Chaine
        | Carry::Jalon
        | Carry::Paille
        | Carry::Fichu
        | Carry::Capuche => unreachable!("{carry:?} is dressed by `props`"),
    }
}
