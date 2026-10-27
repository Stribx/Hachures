//! How enemies look (their parts and inks, made once) and the shards of ink they break into.

use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::prelude::*;

use crate::level::{Level, LevelEntity};
use crate::look::{InkMaterial, ink};

/// Seconds the shards of a defeated enemy fly.
const SHARD_LIFE: f32 = 0.6;

#[derive(Resource, Default)]
pub struct EnemyLook {
    /// The parts of each `Shape` (by its index), placed around the enemy's root, which floats 0.25 above the ground.
    pub shapes: [Vec<(Handle<Mesh>, Transform)>; 4],
    pub materials: Vec<Handle<InkMaterial>>,
    pub borne_mesh: Handle<Mesh>,
    pub borne_material: Handle<InkMaterial>,
    pub shard_mesh: Handle<Mesh>,
    pub shard_material: Handle<InkMaterial>,
}

pub fn setup(
    mut commands: Commands,
    level: Res<Level>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    let mut part = |mesh: Mesh, x: f32, y: f32, z: f32| (meshes.add(mesh), Transform::from_xyz(x, y, z));
    // Heads look along +X: `walk` turns the root towards the next cell.
    let pied =
        vec![part(Capsule3d::new(0.08, 0.16).into(), 0.0, -0.09, 0.0), part(Sphere::new(0.065).into(), 0.0, 0.13, 0.0)];
    let echasses = vec![
        part(Cylinder::new(0.018, 0.3).into(), 0.0, -0.1, 0.05),
        part(Cylinder::new(0.018, 0.3).into(), 0.0, -0.1, -0.05),
        part(Capsule3d::new(0.08, 0.1).into(), 0.0, 0.18, 0.0),
        part(Sphere::new(0.06).into(), 0.04, 0.37, 0.0),
    ];
    let mut bete = vec![
        part(Cuboid::new(0.42, 0.18, 0.24).into(), 0.0, -0.08, 0.0),
        part(Cuboid::new(0.13, 0.13, 0.12).into(), 0.25, 0.0, 0.0),
    ];
    let mut chariot = vec![part(Cuboid::new(0.5, 0.22, 0.3).into(), 0.0, -0.04, 0.0)];
    for (x, z) in [(0.15, 0.09), (0.15, -0.09), (-0.15, 0.09), (-0.15, -0.09)] {
        bete.push(part(Cylinder::new(0.025, 0.1).into(), x, -0.2, z));
    }
    for (x, z) in [(0.15, 0.17), (0.15, -0.17), (-0.15, 0.17), (-0.15, -0.17)] {
        let (mesh, t) = part(Cylinder::new(0.1, 0.04).into(), x, -0.15, z);
        chariot.push((mesh, t.with_rotation(Quat::from_rotation_x(FRAC_PI_2))));
    }
    commands.insert_resource(EnemyLook {
        shapes: [pied, echasses, bete, chariot],
        materials: level
            .species
            .iter()
            .map(|s| materials.add(ink(Color::srgb(s.color.0, s.color.1, s.color.2))))
            .collect(),
        borne_mesh: meshes.add(Cylinder::new(0.1, 0.6)),
        borne_material: materials.add(ink(Color::srgb(0.85, 0.82, 0.7))),
        shard_mesh: meshes.add(Cuboid::from_length(0.06)),
        shard_material: materials.add(ink(Color::srgb(0.12, 0.08, 0.05))),
    });
}

/// A splinter of ink from a defeated enemy (docs, section 1: units break into shards of ink, never blood).
#[derive(Component)]
pub struct Shard {
    velocity: Vec3,
    left: f32,
}

/// Breaks a defeated enemy at `at` into eight shards of ink, fanned out.
pub fn shatter(commands: &mut Commands, look: &EnemyLook, at: Vec3) {
    for k in 0..8 {
        let angle = k as f32 * TAU / 8.0;
        commands.spawn((
            LevelEntity,
            Shard { velocity: Vec3::new(angle.cos() * 1.4, 1.6, angle.sin() * 1.4), left: SHARD_LIFE },
            Mesh3d(look.shard_mesh.clone()),
            MeshMaterial3d(look.shard_material.clone()),
            Transform::from_translation(at),
        ));
    }
}

/// Shards fall and shrink away.
pub fn shards(time: Res<Time>, mut shards: Query<(Entity, &mut Shard, &mut Transform)>, mut commands: Commands) {
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
