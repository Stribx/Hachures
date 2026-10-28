//! Standing a figurine on the board: its root, the `Body` pivot and the parts.

use bevy::light::NotShadowCaster;
use bevy::prelude::*;

use super::gait::Limb;
use super::kit::Role;
use super::look::EnemyLook;

/// How high an enemy's root floats above the ground (`cell_pos`).
pub const FLOAT: f32 = 0.25;

/// The pivot of a figurine's parts: the gait moves it from `rest`, and nothing else writes its transform.
#[derive(Component)]
pub struct Body {
    pub rest: Transform,
}

/// Stands the figure of the species numbered `index` in the level's list at `at`, `scale` times the kit's size: only
/// what is seen, no walker.
pub fn spawn_figure(
    commands: &mut Commands,
    look: &EnemyLook,
    index: usize,
    at: Vec3,
    scale: f32,
) -> Entity {
    commands
        .spawn((
            Transform::from_translation(at).with_scale(Vec3::splat(scale)),
            Visibility::default(),
        ))
        .with_children(|root| {
            // The kit's ground is y = 0, and the root floats `FLOAT` above the board.
            let ground = Transform::from_xyz(0.0, -FLOAT / scale, 0.0);
            root.spawn((Body { rest: ground }, ground, Visibility::default())).with_children(
                |body| {
                    let pieces = look.figures.get(index).into_iter().flatten();
                    for piece in pieces {
                        let (at, role, side) = (piece.at, piece.role, piece.side);
                        let mut part = body.spawn((
                            Mesh3d(piece.mesh.clone()),
                            MeshMaterial3d(piece.material.clone()),
                            at,
                        ));
                        if !piece.casts_shadow {
                            part.insert(NotShadowCaster);
                        }
                        if role != Role::Body {
                            part.insert(Limb { role, rest: at, side });
                        }
                    }
                },
            );
        })
        .id()
}
