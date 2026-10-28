//! The enemy component: the species, health and place on the grid that the towers and waves read.

use bevy::prelude::*;

/// Anything the towers shoot at: walking enemies, the boss, and the bornes it plants.
#[derive(Component)]
pub struct Enemy {
    pub(super) species: usize,
    pub(super) hp: f32,
    pub(super) max_hp: f32,
    /// Cell it stands on until it reaches `next`.
    pub(super) cell: usize,
    pub(super) next: Option<usize>,
    /// Seconds left held in place (harpoon, stun).
    pub(super) pinned: f32,
    /// Seconds left walking at `SLOW` speed (Lanterne).
    pub(super) slowed: f32,
}

impl Enemy {
    /// A fresh enemy of `species` with `hp`, standing on `cell`.
    pub fn new(species: usize, hp: f32, cell: usize) -> Enemy {
        Enemy { species, hp, max_hp: hp, cell, next: None, pinned: 0.0, slowed: 0.0 }
    }

    /// Its species, an index in `Level.species`.
    pub fn species(&self) -> usize {
        self.species
    }

    /// Its HP now and at most.
    pub fn health(&self) -> (f32, f32) {
        (self.hp, self.max_hp)
    }
}
