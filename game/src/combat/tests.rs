use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

use super::boss::{Borne, Boss, boss_phases, drop_bornes};
use super::enemies::{spawn_enemies, walk};
use super::shards::EnemyLook;
use super::towers::{Tower, Weapon, fire};
use super::{Enemy, Shots};
use crate::campaign::save::Save;
use crate::ink::TOOLS;
use crate::level::Level;
use crate::level::data::{LEVELS, START_LIVES};
use crate::map::cell_pos;
use crate::map::grid::{Grid, Tile};
use crate::map::species::{self, index_of};

/// Headless app running the combat systems on `level`, 50 ms per update.
fn app_with(level: Level) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(50)))
        .init_resource::<EnemyLook>()
        .init_resource::<Shots>()
        .insert_resource(level)
        .add_systems(Update, (walk, fire, boss_phases).chain());
    app
}

/// An enemy of `species` standing on the first spawn.
fn walker(level: &Level, species: &str, hp: f32) -> (Enemy, Transform) {
    let species = index_of(&level.species, species).unwrap();
    let cell = level.grid.spawns[0];
    (Enemy::new(species, hp, cell), Transform::from_translation(cell_pos(&level.grid, cell)))
}

fn run(app: &mut App, seconds: u32) {
    (0..seconds * 20).for_each(|_| app.update());
}

/// An arbalète beside the road kills a Chaîneur and lets a Fuyard through to the Refuge.
#[test]
fn arbalete_spares_fuyards() {
    let grid = Grid::parse(
        "
        S.....M
        ...^..R
        ",
    );
    let mut level = Level::new(grid, species::load());
    level.sepia = 0;
    let tower_cell = level.grid.width + 3;
    let tower_at = cell_pos(&level.grid, tower_cell);
    let (chaineur, fuyard) = (walker(&level, "chaineur", 100.0), walker(&level, "fuyard", 100.0));

    let mut app = app_with(level);
    app.world_mut().spawn((
        Tower { cell: tower_cell, weapon: Weapon::Arbalete, ready_in: 0.0 },
        Transform::from_translation(tower_at),
    ));
    app.world_mut().spawn(chaineur);
    app.world_mut().spawn(fuyard);
    run(&mut app, 10);

    let level = app.world().resource::<Level>();
    assert_eq!(level.lives, START_LIVES, "the Chaîneur never reached the Moulin");
    assert_eq!(level.sepia, 2, "its loot was collected");
    assert_eq!(level.names, 1, "the Fuyard reached the Refuge");
}

/// The Lanterne blinds the Gens du Col (double damage) and slows them.
#[test]
fn lanterne_slows() {
    let level = Level::new(Grid::parse("S.....M"), species::load());
    let gens = walker(&level, "gens_du_col", 100.0);
    let tower_at = cell_pos(&level.grid, 1);
    let mut app = app_with(level);
    app.world_mut()
        .spawn((Tower { cell: 1, weapon: Weapon::Lanterne, ready_in: 0.0 }, Transform::from_translation(tower_at)));
    let gens = app.world_mut().spawn(gens).id();
    app.update();
    let enemy = app.world().get::<Enemy>(gens).unwrap();
    assert_eq!(enemy.hp, 70.0, "15 damage, doubled against the Gens du Col");
    assert!(enemy.slowed > 0.0, "and it walks slowed");
}

/// The map owns the bornes: one taken off it (scraped) loses its piece, the other stays.
#[test]
fn a_borne_off_the_map_breaks() {
    let mut level = Level::new(Grid::parse("S...M"), species::load());
    level.base.bornes = vec![1, 3];
    let jalonneur = index_of(&level.species, "jalonneur").unwrap();
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<EnemyLook>()
        .insert_resource(level)
        .add_systems(Update, drop_bornes);
    for cell in [1, 3] {
        app.world_mut().spawn((Borne, Enemy::new(jalonneur, 50.0, cell), Transform::default()));
    }
    app.world_mut().resource_mut::<Level>().base.bornes.retain(|&b| b != 1);
    app.update();
    let left: Vec<usize> =
        app.world_mut().query_filtered::<&Enemy, With<Borne>>().iter(app.world()).map(|e| e.cell).collect();
    assert_eq!(left, vec![3]);
}

/// La Grande Chaîne lays Chaussée, then plants three bornes, then reads l'Ombre, as its HP drops.
#[test]
fn grande_chaine_phases() {
    let grid = Grid::parse(
        "
        S...................M
        .....................
        ",
    );
    let level = Level::new(grid, species::load());
    let boss = walker(&level, "grande_chaine", 1000.0);
    let mut app = app_with(level);
    let boss = app.world_mut().spawn((boss, Boss::default())).id();
    let set_hp = |app: &mut App, hp: f32| app.world_mut().get_mut::<Enemy>(boss).unwrap().hp = hp;

    run(&mut app, 10);
    let level = app.world().resource::<Level>();
    assert_eq!(level.boss, None, "phase names come from spawn_enemies and boss_phases");
    assert!(level.base.tiles.contains(&Tile::Chaussee), "Le Levé lays Chaussée");
    assert!(level.base.bornes.is_empty());

    set_hp(&mut app, 500.0);
    run(&mut app, 16);
    let level = app.world().resource::<Level>();
    assert_eq!(level.boss.as_deref(), Some("boss.grande_chaine.bornage"));
    assert_eq!(level.base.bornes.len(), 3, "Le Bornage plants three bornes");
    let bornes = app.world_mut().query_filtered::<(), With<Borne>>().iter(app.world()).count();
    assert_eq!(bornes, 3);

    set_hp(&mut app, 200.0);
    run(&mut app, 1);
    let level = app.world().resource::<Level>();
    let rompue = index_of(&level.species, "grande_chaine_rompue").unwrap();
    assert_eq!(level.boss.as_deref(), Some("boss.grande_chaine.chaine_rompue"));
    assert_eq!(app.world().get::<Enemy>(boss).unwrap().species, rompue, "la Chaîne rompue reads l'Ombre");
}

/// A naive player, to balance the levels without playing them: before each wave it spends
/// all its sépia on towers, where they cover the most of the hostile routes per sépia.
/// It never inks terrain, so a player who reads the map should do better.
/// Run: `cargo test --features dev balance -- --ignored --nocapture`
#[test]
#[ignore]
fn balance() {
    println!("\n{:<32} {:>5} {:>5} {:>5} {:>7} {:>6}", "level", "won", "Rôle", "Noms", "towers", "sépia");
    for index in 0..LEVELS.len() {
        let mut save = Save::default();
        save.choices.insert("refuge".into(), "moulin".into());
        let level = Level::load(index, &save);
        let waves = level.waves.len();
        let species = level.species.len();
        let mut app = app_with(level);
        app.add_systems(Update, spawn_enemies.before(walk));
        app.world_mut().resource_mut::<EnemyLook>().materials = vec![default(); species];

        let mut towers = 0;
        // (wave, Rôle lost)
        let mut losses = Vec::new();
        for _ in 0..waves {
            towers += bot_build(&mut app);
            let before = app.world().resource::<Level>().lives;
            app.world_mut().resource_mut::<Level>().start_wave();
            for _ in 0..600 {
                run(&mut app, 1);
                let level = app.world().resource::<Level>();
                if !level.wave.running || level.lives == 0 {
                    break;
                }
            }
            let level = app.world().resource::<Level>();
            if level.lives < before {
                losses.push((level.wave.number, before - level.lives));
            }
            if level.lives == 0 {
                break;
            }
        }
        let level = app.world().resource::<Level>();
        let won = if level.won() { "oui" } else { &format!("v{}", level.wave.number) };
        println!(
            "{:<32} {:>5} {:>5} {:>5} {:>7} {:>6}",
            level.name, won, level.lives, level.names, towers, level.sepia
        );
        println!("    Rôle lost (wave, lost): {losses:?}");
    }
}

/// The bot's turn between waves: towers until nothing useful is affordable. Returns how many.
fn bot_build(app: &mut App) -> usize {
    let mut built = 0;
    loop {
        let mut level = app.world_mut().resource_mut::<Level>();
        let mut route: Vec<usize> = (0..level.species.len())
            .filter(|&k| level.species[k].hostile && level.waves.iter().flatten().any(|&s| s == k))
            .flat_map(|k| {
                let spawns = level.grid.spawns_for(&level.species[k]);
                spawns.iter().flat_map(|&sp| level.fields[k].path(&level.grid, sp)).collect::<Vec<_>>()
            })
            .collect();
        route.sort_unstable();
        route.dedup();

        // (value per sépia, tool, cell), best first.
        let mut candidates = Vec::new();
        for t in level.tools.clone().into_iter().filter(|&t| TOOLS[t].act.weapon().is_some()) {
            let cost = level.tool_cost(t);
            if cost > level.sepia {
                continue;
            }
            let stats = &level.tower_stats[&TOOLS[t].act.weapon().unwrap()];
            let dps = stats.damage / stats.cooldown * if stats.splash > 0.0 { 2.0 } else { 1.0 };
            for cell in 0..level.grid.tiles.len() {
                let covered = route.iter().filter(|&&c| level.grid.within(c, cell, stats.range)).count();
                if covered > 0 {
                    candidates.push((covered as f32 * dps / cost as f32, t, cell));
                }
            }
        }
        candidates.sort_by(|a, b| b.0.total_cmp(&a.0));
        let Some(&(_, t, cell)) = candidates.iter().find(|&&(_, t, cell)| level.refusal(cell, t).is_none()) else {
            return built;
        };
        level.ink(cell, t);
        let weapon = TOOLS[t].act.weapon().unwrap();
        let (x, z) = level.grid.coords(cell);
        app.world_mut().spawn((Tower { cell, weapon, ready_in: 0.0 }, Transform::from_xyz(x as f32, 0.8, z as f32)));
        built += 1;
    }
}
