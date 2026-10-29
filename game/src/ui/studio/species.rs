//! The species' figurines in the studio, for the Lexique: each stands on the page itself, framed
//! whole, and turned a little so its shape shows.

use bevy::camera::primitives::MeshAabb;
use bevy::prelude::*;

use super::{Framing, Reel, spawn_camera, stand, sun};
use crate::combat::figure::EnemyLook;
use crate::ink::TOOLS;

/// The picture of each species, by its index in the level's list.
#[derive(Resource)]
pub struct SpeciesIcons(pub Vec<Handle<Image>>);

/// How far a figurine is turned from its front.
const TURN: f32 = 0.6;

/// The species stand on the spots after the tools', one each.
pub(super) fn setup(
    mut commands: Commands,
    look: Res<EnemyLook>,
    meshes: Res<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    let first = TOOLS.len();
    sun(&mut commands, first..first + look.figures.len());
    let turn = Quat::from_rotation_y(TURN);
    let mut icons = Vec::new();
    for (s, pieces) in look.figures.iter().enumerate() {
        let spot = stand(first + s, &mut images);
        let whole = framing(pieces.iter().map(|p| (&p.mesh, p.at)), &meshes, turn);
        spawn_camera(&mut commands, &spot, Reel::Species, whole);
        let figure = Transform::from_translation(spot.at).with_rotation(turn);
        for piece in pieces {
            commands.spawn((
                Mesh3d(piece.mesh.clone()),
                MeshMaterial3d(piece.material.clone()),
                figure * piece.at,
                spot.layer.clone(),
            ));
        }
        icons.push(spot.image);
    }
    commands.insert_resource(SpeciesIcons(icons));
}

/// The framing that holds a figure of `pieces` (each a mesh and its place) turned by `turn`: it
/// looks at the middle of their box and sees the ball around it, so no side is cut.
pub(super) fn framing<'a>(
    pieces: impl IntoIterator<Item = (&'a Handle<Mesh>, Transform)>,
    meshes: &Assets<Mesh>,
    turn: Quat,
) -> Framing {
    let (mut low, mut high) = (Vec3::INFINITY, Vec3::NEG_INFINITY);
    for (mesh, at) in pieces {
        let Some(aabb) = meshes.get(mesh).and_then(MeshAabb::get_aabb) else { continue };
        // The box once placed, along the axes (an over-estimate when it is turned).
        let centre = at.transform_point(aabb.center.into());
        let half = Mat3::from_quat(at.rotation).abs() * (at.scale * Vec3::from(aabb.half_extents));
        (low, high) = (low.min(centre - half), high.max(centre + half));
    }
    if !low.is_finite() {
        return Framing { side: 1.0, aim: Vec3::ZERO };
    }
    // A little room around the ball, so the figure does not touch the edge.
    Framing { side: (high - low).length() * 1.05, aim: turn * (low + high) / 2.0 }
}
