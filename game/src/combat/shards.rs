//! The pieces a defeated enemy breaks into: shards, droplets of sépia and a blot on the ground.

use std::f32::consts::TAU;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;

use super::figure::{EnemyLook, FLOAT};
use crate::level::LevelEntity;
use crate::look::InkMaterial;

/// Seconds the shards of a defeated enemy fly, and the blot lasts.
const SHARD_LIFE: f32 = 0.6;
const BLOT: f32 = 1.5;

/// A splinter of ink from a defeated enemy (docs, section 1: units break into shards of ink, never
/// blood).
#[derive(Component)]
pub struct Shard {
    velocity: Vec3,
    left: f32,
}

/// A stain of sépia on the ground, shrinking away.
#[derive(Component)]
pub struct Blot {
    left: f32,
}

/// `count` pieces of `mesh` fanned out from `at`, `speed` sideways and `up` upward.
fn fan(
    commands: &mut Commands,
    at: Vec3,
    (mesh, material): (&Handle<Mesh>, &Handle<InkMaterial>),
    count: u32,
    speed: f32,
    up: f32,
) {
    for k in 0..count {
        let angle = k as f32 * TAU / count as f32;
        commands.spawn((
            LevelEntity,
            NotShadowCaster,
            Shard {
                velocity: Vec3::new(angle.cos() * speed, up, angle.sin() * speed),
                left: SHARD_LIFE,
            },
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(at),
        ));
    }
}

/// Breaks a defeated borne at `at` into eight dark shards of ink, fanned out.
pub fn shatter(commands: &mut Commands, look: &EnemyLook, at: Vec3) {
    fan(commands, at, (&look.shard_mesh, &look.shard_material), 8, 1.4, 1.6);
}

/// Breaks a defeated figure of `species` at `at` into shards of its coat and droplets of sépia, and
/// leaves a blot.
pub fn break_up(commands: &mut Commands, look: &EnemyLook, species: usize, at: Vec3) {
    let coat = look.coats.get(species).cloned().unwrap_or_default();
    fan(commands, at, (&look.shard_mesh, &coat), 6, 1.4, 1.6);
    fan(commands, at, (&look.drop_mesh, &look.sepia_material), 4, 0.9, 2.0);
    commands.spawn((
        LevelEntity,
        Blot { left: BLOT },
        Mesh3d(look.blot_mesh.clone()),
        MeshMaterial3d(look.sepia_material.clone()),
        // Just above the board under the figure's feet.
        Transform::from_xyz(at.x, at.y - FLOAT + 0.01, at.z),
    ));
}

/// Shards fall and shrink away.
pub fn shards(
    time: Res<Time>,
    mut shards: Query<(Entity, &mut Shard, &mut Transform)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut shard, mut transform) in &mut shards {
        shard.left -= dt;
        if shard.left <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        shard.velocity.y -= 5.0 * dt;
        transform.translation += shard.velocity * dt;
        transform.scale = Vec3::splat(shard.left / SHARD_LIFE);
    }
}

/// Blots shrink away (the material is shared, so by scale, not by alpha).
pub fn blots(
    time: Res<Time>,
    mut blots: Query<(Entity, &mut Blot, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, mut blot, mut transform) in &mut blots {
        blot.left -= time.delta_secs();
        if blot.left <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let size = blot.left / BLOT;
        transform.scale = Vec3::new(size, 1.0, size);
    }
}
