//! How a figurine leaves the board: it falls, sinks or hops away as a ghost, a copy with no `Enemy`.

use bevy::prelude::*;

use std::f32::consts::{FRAC_PI_2, PI};

use super::look::EnemyLook;
use super::spawn::spawn_figure;
use crate::combat::shards::break_up;
use crate::level::LevelEntity;
use crate::map::species::Goal;

/// Seconds to tip over, to sink and to hop away.
const FALL: f32 = 0.15;
const SINK: f32 = 0.3;
const HOP: f32 = 0.4;

/// How a ghost goes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Tips over sideways: a figure defeated.
    Fall,
    /// Sinks into the board like a pin pulled out: an enemy at its goal.
    Sink,
    /// A little hop, shrinking away: an ally safe at its goal.
    Hop,
}

impl Kind {
    /// How a figurine reaching `goal` leaves: an ally hops, an enemy sinks.
    pub fn arriving(goal: Goal) -> Kind {
        match goal {
            Goal::Refuge | Goal::Sortie => Kind::Hop,
            Goal::Cible | Goal::BorneMere => Kind::Sink,
        }
    }
}

/// A figurine on its way out.
#[derive(Component)]
pub struct Exit {
    pub kind: Kind,
    age: f32,
    from: Transform,
    species: usize,
}

/// The ghost's offset from where it started, `t` seconds in.
pub fn exit_pose(kind: Kind, t: f32) -> Transform {
    let u = (t / exit_life(kind)).clamp(0.0, 1.0);
    match kind {
        // Ease-in, about the forward axis (X).
        Kind::Fall => Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2 * u * u)),
        Kind::Sink => Transform::from_xyz(0.0, -0.5 * u, 0.0),
        Kind::Hop => Transform::from_xyz(0.0, 0.15 * (PI * u).sin(), 0.0).with_scale(Vec3::splat(1.0 - u)),
    }
}

/// Seconds the exit lasts.
pub fn exit_life(kind: Kind) -> f32 {
    match kind {
        Kind::Fall => FALL,
        Kind::Sink => SINK,
        Kind::Hop => HOP,
    }
}

/// Stands a ghost of `species` where `from` stood.
pub fn leave(commands: &mut Commands, look: &EnemyLook, species: usize, from: &Transform, kind: Kind) {
    let ghost = spawn_figure(commands, look, species, from.translation, from.scale.x);
    commands.entity(ghost).insert((LevelEntity, *from, Exit { kind, age: 0.0, from: *from, species }));
}

/// Plays the ghosts out; a figure that fell breaks into shards where it lay.
pub fn exits(
    time: Res<Time>,
    look: Res<EnemyLook>,
    mut ghosts: Query<(Entity, &mut Exit, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut exit, mut at) in &mut ghosts {
        exit.age += time.delta_secs();
        if exit.age < exit_life(exit.kind) {
            let pose = exit_pose(exit.kind, exit.age);
            *at = Transform {
                translation: exit.from.translation + pose.translation,
                rotation: exit.from.rotation * pose.rotation,
                scale: exit.from.scale * pose.scale,
            };
            continue;
        }
        if exit.kind == Kind::Fall {
            break_up(&mut commands, &look, exit.species, exit.from.translation);
        }
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;

    const KINDS: [Kind; 3] = [Kind::Fall, Kind::Sink, Kind::Hop];

    #[test]
    fn exits_end_where_they_should() {
        for kind in KINDS {
            let start = exit_pose(kind, 0.0);
            let still = start.translation.length() < 1e-5
                && start.rotation.xyz().length() < 1e-5
                && start.scale.distance(Vec3::ONE) < 1e-5;
            assert!(still, "{kind:?} does not start at rest: {start:?}");
        }
        let end = |kind: Kind| exit_pose(kind, exit_life(kind));
        assert!((end(Kind::Sink).translation.y + 0.5).abs() < 1e-3, "a pin pulled out: {:?}", end(Kind::Sink));
        let tipped = end(Kind::Fall).rotation.angle_between(Quat::IDENTITY);
        assert!((tipped - FRAC_PI_2).abs() < 1e-3, "lies on its side: {tipped}");
        let hop = end(Kind::Hop);
        assert!(hop.scale.max_element() < 1e-3 && hop.translation.y.abs() < 1e-3, "gone: {hop:?}");
    }
}
