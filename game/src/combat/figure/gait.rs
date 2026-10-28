//! The walk of a figurine: its step follows the ground walked, and it turns toward its heading.

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use super::jolt::{JOLT, jolt, stagger};
use super::kit::Role;
use super::spawn::Body;
use crate::combat::Enemy;
use crate::level::Level;
use crate::map::species::{Reading, Shape};

/// Radius of a wheel in figure units (the kit's wheels), so it turns with the ground walked.
const WHEEL_RADIUS: f32 = 0.1;

/// How fast the figurine turns toward its heading, in 1/s of the way left.
const TURN: f32 = 12.0;

/// How far a leg swings, in radians.
const SWING: f32 = 0.35;

/// How a figurine moves, from its silhouette.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gait {
    Pas,
    Echasses,
    Trot,
    Roule,
    /// A cart whose oxen bolt (it reads l'Ombre).
    Panique,
}

impl From<(Shape, Reading)> for Gait {
    fn from((shape, reading): (Shape, Reading)) -> Gait {
        match shape {
            Shape::Chariot if reading == Reading::Ombre => Gait::Panique,
            Shape::Pied => Gait::Pas,
            Shape::Echasses => Gait::Echasses,
            Shape::Bete => Gait::Trot,
            Shape::Chariot => Gait::Roule,
        }
    }
}

/// Strides per tile walked.
pub fn cadence(gait: Gait) -> f32 {
    match gait {
        Gait::Pas => 2.5,
        Gait::Echasses => 1.2,
        Gait::Trot => 3.0,
        Gait::Roule => 1.0,
        Gait::Panique => 2.2,
    }
}

/// The body's offset from its rest pose at `phase` (radians, period 2π); none when it stands.
/// The forward axis is X: a roll turns about it, a pitch about Z.
pub fn pose(gait: Gait, phase: f32, moving: bool) -> Transform {
    if !moving {
        return Transform::IDENTITY;
    }
    let (s, hop) = (phase.sin(), phase.sin().abs());
    let (y, roll, pitch) = match gait {
        Gait::Pas => (hop * 0.04, s * 0.12, 0.0),
        Gait::Echasses => (0.0, s * 0.08, (2.0 * phase).sin() * 0.03),
        Gait::Trot => (hop * 0.02, 0.0, s * 0.06),
        Gait::Roule => (0.0, 0.0, s * 0.025),
        // The box rocks hard and jitters.
        Gait::Panique => (0.0, (3.0 * phase).sin() * 0.05, s * 0.06),
    };
    Transform::from_xyz(0.0, y, 0.0)
        .with_rotation(Quat::from_rotation_x(roll) * Quat::from_rotation_z(pitch))
}

/// The turn of a wheel, a leg (`side` is +1 or -1, which way it swings first) or the yoke, to
/// compose with its rest rotation.
pub fn limb(role: Role, gait: Gait, side: f32, phase: f32, walked: f32) -> Quat {
    match role {
        Role::Wheel => Quat::from_rotation_z(-walked / WHEEL_RADIUS),
        Role::Leg => {
            Quat::from_rotation_z((phase + if side > 0.0 { 0.0 } else { PI }).sin() * SWING)
        }
        // The oxen bolting knock the yoke about the forward axis.
        Role::Yoke if gait == Gait::Panique => Quat::from_rotation_x((2.0 * phase).sin() * 0.25),
        Role::Yoke => Quat::IDENTITY,
        Role::Body => Quat::IDENTITY,
    }
}

/// A part the gait turns: it is set to `rest` turned by `limb(..)`.
#[derive(Component)]
pub struct Limb {
    pub role: Role,
    pub rest: Transform,
    pub side: f32,
}

/// What a walking figurine has done: the ground walked, where it stood, and the way it faces.
#[derive(Component, Default)]
pub struct Stride {
    pub walked: f32,
    last: Option<Vec3>,
    yaw: f32,
    /// The hp last seen, the seconds since it dropped, and a clock for the stagger.
    hp: f32,
    since_hit: f32,
    clock: f32,
}

impl Stride {
    /// Notes the hp seen this frame: a drop starts a jolt.
    fn feel(&mut self, hp: f32, dt: f32) {
        self.since_hit = if hp < self.hp { 0.0 } else { self.since_hit + dt };
        self.hp = hp;
        self.clock += dt;
    }
}

/// A walker's root: neither the body pivot nor a part.
type Roots = (Without<Body>, Without<Limb>);

/// The yaw of a rotation about the vertical.
fn yaw_of(rotation: Quat) -> f32 {
    rotation.to_euler(EulerRot::YXZ).0
}

/// Steps and turns every walker from the ground it covered this frame.
pub fn animate(
    time: Res<Time>,
    level: Res<Level>,
    mut walkers: Query<(&mut Transform, &mut Stride, &Enemy, &Children), Roots>,
    mut bodies: Query<(&Body, &mut Transform, &Children), Without<Limb>>,
    mut limbs: Query<(&Limb, &mut Transform), Without<Body>>,
) {
    let ease = (TURN * time.delta_secs()).min(1.0);
    for (mut transform, mut stride, enemy, children) in &mut walkers {
        let here = transform.translation;
        let last = stride.last.replace(here);
        let Some(last) = last else {
            stride.yaw = yaw_of(transform.rotation);
            (stride.hp, stride.since_hit) = (enemy.hp, JOLT);
            continue;
        };
        stride.feel(enemy.hp, time.delta_secs());
        let delta = (here - last).xz();
        let moved = delta.length();
        stride.walked += moved;
        if moved > 1e-4 {
            let gap = ((-delta.y).atan2(delta.x) - stride.yaw + PI).rem_euclid(TAU) - PI;
            stride.yaw += gap * ease;
        }
        transform.rotation = Quat::from_rotation_y(stride.yaw);
        let species = &level.species[enemy.species];
        let gait = Gait::from((species.shape, species.reading));
        let phase = stride.walked * cadence(gait) * TAU;
        let Some(child) = children.iter().find(|&c| bodies.contains(c)) else {
            continue;
        };
        let Ok((body, mut at, parts)) = bodies.get_mut(child) else {
            continue;
        };
        let sway = if enemy.pinned > 0.0 { stagger(stride.clock) } else { Transform::IDENTITY };
        *at = body.rest * pose(gait, phase, moved > 1e-5) * jolt(stride.since_hit) * sway;
        for &part in parts {
            if let Ok((joint, mut at)) = limbs.get_mut(part) {
                // The wheels are sized in figure units: roll them by the distance in those units.
                at.rotation =
                    limb(joint.role, gait, joint.side, phase, stride.walked / transform.scale.x)
                        * joint.rest.rotation;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAITS: [Gait; 5] = [Gait::Pas, Gait::Echasses, Gait::Trot, Gait::Roule, Gait::Panique];

    #[test]
    fn pose_never_goes_under_the_ground() {
        for gait in GAITS {
            for i in 0..126 {
                let y = pose(gait, i as f32 * 0.1, true).translation.y;
                assert!(y >= 0.0, "{gait:?} sinks to {y}");
            }
        }
    }

    #[test]
    fn pose_is_periodic() {
        for gait in GAITS {
            for i in 0..63 {
                let p = i as f32 * 0.1;
                let (a, b) = (pose(gait, p, true), pose(gait, p + TAU, true));
                assert!(a.translation.distance(b.translation) < 1e-4, "{gait:?}");
                // angle_between loses precision near zero (acos), so compare the vector part of the
                // difference.
                assert!((a.rotation * b.rotation.inverse()).xyz().length() < 1e-4, "{gait:?}");
            }
        }
    }

    #[test]
    fn a_still_figure_stands_still() {
        for gait in GAITS {
            for p in [0.3, 1.0, 2.5] {
                assert_eq!(pose(gait, p, false), Transform::IDENTITY);
            }
        }
    }

    #[test]
    fn stilts_stride_slower_than_feet() {
        assert!(cadence(Gait::Echasses) < cadence(Gait::Pas));
    }

    #[test]
    fn legs_alternate() {
        let leg = |side| limb(Role::Leg, Gait::Trot, side, PI / 2.0, 0.0);
        let (a, b) = (leg(1.0), leg(-1.0));
        let (a, b) = (a.to_euler(EulerRot::XYZ).2, b.to_euler(EulerRot::XYZ).2);
        assert!(a * b < 0.0, "legs swing the same way: {a} {b}");
    }

    #[test]
    fn the_yoke_knocks_only_in_panic() {
        for phase in [0.3, PI / 4.0, 1.0, 2.5] {
            assert_eq!(limb(Role::Yoke, Gait::Roule, 0.0, phase, 0.0), Quat::IDENTITY, "{phase}");
        }
        let knock = limb(Role::Yoke, Gait::Panique, 0.0, PI / 4.0, 0.0);
        assert!(knock.xyz().length() > 0.1, "{knock:?}");
    }

    #[test]
    fn a_cart_that_reads_the_shadow_panics() {
        assert_eq!(Gait::from((Shape::Chariot, Reading::Ombre)), Gait::Panique);
        assert_eq!(Gait::from((Shape::Chariot, Reading::Pente)), Gait::Roule);
        assert!(cadence(Gait::Panique) > cadence(Gait::Roule));
        // The Harde reads l'Ombre too, and trots all the same.
        assert_eq!(Gait::from((Shape::Bete, Reading::Ombre)), Gait::Trot);
        assert_eq!(Gait::from((Shape::Pied, Reading::Ombre)), Gait::Pas);
    }
}
