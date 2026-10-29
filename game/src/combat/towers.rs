//! Towers: their stats, and the shots they fire.

use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::*;
use serde::Deserialize;

use super::Enemy;
use super::boss::{Borne, Boss};
use super::draw::{Shot, Shots};
use super::figure::{EnemyLook, ExitKind, leave};
use super::shards::shatter;
use super::stand::Tower;
use crate::campaign::skills::Skill;
use crate::level::Level;
use crate::level::story::Trigger;
use crate::map::species::Species;
use crate::map::terrain::Edit;

/// Damage multiplier against a species weak to the weapon.
const WEAKNESS: f32 = 2.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum Weapon {
    Arbalete,
    Mortier,
    Harpon,
    /// Blinds and slows (world 2).
    Lanterne,
    /// Strikes under the ground (world 3): the only weapon that reaches what digs.
    Sonde,
}

impl Weapon {
    /// Every weapon, in the order the bar offers them.
    pub const ALL: [Weapon; 5] =
        [Weapon::Arbalete, Weapon::Mortier, Weapon::Harpon, Weapon::Lanterne, Weapon::Sonde];
}

/// A tower's combat stats (`assets/data/towers.ron`).
#[derive(Deserialize)]
pub struct TowerStats {
    pub damage: f32,
    /// Tiles; also the reach of its shadow for the species reading l'Ombre.
    pub range: f32,
    pub cooldown: f32,
    pub splash: f32,
    pub pin: f32,
    /// Seconds the target walks slowed.
    #[serde(default)]
    pub slow: f32,
    /// Whether it reaches a species walking under the ground.
    #[serde(default)]
    pub underground: bool,
}

impl TowerStats {
    /// Whether it can hit `species` at all: what walks under the ground only the Sonde reaches.
    fn reaches(&self, species: &Species) -> bool {
        self.underground || !species.underground
    }
}

pub fn load_stats() -> HashMap<Weapon, TowerStats> {
    ron::from_str(include_str!("../../assets/data/towers.ron")).expect("invalid towers.ron")
}

/// What a tower can hit: enemies, the boss, its bornes.
type Targets<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static mut Enemy, &'static Transform, Option<&'static mut Boss>, Has<Borne>),
>;

/// The hostile in `range` of `pos` that is closest to its goal, and where it stands.
fn aim(
    level: &Level,
    enemies: &Targets,
    stats: &TowerStats,
    pos: Vec3,
    range: f32,
) -> Option<(Entity, Vec3)> {
    enemies
        .iter()
        .filter(|(_, e, t, _, _)| {
            let species = &level.species[e.species];
            species.hostile
                && stats.reaches(species)
                && t.translation.xz().distance(pos.xz()) <= range
        })
        .min_by(|(_, a, ..), (_, b, ..)| {
            level.terrain.fields()[a.species].dist[a.cell]
                .total_cmp(&level.terrain.fields()[b.species].dist[b.cell])
        })
        .map(|(entity, _, t, _, _)| (entity, t.translation))
}

/// A shot of `weapon` lands on `enemy`: its damage, the pin and slow it brings, and the boss's
/// stun.
fn strike(
    level: &Level,
    weapon: Weapon,
    stats: &TowerStats,
    enemy: &mut Enemy,
    mut boss: Option<&mut Boss>,
) {
    let species = &level.species[enemy.species];
    // Armour holds against everything but the weapon the species is weak to.
    let weak = if species.weak_to == Some(weapon) { WEAKNESS } else { 1.0 - species.armor };
    let counter_sight =
        boss.is_some() && weapon == Weapon::Mortier && level.has(Skill::ContreVisee);
    let phase = boss.as_ref().map(|b| &level.boss_file().phases[b.phase]);
    // La fêlure: the Lanterne's light shows where to hit.
    let crack = if phase.is_some_and(|p| p.cracked) && enemy.slowed > 0.0 { 2.0 } else { 1.0 };
    enemy.hp -= stats.damage * weak * crack * if counter_sight { 1.25 } else { 1.0 };
    if phase.is_none_or(|p| p.pinnable) {
        enemy.pinned = enemy.pinned.max(stats.pin);
    }
    enemy.slowed = enemy.slowed.max(stats.slow);
    // Le Levé: arbalète bolts on the sighting lantern stun the boss.
    if let (Some(boss), Some(phase)) = (boss.as_mut(), phase)
        && weapon == Weapon::Arbalete
        && phase.stun_hits > 0
    {
        boss.hits += 1;
        if boss.hits >= phase.stun_hits {
            boss.hits = 0;
            enemy.pinned = enemy.pinned.max(phase.stun);
        }
    }
}

/// Breaks the enemies with no HP left into shards, and pays out what they leave.
fn defeat(level: &mut Level, enemies: &Targets, look: &EnemyLook, commands: &mut Commands) {
    let mut bornes_down = Vec::new();
    for (entity, enemy, transform, boss, borne) in enemies {
        if enemy.hp > 0.0 {
            continue;
        }
        if borne {
            shatter(commands, look, transform.translation);
            bornes_down.push((enemy.cell, Edit::DropBorne));
        } else {
            leave(commands, look, enemy.species, transform, ExitKind::Fall);
            level.loot(level.species[enemy.species].loot);
        }
        if boss.is_some() {
            level.boss = None;
        }
        commands.entity(entity).try_despawn();
    }
    if !bornes_down.is_empty() {
        level.alter(bornes_down);
    }
}

pub(super) fn fire(
    time: Res<Time>,
    mut level: ResMut<Level>,
    mut shots: ResMut<Shots>,
    mut towers: Query<(&mut Tower, &mut Transform), Without<Enemy>>,
    mut enemies: Targets,
    look: Res<EnemyLook>,
    mut commands: Commands,
) {
    let level = &mut *level;
    let mut fuyard_hurt = false;
    for (mut tower, mut tower_transform) in &mut towers {
        tower.ready_in -= time.delta_secs();
        if tower.ready_in > 0.0 {
            continue;
        }
        let stats = &level.tower_stats[&tower.weapon];
        let range = level.terrain.reach(tower.cell, tower.weapon);
        let pos = tower_transform.translation;
        // Aim at the hostile in range that is closest to its goal.
        let Some((target, target_pos)) = aim(level, &enemies, stats, pos, range) else { continue };

        // A mortar shell hits everyone in the splash, Fuyards included.
        for (entity, mut enemy, t, mut boss, _) in &mut enemies {
            let hit = if stats.splash > 0.0 {
                t.translation.distance(target_pos) <= stats.splash
                    && stats.reaches(&level.species[enemy.species])
            } else {
                entity == target
            };
            if hit {
                fuyard_hurt |= !level.species[enemy.species].hostile;
                strike(level, tower.weapon, stats, &mut enemy, boss.as_deref_mut());
            }
        }
        tower.ready_in = stats.cooldown;
        shots.0.push(Shot { from: pos, to: target_pos, left: 0.15 });
        // Kenney weapons face +Z, look_at points -Z.
        tower_transform.look_at(Vec3::new(target_pos.x, pos.y, target_pos.z), Vec3::Y);
        tower_transform.rotate_local_y(PI);
    }

    defeat(level, &enemies, &look, &mut commands);
    if fuyard_hurt {
        level.fuyard_hurt = true;
        level.note(Trigger::FuyardHurt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Weapon::ALL` lists every weapon the stats table knows: a new weapon forgotten there fails here.
    #[test]
    fn all_lists_every_weapon_with_stats() {
        let stats = load_stats();
        assert_eq!(Weapon::ALL.len(), stats.len());
        assert!(Weapon::ALL.iter().all(|w| stats.contains_key(w)));
    }
}
