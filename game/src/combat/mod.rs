//! Towers, enemies, waves and La Grande Chaîne.

pub mod boss;
pub mod enemies;
#[cfg(test)]
mod tests;
pub mod towers;

use bevy::prelude::*;

use crate::Step;

pub fn plugin(app: &mut App) {
    app.init_resource::<Shots>().add_systems(Startup, enemies::setup).add_systems(
        Update,
        (
            enemies::start_waves.in_set(Step::Input),
            (enemies::spawn_enemies, enemies::walk, towers::fire, boss::boss_phases).chain().in_set(Step::Simulate),
            draw.in_set(Step::Render),
        ),
    );
}

/// Anything the towers shoot at: walking enemies, the boss, and the bornes it plants.
#[derive(Component)]
pub struct Enemy {
    species: usize,
    hp: f32,
    max_hp: f32,
    /// Cell it stands on until it reaches `next`.
    cell: usize,
    next: Option<usize>,
    /// Seconds left held in place (harpoon, stun).
    pinned: f32,
    /// Seconds left walking at `SLOW` speed (Lanterne).
    slowed: f32,
}

impl Enemy {
    /// A fresh enemy of `species` with `hp`, standing on `cell`.
    pub fn new(species: usize, hp: f32, cell: usize) -> Enemy {
        Enemy { species, hp, max_hp: hp, cell, next: None, pinned: 0.0, slowed: 0.0 }
    }
}

/// Shots drawn for a moment: (from, to, seconds left).
#[derive(Resource, Default)]
struct Shots(Vec<(Vec3, Vec3, f32)>);

/// Health bars over the enemies, and the shots fired.
fn draw(time: Res<Time>, mut shots: ResMut<Shots>, enemies: Query<(&Enemy, &Transform)>, mut gizmos: Gizmos) {
    for (enemy, t) in &enemies {
        let left = t.translation + Vec3::new(-0.25, 0.35 * t.scale.y, 0.0);
        let frac = (enemy.hp / enemy.max_hp).clamp(0.0, 1.0);
        gizmos.line(left, left + Vec3::X * 0.5 * frac, Color::srgb(1.0 - frac, frac, 0.1));
    }
    for (from, to, left) in &mut shots.0 {
        gizmos.line(*from, *to, Color::srgb(0.2, 0.1, 0.05));
        *left -= time.delta_secs();
    }
    shots.0.retain(|s| s.2 > 0.0);
}
