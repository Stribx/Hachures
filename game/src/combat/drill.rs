//! The Sonde's piece, from plain shapes until it is carved: a tripod over a drill rod.

use std::f32::consts::TAU;

use bevy::prelude::*;

use super::stand::Tower;
use super::towers::Weapon;
use crate::look::{InkMaterial, ink};
use crate::map::props::{PROP_WOOD, PropPart};

/// Three wooden legs leaning in over an iron rod, its bit at the foot and a brass head on top; the
/// tower's top at the origin.
// ponytail: plain shapes stand in for the Sonde's carved piece; swap for a model when one is drawn.
pub(crate) fn sonde() -> Vec<PropPart> {
    let wood = PROP_WOOD;
    let mut parts: Vec<PropPart> = (0..3)
        .map(|k| {
            let around = Quat::from_rotation_y(k as f32 * TAU / 3.0);
            let leg = Mesh::from(Cylinder { radius: 0.025, half_height: 0.3 })
                .rotated_by(Quat::from_rotation_z(0.35))
                .translated_by(Vec3::new(0.12, 0.28, 0.0))
                .rotated_by(around);
            PropPart { mesh: leg, color: wood }
        })
        .collect();
    let rod =
        Mesh::from(Cylinder { radius: 0.035, half_height: 0.32 }).translated_by(Vec3::Y * 0.3);
    let bit = Mesh::from(Cone { radius: 0.07, height: 0.14 })
        .rotated_by(Quat::from_rotation_x(std::f32::consts::PI))
        .translated_by(Vec3::Y * 0.02);
    let head = Mesh::from(Cuboid::new(0.14, 0.08, 0.14)).translated_by(Vec3::Y * 0.6);
    parts.push(PropPart { mesh: rod, color: Color::srgb(0.35, 0.36, 0.4) });
    parts.push(PropPart { mesh: bit, color: Color::srgb(0.3, 0.3, 0.33) });
    parts.push(PropPart { mesh: head, color: Color::srgb(0.75, 0.6, 0.3) });
    parts
}

/// A Sonde just stood up gets its piece: it has no model to load. The meshes and paints are made
/// the first time.
pub(super) fn dress_sondes(
    towers: Query<(Entity, &Tower), Added<Tower>>,
    mut made: Local<Vec<(Handle<Mesh>, Handle<InkMaterial>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
    mut commands: Commands,
) {
    for (entity, tower) in &towers {
        if tower.weapon != Weapon::Sonde {
            continue;
        }
        if made.is_empty() {
            *made = sonde()
                .into_iter()
                .map(|p| (meshes.add(p.mesh), materials.add(ink(p.color))))
                .collect();
        }
        commands.entity(entity).with_children(|piece| {
            for (mesh, paint) in made.iter() {
                piece.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(paint.clone())));
            }
        });
    }
}
