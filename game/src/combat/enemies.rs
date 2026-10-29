//! Enemies: sent wave by wave, walking their species' flow field to its goal.

use bevy::prelude::*;

use super::Enemy;
use super::boss::{Borne, Boss};
use super::figure::{EnemyLook, ExitKind, Stride, leave, spawn_figure};
use super::trail::{Trail, lay};
use crate::level::story::Trigger;
use crate::level::{Level, LevelEntity, Walker};
use crate::map;
use crate::map::species::Goal;
use crate::settings::Settings;

/// Speed factor of an enemy the Lanterne blinds.
const SLOW: f32 = 0.5;

/// ×1.12 HP per wave (docs, section 8).
const HP_PER_WAVE: f32 = 1.12;

/// Size of the silhouettes, so they read on the board.
pub const SCALE: f32 = 1.3;

/// The boss stands larger than the rest.
pub const BOSS_SCALE: f32 = 1.6 * SCALE;

/// Space sends the next wave of the level.
pub fn start_waves(
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
    settings: Res<Settings>,
    mut level: ResMut<Level>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        send_wave(&mut level, &mut commands, &assets, &settings);
    }
}

/// Sends the next wave, if the last one is over and the level isn't.
pub fn send_wave(
    level: &mut Level,
    commands: &mut Commands,
    assets: &AssetServer,
    settings: &Settings,
) {
    let more = (level.wave.number as usize) < level.waves.len();
    if !level.wave.running && more && level.lives > 0 && level.start_wave() {
        // ponytail: a thud stands in for the rumble of the flood until there is a real sound.
        commands.spawn((
            AudioPlayer::new(assets.load("audio/sfx/impact/impactSoft_heavy_000.ogg")),
            PlaybackSettings::DESPAWN.with_volume(settings.effects_volume()),
        ));
    }
}

/// Spawns the wave one enemy a second, and ends it once the map is clear. Waves start and end
/// on an empty map, so La Crue comes and goes with them without stranding anyone.
pub fn spawn_enemies(
    time: Res<Time>,
    mut level: ResMut<Level>,
    // Bornes left standing don't hold the wave open.
    enemies: Query<(), (With<Enemy>, Without<Borne>)>,
    look: Res<EnemyLook>,
    mut commands: Commands,
) {
    if level.wave.running && level.wave.queue.is_empty() && enemies.is_empty() {
        level.end_wave();
    }
    if level.wave.queue.is_empty() {
        return;
    }
    level.wave.spawn_in -= time.delta_secs();
    if level.wave.spawn_in > 0.0 {
        return;
    }
    let level = &mut *level;
    let Some(species) = level.wave.queue.pop() else { return };
    level.wave.spawn_in = 1.0;
    let spawns = level.terrain.grid().spawns_for(&level.species[species]);
    let cell = spawns[level.wave.spawned % spawns.len()];
    level.wave.spawned += 1;
    let first_phase = level.world().boss.as_ref().map(|b| &b.phases[0]);
    let boss = first_phase.filter(|p| level.species[species].name == p.species);
    // Docs, section 8: ×1.5 per world, ×1.12 per wave (not for the boss).
    let wave_scale =
        if boss.is_some() { 1.0 } else { HP_PER_WAVE.powi(level.wave.number as i32 - 1) };
    let hp = level.hp(level.species[species].hp) * wave_scale;
    let at = map::cell_pos(level.terrain.grid(), cell);
    let scale = if boss.is_some() { BOSS_SCALE } else { SCALE };
    let enemy = spawn_figure(&mut commands, &look, species, at, scale);
    let mut enemy = commands.entity(enemy);
    enemy.insert((LevelEntity, Enemy::new(species, hp, cell), Stride::default()));
    if let Some(first_phase) = boss {
        enemy.insert(Boss::default());
        level.boss = Some(first_phase.name.clone());
        level.note(Trigger::Boss(first_phase.name.clone()));
    }
}

/// An enemy of `species` stands at its goal: it costs Rôle, or keeps a Name if it is an ally.
fn reach_goal(level: &mut Level, species: usize, is_boss: bool, trail: &mut Trail) {
    let loss = if is_boss { level.boss_file().breach } else { 1 };
    match level.species[species].goal {
        // An ally (Fuyard, convoy) safe at its goal keeps a Name.
        Goal::Refuge | Goal::Sortie => {
            level.names += 1;
            trail.saved = true;
        }
        Goal::Cible | Goal::BorneMere => level.lives = level.lives.saturating_sub(loss),
    }
    if is_boss {
        level.boss = None;
    }
}

/// One step of an enemy towards its next cell, unless it is pinned; the boss lays its road and
/// bornes on arriving.
fn step(
    level: &Level,
    enemy: &mut Enemy,
    transform: &mut Transform,
    mut boss: Option<&mut Boss>,
    dt: f32,
    trail: &mut Trail,
) {
    let species = &level.species[enemy.species];
    let slow = if enemy.slowed > 0.0 { SLOW } else { 1.0 };
    enemy.slowed = (enemy.slowed - dt).max(0.0);
    if enemy.pinned > 0.0 {
        enemy.pinned -= dt;
    } else if let Some(next) = enemy.next {
        // Off a cell that turned impassable under it (the Dégel), it wades out at a plain pace.
        let cost = species.cost(level.terrain.grid().tiles[enemy.cell]);
        let step = species.speed * slow / if cost.is_finite() { cost } else { 1.0 } * dt;
        let to = map::cell_pos(level.terrain.grid(), next) - transform.translation;
        if to.length() <= step {
            transform.translation += to;
            let left = enemy.cell;
            enemy.cell = next;
            enemy.next = None;
            if let Some(boss) = boss.as_mut() {
                boss.cells += 1;
                let phase = &level.boss_file().phases[boss.phase];
                if phase.chaussee_every > 0 && boss.cells % phase.chaussee_every == 0 {
                    trail.chaussee.push(left);
                }
                if phase.freeze_every > 0 && boss.cells % phase.freeze_every == 0 {
                    trail.sights.push((left, next));
                }
                if phase.change_every > 0
                    && boss.cells % phase.change_every == 0
                    && let Some(tile) = phase.change
                {
                    trail.relief.push((left, tile));
                }
                if boss.planted < phase.bornes && boss.cells % 2 == 0 {
                    boss.planted += 1;
                    trail.bornes.push((left, enemy.species));
                }
            }
        } else {
            transform.translation += to.normalize() * step;
        }
    }
}

/// Enemies step along their species' flow field, slower on costly tiles.
/// The boss leaves Chaussée and bornes behind it, as its phase says.
pub fn walk(
    time: Res<Time>,
    look: Res<EnemyLook>,
    mut level: ResMut<Level>,
    mut enemies: Query<(Entity, &mut Enemy, &mut Transform, Option<&mut Boss>), Without<Borne>>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    let level = &mut *level;
    level.walkers.clear();
    let mut trail = Trail::default();
    for (entity, mut enemy, mut transform, mut boss) in &mut enemies {
        if enemy.next.is_none() {
            let field = &level.terrain.fields()[enemy.species];
            enemy.next = field.next_step(level.terrain.grid(), enemy.cell);
            if enemy.next.is_none() && field.dist[enemy.cell] == 0.0 {
                reach_goal(level, enemy.species, boss.is_some(), &mut trail);
                let goal = level.species[enemy.species].goal;
                leave(&mut commands, &look, enemy.species, &transform, ExitKind::arriving(goal));
                commands.entity(entity).try_despawn();
                continue;
            }
        }
        step(level, &mut enemy, &mut transform, boss.as_deref_mut(), dt, &mut trail);
        level.walkers.push(Walker { species: enemy.species, cell: enemy.cell });
        if let Some(next) = enemy.next {
            level.walkers.push(Walker { species: enemy.species, cell: next });
        }
    }
    lay(level, &look, &mut commands, trail);
}
