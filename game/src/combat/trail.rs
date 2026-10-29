//! What the walkers leave behind them: the boss's road, bornes and ice, and a Name saved.

use bevy::prelude::*;

use super::Enemy;
use super::boss::Borne;
use super::figure::EnemyLook;
use super::sighting::sight_line;
use crate::level::story::Trigger;
use crate::level::{Level, LevelEntity};
use crate::map;
use crate::map::grid::Tile;
use crate::map::terrain::Edit;

/// What the walkers leave for after the walk: the road and bornes the boss lays, and whether a Name
/// was saved.
#[derive(Default)]
pub(super) struct Trail {
    pub(super) chaussee: Vec<usize>,
    /// (cell, species of the boss planting it)
    pub(super) bornes: Vec<(usize, usize)>,
    /// (cell left, cell reached) where the boss sights the water ahead.
    pub(super) sights: Vec<(usize, usize)>,
    /// Relief changes made by a drilling boss.
    pub(super) relief: Vec<(usize, Tile)>,
    pub(super) saved: bool,
}

/// Lays what the boss left on the way: the road, the bornes; the map is inked again if it changed.
pub(super) fn lay(level: &mut Level, look: &EnemyLook, commands: &mut Commands, trail: Trail) {
    // [T] Le Levé: the imperial road.
    let mut batch: Vec<(usize, Edit)> = trail
        .chaussee
        .iter()
        .filter(|&&cell| level.terrain.base().tiles[cell].buildable())
        .map(|&cell| (cell, Edit::Ink(Tile::Chaussee)))
        .collect();
    // [T] La Visée: the water ahead freezes.
    for &(from, to) in &trail.sights {
        batch.extend(
            sight_line(level.terrain.base(), from, to).map(|i| (i, Edit::Ink(Tile::Glace))),
        );
    }
    for &(cell, tile) in &trail.relief {
        batch.push((cell, Edit::Land(tile)));
    }
    // [S] Le Bornage: every borne is one more target for the Jalonneurs.
    for &(cell, species) in &trail.bornes {
        batch.push((cell, Edit::PlantBorne));
        commands
            .spawn((
                LevelEntity,
                Borne,
                Enemy::new(species, level.hp(level.boss_file().borne_hp), cell),
                Transform::from_translation(map::cell_pos(level.terrain.grid(), cell)),
                Visibility::default(),
            ))
            .with_children(|borne| {
                for piece in &look.borne {
                    borne.spawn((
                        Mesh3d(piece.mesh.clone()),
                        MeshMaterial3d(piece.material.clone()),
                        piece.at,
                    ));
                }
            });
    }
    if !batch.is_empty() {
        level.alter(batch);
    }
    if trail.saved {
        level.note(Trigger::FuyardSaved);
    }
}
