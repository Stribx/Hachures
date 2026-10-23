//! Enemies: sent wave by wave, walking their species' flow field to its goal.

use bevy::prelude::*;

use super::Enemy;
use super::boss::{Borne, Boss};
use crate::campaign::save::Save;
use crate::level::story::Trigger;
use crate::level::{Level, LevelEntity};
use crate::map::cell_pos;
use crate::map::grid::Tile;
use crate::map::species::Goal;

/// Speed factor of an enemy the Lanterne blinds.
const SLOW: f32 = 0.5;

/// ×1.12 HP per wave (docs, section 8).
const HP_PER_WAVE: f32 = 1.12;

#[derive(Resource, Default)]
pub struct EnemyLook {
    pub mesh: Handle<Mesh>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub borne_mesh: Handle<Mesh>,
    pub borne_material: Handle<StandardMaterial>,
}

pub fn setup(
    mut commands: Commands,
    level: Res<Level>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(EnemyLook {
        mesh: meshes.add(Sphere::new(0.2)),
        materials: level.species.iter().map(|s| materials.add(Color::srgb(s.color.0, s.color.1, s.color.2))).collect(),
        borne_mesh: meshes.add(Cylinder::new(0.1, 0.6)),
        borne_material: materials.add(Color::srgb(0.85, 0.82, 0.7)),
    });
}

/// Space sends the next wave of the level.
pub fn start_waves(
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
    save: Res<Save>,
    mut level: ResMut<Level>,
    mut commands: Commands,
) {
    let more = (level.wave.number as usize) < level.waves.len();
    if keys.just_pressed(KeyCode::Space) && !level.wave.running && more && level.lives > 0 && level.start_wave() {
        // ponytail: a thud stands in for the rumble of the flood until there is a real sound.
        commands.spawn((
            AudioPlayer::new(assets.load("audio/sfx/impact/impactSoft_heavy_000.ogg")),
            PlaybackSettings::DESPAWN.with_volume(save.settings.effects_volume()),
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
    level.wave.spawn_in = 1.0;
    let species = level.wave.queue.pop().unwrap();
    let spawns = level.grid.spawns_for(&level.species[species]);
    let cell = spawns[level.wave.spawned % spawns.len()];
    level.wave.spawned += 1;
    let first_phase = level.world().boss.as_ref().map(|b| &b.phases[0]);
    let boss = first_phase.filter(|p| level.species[species].name == p.species);
    // Docs, section 8: ×1.5 per world, ×1.12 per wave (not for the boss).
    let wave_scale = if boss.is_some() { 1.0 } else { HP_PER_WAVE.powi(level.wave.number as i32 - 1) };
    let hp = level.hp(level.species[species].hp) * wave_scale;
    let at = cell_pos(&level.grid, cell);
    let mut enemy = commands.spawn((
        LevelEntity,
        Enemy::new(species, hp, cell),
        Mesh3d(look.mesh.clone()),
        MeshMaterial3d(look.materials[species].clone()),
        Transform::from_translation(at),
    ));
    if let Some(first_phase) = boss {
        enemy.insert((Boss::default(), Transform::from_translation(at).with_scale(Vec3::splat(1.6))));
        level.boss = Some(first_phase.name.clone());
        level.note(Trigger::Boss(first_phase.name.clone()));
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
    let mut chaussee = Vec::new();
    // (cell, species of the boss planting it)
    let mut bornes = Vec::new();
    let mut saved = false;
    for (entity, mut enemy, mut transform, mut boss) in &mut enemies {
        let species = &level.species[enemy.species];
        let field = &level.fields[enemy.species];
        if enemy.next.is_none() {
            enemy.next = field.next(&level.grid, enemy.cell);
            if enemy.next.is_none() && field.dist[enemy.cell] == 0.0 {
                let loss = if boss.is_some() { level.boss_file().breach } else { 1 };
                match species.goal {
                    // An ally (Fuyard, convoy) safe at its goal keeps a Name.
                    Goal::Refuge | Goal::Sortie => {
                        level.names += 1;
                        saved = true;
                    }
                    Goal::Cible | Goal::BorneMere => level.lives = level.lives.saturating_sub(loss),
                }
                if boss.is_some() {
                    level.boss = None;
                }
                commands.entity(entity).try_despawn();
                continue;
            }
        }
        let slow = if enemy.slowed > 0.0 { SLOW } else { 1.0 };
        enemy.slowed = (enemy.slowed - dt).max(0.0);
        if enemy.pinned > 0.0 {
            enemy.pinned -= dt;
        } else if let Some(next) = enemy.next {
            let step = species.speed * slow / species.cost(level.grid.tiles[enemy.cell]) * dt;
            let to = cell_pos(&level.grid, next) - transform.translation;
            if to.length() <= step {
                transform.translation += to;
                let left = enemy.cell;
                enemy.cell = next;
                enemy.next = None;
                if let Some(boss) = boss.as_mut() {
                    boss.cells += 1;
                    let phase = &level.boss_file().phases[boss.phase];
                    if phase.chaussee_every > 0 && boss.cells % phase.chaussee_every == 0 {
                        chaussee.push(left);
                    }
                    if boss.planted < phase.bornes && boss.cells % 2 == 0 {
                        boss.planted += 1;
                        bornes.push((left, enemy.species));
                    }
                }
            } else {
                transform.translation += to.normalize() * step;
            }
        }
        level.walkers.push((enemy.species, enemy.cell));
        if let Some(next) = enemy.next {
            level.walkers.push((enemy.species, next));
        }
    }

    // [T] Le Levé: the imperial road.
    for &cell in &chaussee {
        if level.base.tiles[cell].buildable() {
            level.base.tiles[cell] = Tile::Chaussee;
        }
    }
    // [S] Le Bornage: every borne is one more target for the Jalonneurs.
    for &(cell, species) in &bornes {
        level.base.bornes.push(cell);
        commands.spawn((
            LevelEntity,
            Borne,
            Enemy::new(species, level.hp(level.boss_file().borne_hp), cell),
            Mesh3d(look.borne_mesh.clone()),
            MeshMaterial3d(look.borne_material.clone()),
            Transform::from_translation(cell_pos(&level.grid, cell)),
        ));
    }
    if !chaussee.is_empty() || !bornes.is_empty() {
        level.refresh();
    }
    if saved {
        level.note(Trigger::FuyardSaved);
    }
}
