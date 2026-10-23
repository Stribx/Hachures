//! Towers: their stats, their models, and the shots they fire.

use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::*;
use serde::Deserialize;

use super::boss::{Borne, Boss};
use super::{Enemy, Shots};
use crate::campaign::skills::Skill;
use crate::level::story::Trigger;
use crate::level::{Level, LevelEntity};
use crate::map::grid::Grid;

/// Damage multiplier against a species weak to the weapon.
const WEAKNESS: f32 = 2.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum Weapon {
    Arbalete,
    Mortier,
    Harpon,
    /// Blinds and slows (world 2).
    Lanterne,
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
}

pub fn load_stats() -> HashMap<Weapon, TowerStats> {
    ron::from_str(include_str!("../../assets/data/towers.ron")).expect("invalid towers.ron")
}

#[derive(Component)]
pub struct Tower {
    pub cell: usize,
    pub weapon: Weapon,
    pub ready_in: f32,
}

impl Level {
    /// Reach of the tower at `cell`; Équerre adds 1 when another tower stands beside it.
    pub fn range(&self, cell: usize, weapon: Weapon) -> f32 {
        let paired =
            self.has(Skill::Equerre) && self.base.neighbours(cell).any(|n| self.towers.iter().any(|t| t.0 == n));
        self.tower_stats[&weapon].range + if paired { 1.0 } else { 0.0 }
    }
}

/// The tower's base and its weapon, on top of the cell at `i`.
pub fn spawn_tower(commands: &mut Commands, assets: &AssetServer, grid: &Grid, i: usize, weapon: Weapon) {
    let (x, z) = grid.coords(i);
    let model = match weapon {
        Weapon::Arbalete => "models/weapon-ballista.glb",
        Weapon::Mortier => "models/weapon-cannon.glb",
        Weapon::Harpon => "models/weapon-catapult.glb",
        Weapon::Lanterne => "models/weapon-turret.glb",
    };
    let scene = |path: &'static str| WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(path)));
    commands.spawn((
        LevelEntity,
        scene("models/tower-round-bottom-a.glb"),
        Transform::from_xyz(x as f32, 0.2, z as f32),
    ));
    commands.spawn((
        LevelEntity,
        Tower { cell: i, weapon, ready_in: 0.0 },
        scene(model),
        Transform::from_xyz(x as f32, 0.8, z as f32),
    ));
}

/// What a tower can hit: enemies, the boss, its bornes.
type Targets<'w, 's> =
    Query<'w, 's, (Entity, &'static mut Enemy, &'static Transform, Option<&'static mut Boss>, Has<Borne>)>;

pub(super) fn fire(
    time: Res<Time>,
    mut level: ResMut<Level>,
    mut shots: ResMut<Shots>,
    mut towers: Query<(&mut Tower, &mut Transform), Without<Enemy>>,
    mut enemies: Targets,
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
        let range = level.range(tower.cell, tower.weapon);
        let pos = tower_transform.translation;
        // Aim at the hostile in range that is closest to its goal.
        let Some((target, target_pos)) = enemies
            .iter()
            .filter(|(_, e, t, _, _)| {
                level.species[e.species].hostile && t.translation.xz().distance(pos.xz()) <= range
            })
            .min_by(|(_, a, ..), (_, b, ..)| {
                level.fields[a.species].dist[a.cell].total_cmp(&level.fields[b.species].dist[b.cell])
            })
            .map(|(entity, _, t, _, _)| (entity, t.translation))
        else {
            continue;
        };

        // A mortar shell hits everyone in the splash, Fuyards included.
        for (entity, mut enemy, t, mut boss, _) in &mut enemies {
            let hit =
                if stats.splash > 0.0 { t.translation.distance(target_pos) <= stats.splash } else { entity == target };
            if !hit {
                continue;
            }
            let species = &level.species[enemy.species];
            fuyard_hurt |= !species.hostile;
            let weak = if species.weak_to == Some(tower.weapon) { WEAKNESS } else { 1.0 };
            let counter_sight = boss.is_some() && tower.weapon == Weapon::Mortier && level.has(Skill::ContreVisee);
            enemy.hp -= stats.damage * weak * if counter_sight { 1.25 } else { 1.0 };
            let phase = boss.as_ref().map(|b| &level.boss_file().phases[b.phase]);
            if phase.is_none_or(|p| p.pinnable) {
                enemy.pinned = enemy.pinned.max(stats.pin);
            }
            enemy.slowed = enemy.slowed.max(stats.slow);
            // Le Levé: arbalète bolts on the sighting lantern stun the boss.
            if let (Some(boss), Some(phase)) = (boss.as_mut(), phase)
                && tower.weapon == Weapon::Arbalete
                && phase.stun_hits > 0
            {
                boss.hits += 1;
                if boss.hits >= phase.stun_hits {
                    boss.hits = 0;
                    enemy.pinned = enemy.pinned.max(phase.stun);
                }
            }
        }
        tower.ready_in = stats.cooldown;
        shots.0.push((pos, target_pos, 0.15));
        // Kenney weapons face +Z, look_at points -Z.
        tower_transform.look_at(Vec3::new(target_pos.x, pos.y, target_pos.z), Vec3::Y);
        tower_transform.rotate_local_y(PI);
    }

    let mut bornes_down = false;
    for (entity, enemy, _, boss, borne) in &enemies {
        if enemy.hp > 0.0 {
            continue;
        }
        if borne {
            level.base.bornes.retain(|&b| b != enemy.cell);
            bornes_down = true;
        } else {
            level.loot(level.species[enemy.species].loot);
        }
        if boss.is_some() {
            level.boss = None;
        }
        commands.entity(entity).try_despawn();
    }
    if bornes_down {
        level.refresh();
    }
    if fuyard_hurt {
        level.fuyard_hurt = true;
        level.note(Trigger::FuyardHurt);
    }
}
