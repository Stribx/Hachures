//! Towers: their stats, their models, and the shots they fire.

use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::*;
use serde::Deserialize;

use super::boss::{Borne, Boss};
use super::figure::{EnemyLook, Kind, leave};
use super::shards::shatter;
use super::{Enemy, Shots};
use crate::campaign::skills::Skill;
use crate::ink::{TowerBiffed, TowerInked};
use crate::level::story::Trigger;
use crate::level::{Level, LevelEntity};
use crate::look::Carved;
use crate::map::grid::Grid;
use crate::map::land::Land;
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

/// The base under the tower on a cell.
#[derive(Component)]
pub struct TowerBase(pub usize);

/// A tower's base model; it stands 0.6 high, and the weapon sits on top.
pub const TOWER_BASE: &str = "models/tower-round-bottom-a.glb";

/// Carved for the plan-relief: a base of plain limewood, the weapon in walnut with its parts still told apart.
pub const BASE_WOOD: Carved = Carved([214, 186, 140], false);
pub const WEAPON_WOOD: Carved = Carved([196, 150, 110], true);

/// The glTF asset path of the weapon's model.
pub fn weapon_model(weapon: Weapon) -> &'static str {
    match weapon {
        Weapon::Arbalete => "models/weapon-ballista.glb",
        Weapon::Mortier => "models/weapon-cannon.glb",
        Weapon::Harpon => "models/weapon-catapult.glb",
        Weapon::Lanterne => "models/weapon-turret.glb",
    }
}

/// Stands up the towers just inked.
pub(super) fn raise_towers(
    mut inked: MessageReader<TowerInked>,
    level: Res<Level>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    for &TowerInked { cell, weapon } in inked.read() {
        spawn_tower(&mut commands, &assets, level.terrain.grid(), cell, weapon);
    }
}

/// Takes down the towers just biffed, base and weapon.
pub(super) fn take_down(
    mut biffed: MessageReader<TowerBiffed>,
    towers: Query<(Entity, &Tower)>,
    bases: Query<(Entity, &TowerBase)>,
    mut commands: Commands,
) {
    for &TowerBiffed { cell } in biffed.read() {
        let weapon = towers.iter().filter(|(_, t)| t.cell == cell).map(|(e, _)| e);
        let base = bases.iter().filter(|(_, b)| b.0 == cell).map(|(e, _)| e);
        for entity in weapon.chain(base) {
            commands.entity(entity).despawn();
        }
    }
}

/// The tower's base and its weapon, on top of the cell at `i`.
pub fn spawn_tower(commands: &mut Commands, assets: &AssetServer, grid: &Grid, i: usize, weapon: Weapon) {
    let (x, z) = grid.coords(i);
    let scene = |path: &'static str| WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(path)));
    // Both land in the cell like the other pieces; the base makes the sound of a wooden figure set down.
    let (base, top) = (Transform::from_xyz(x as f32, 0.2, z as f32), Transform::from_xyz(x as f32, 0.8, z as f32));
    commands.spawn((
        LevelEntity,
        TowerBase(i),
        scene(TOWER_BASE),
        BASE_WOOD,
        base,
        Land::new(base, Some("audio/sfx/impact/impactPlank_medium_000.ogg")),
    ));
    commands.spawn((
        LevelEntity,
        Tower { cell: i, weapon, ready_in: 0.0 },
        scene(weapon_model(weapon)),
        WEAPON_WOOD,
        top,
        Land::new(top, None),
    ));
}

/// What a tower can hit: enemies, the boss, its bornes.
type Targets<'w, 's> =
    Query<'w, 's, (Entity, &'static mut Enemy, &'static Transform, Option<&'static mut Boss>, Has<Borne>)>;

/// The hostile in `range` of `pos` that is closest to its goal, and where it stands.
fn aim(level: &Level, enemies: &Targets, pos: Vec3, range: f32) -> Option<(Entity, Vec3)> {
    enemies
        .iter()
        .filter(|(_, e, t, _, _)| level.species[e.species].hostile && t.translation.xz().distance(pos.xz()) <= range)
        .min_by(|(_, a, ..), (_, b, ..)| {
            level.terrain.fields()[a.species].dist[a.cell].total_cmp(&level.terrain.fields()[b.species].dist[b.cell])
        })
        .map(|(entity, _, t, _, _)| (entity, t.translation))
}

/// A shot of `weapon` lands on `enemy`: its damage, the pin and slow it brings, and the boss's stun.
fn strike(level: &Level, weapon: Weapon, stats: &TowerStats, enemy: &mut Enemy, mut boss: Option<&mut Boss>) {
    let species = &level.species[enemy.species];
    let weak = if species.weak_to == Some(weapon) { WEAKNESS } else { 1.0 };
    let counter_sight = boss.is_some() && weapon == Weapon::Mortier && level.has(Skill::ContreVisee);
    enemy.hp -= stats.damage * weak * if counter_sight { 1.25 } else { 1.0 };
    let phase = boss.as_ref().map(|b| &level.boss_file().phases[b.phase]);
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
            leave(commands, look, enemy.species, transform, Kind::Fall);
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
        let Some((target, target_pos)) = aim(level, &enemies, pos, range) else { continue };

        // A mortar shell hits everyone in the splash, Fuyards included.
        for (entity, mut enemy, t, mut boss, _) in &mut enemies {
            let hit =
                if stats.splash > 0.0 { t.translation.distance(target_pos) <= stats.splash } else { entity == target };
            if hit {
                fuyard_hurt |= !level.species[enemy.species].hostile;
                strike(level, tower.weapon, stats, &mut enemy, boss.as_deref_mut());
            }
        }
        tower.ready_in = stats.cooldown;
        shots.0.push((pos, target_pos, 0.15));
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
