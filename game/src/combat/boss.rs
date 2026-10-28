//! A world's boss (`boss` in `assets/data/worlds/`): its phases, and the bornes it plants.

use bevy::prelude::*;
use serde::Deserialize;

use super::Enemy;
use super::figure::EnemyLook;
use super::shards::shatter;
use crate::level::Level;
use crate::level::story::Trigger;
use crate::map::species::index_of;

/// One phase of the boss.
#[derive(Deserialize)]
pub struct Phase {
    /// Its key in `assets/data/i18n/`, which is also what level notes `on: Boss(…)` wait for.
    pub name: String,
    /// Lasts until the boss's HP fraction drops to this.
    pub until: f32,
    pub species: String,
    /// Lays Chaussée on every n-th cell it leaves (0: never).
    #[serde(default)]
    pub chaussee_every: u32,
    /// Arbalète hits that stun it (0: never), and for how many seconds.
    #[serde(default)]
    pub stun_hits: u32,
    #[serde(default)]
    pub stun: f32,
    /// Bornes planted, one every other cell.
    #[serde(default)]
    pub bornes: u32,
    /// Whether a harpoon holds it.
    #[serde(default)]
    pub pinnable: bool,
}

#[derive(Deserialize)]
pub struct BossFile {
    /// As the HUD names it.
    pub name: String,
    /// Rôle lost when it reaches the Cible.
    pub breach: u32,
    pub borne_hp: f32,
    pub phases: Vec<Phase>,
}

impl Level {
    /// The boss of this level's world. Only asked for with a boss on the map, which comes from it.
    ///
    /// # Panics
    ///
    /// If this level's world has no boss.
    pub fn boss_file(&self) -> &'static BossFile {
        self.world().boss.as_ref().expect("a boss on the map comes from its world")
    }
}

#[derive(Component, Default)]
pub struct Boss {
    pub phase: usize,
    /// Cells walked so far.
    pub cells: u32,
    /// Arbalète hits towards the next stun.
    pub hits: u32,
    pub planted: u32,
}

/// An imperial borne planted by the boss: shot down like an enemy, but it doesn't walk.
#[derive(Component)]
pub struct Borne;

/// The map owns the bornes: one gone from it (Nève scraped it off) breaks into shards like a shot one.
pub(super) fn drop_bornes(
    level: Res<Level>,
    bornes: Query<(Entity, &Enemy, &Transform), With<Borne>>,
    look: Res<EnemyLook>,
    mut commands: Commands,
) {
    for (entity, enemy, transform) in &bornes {
        if !level.terrain.base().bornes.contains(&enemy.cell) {
            shatter(&mut commands, &look, transform.translation);
            commands.entity(entity).try_despawn();
        }
    }
}

/// The boss moves on to its next phase as its HP drops; [P] a new phase can mean a new species profile.
pub fn boss_phases(mut level: ResMut<Level>, mut bosses: Query<(&mut Enemy, &mut Boss)>) {
    let Some(boss_file) = level.world().boss.as_ref() else { return };
    let phases = &boss_file.phases;
    for (mut enemy, mut boss) in &mut bosses {
        while boss.phase + 1 < phases.len()
            && enemy.hp > 0.0
            && enemy.hp <= enemy.max_hp * phases[boss.phase].until
        {
            boss.phase += 1;
            let phase = &phases[boss.phase];
            enemy.species = index_of(&level.species, &phase.species)
                .unwrap_or_else(|| panic!("boss: unknown species {:?}", phase.species));
            enemy.next = None;
            level.boss = Some(phase.name.clone());
            level.note(Trigger::Boss(phase.name.clone()));
        }
    }
}
