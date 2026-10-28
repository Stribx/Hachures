//! Towers, enemies, waves and La Grande Chaîne.

pub mod boss;
pub mod enemies;
pub mod figure;
mod noted;
mod range;
mod shards;
#[cfg(test)]
mod tests;
pub mod towers;

use bevy::prelude::*;

use crate::Step;

pub fn plugin(app: &mut App) {
    app.init_resource::<Shots>().add_systems(Startup, figure::setup).add_systems(
        Update,
        (
            enemies::start_waves.in_set(Step::Input),
            (towers::raise_towers, towers::take_down).in_set(Step::Input).after(crate::ink::inking),
            boss::drop_bornes.in_set(Step::Input).after(crate::ink::inking),
            (enemies::spawn_enemies, noted::notice, enemies::walk, towers::fire, boss::boss_phases, shards::shards)
                .chain()
                .in_set(Step::Simulate)
                .before(crate::level::finish_level),
            figure::animate.in_set(Step::Simulate).after(enemies::walk),
            figure::drag_chain.in_set(Step::Simulate).after(figure::animate),
            (figure::exits, shards::blots).in_set(Step::Simulate),
            (draw, range::draw).in_set(Step::Render),
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

    /// Its species, an index in `Level.species`.
    pub fn species(&self) -> usize {
        self.species
    }

    /// Its HP now and at most.
    pub fn health(&self) -> (f32, f32) {
        (self.hp, self.max_hp)
    }
}

/// Shots drawn for a moment.
#[derive(Resource, Default)]
struct Shots(Vec<Shot>);

struct Shot {
    from: Vec3,
    to: Vec3,
    /// Seconds left on screen.
    left: f32,
}

/// Health bars over the enemies, and the shots fired.
fn draw(time: Res<Time>, mut shots: ResMut<Shots>, enemies: Query<(&Enemy, &Transform)>, mut gizmos: Gizmos) {
    for (enemy, t) in &enemies {
        let left = t.translation + Vec3::new(-0.25, 0.35 * t.scale.y, 0.0);
        let frac = (enemy.hp / enemy.max_hp).clamp(0.0, 1.0);
        // Ink on a strip of paper; the seal's red once it is nearly broken.
        gizmos.line(left, left + Vec3::X * 0.5, crate::theme::SHEET);
        let ink = if frac < 0.35 { crate::theme::SEAL } else { crate::theme::WORLD_INK };
        gizmos.line(left + Vec3::Z * 0.01, left + Vec3::new(0.5 * frac, 0.0, 0.01), ink);
    }
    for shot in &mut shots.0 {
        gizmos.line(shot.from, shot.to, Color::srgb(0.2, 0.1, 0.05));
        shot.left -= time.delta_secs();
    }
    shots.0.retain(|s| s.left > 0.0);
}
