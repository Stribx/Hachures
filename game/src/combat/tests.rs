use std::f32::consts::{PI, TAU};
use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

use super::boss::{Borne, Boss, boss_phases, drop_bornes};
use super::enemies::{spawn_enemies, walk};
use super::figure::chain::{Link, drag_chain};
use super::figure::exit::Exit;
use super::figure::kit::BOSS_REAR;
use super::figure::{EnemyLook, Kind, Stride, animate, exits, spawn_figure};
use super::shards::Shard;
use super::towers::{Tower, Weapon, fire};
use super::{Enemy, Shots};
use crate::campaign::save::Save;
use crate::ink::TOOLS;
use crate::level::Level;
use crate::level::data::{LEVELS, START_LIVES};
use crate::map::cell_pos;
use crate::map::grid::{Grid, Tile};
use crate::map::species::{self, index_of};
use crate::map::terrain::Edit;

/// The balance bot's last blessed table, from the crate root.
const BALANCE: &str = "tests/balance.txt";

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
    let cell = level.terrain.grid().spawns[0];
    (Enemy::new(species, hp, cell), Transform::from_translation(cell_pos(level.terrain.grid(), cell)))
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
    let tower_cell = level.terrain.grid().width + 3;
    let tower_at = cell_pos(level.terrain.grid(), tower_cell);
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
    let tower_at = cell_pos(level.terrain.grid(), 1);
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
    level.alter([(1, Edit::PlantBorne), (3, Edit::PlantBorne)]);
    let jalonneur = index_of(&level.species, "jalonneur").unwrap();
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<EnemyLook>()
        .insert_resource(level)
        .add_systems(Update, drop_bornes);
    for cell in [1, 3] {
        app.world_mut().spawn((Borne, Enemy::new(jalonneur, 50.0, cell), Transform::default()));
    }
    app.world_mut().resource_mut::<Level>().alter([(1, Edit::DropBorne)]);
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
    assert!(level.terrain.base().tiles.contains(&Tile::Chaussee), "Le Levé lays Chaussée");
    assert!(level.terrain.base().bornes.is_empty(), "{:?}", level.terrain.base().bornes);

    set_hp(&mut app, 500.0);
    run(&mut app, 16);
    let level = app.world().resource::<Level>();
    assert_eq!(level.boss.as_deref(), Some("boss.grande_chaine.bornage"));
    assert_eq!(level.terrain.base().bornes.len(), 3, "Le Bornage plants three bornes");
    let bornes = app.world_mut().query_filtered::<(), With<Borne>>().iter(app.world()).count();
    assert_eq!(bornes, 3);

    set_hp(&mut app, 200.0);
    run(&mut app, 1);
    let level = app.world().resource::<Level>();
    let rompue = index_of(&level.species, "grande_chaine_rompue").unwrap();
    assert_eq!(level.boss.as_deref(), Some("boss.grande_chaine.chaine_rompue"));
    assert_eq!(app.world().get::<Enemy>(boss).unwrap().species, rompue, "la Chaîne rompue reads l'Ombre");
}

/// The boss drags twenty links along the road its cart drove, and they go with it.
#[test]
fn the_chain_follows_and_goes_with_the_boss() {
    let grid = Grid::parse(
        "
        S...................M
        .....................
        ",
    );
    let level = Level::new(grid, species::load());
    let boss = walker(&level, "grande_chaine", 1000.0);
    let mut app = app_with(level);
    app.add_systems(Update, drag_chain);
    let boss = app.world_mut().spawn((boss, Boss::default())).id();

    run(&mut app, 2);
    let links: Vec<Vec3> =
        app.world_mut().query::<(&Link, &Transform)>().iter(app.world()).map(|(_, t)| t.translation).collect();
    assert_eq!(links.len(), 20);
    let boss_at = app.world().get::<Transform>(boss).unwrap().translation;
    let rear = boss_at - Vec3::X * BOSS_REAR;
    let nearest = links.iter().map(|l| l.xz().distance(rear.xz())).fold(f32::MAX, f32::min);
    assert!(nearest < 0.3, "the first link is {nearest} from the rear");

    app.world_mut().despawn(boss);
    app.update();
    assert_eq!(app.world_mut().query::<&Link>().iter(app.world()).count(), 0, "the chain goes with the boss");
}

/// A naive player, to balance the levels without playing them: before each wave it spends
/// all its sépia on towers, where they cover the most of the hostile routes per sépia.
/// It never inks terrain, so a player who reads the map should do better.
/// It is deterministic, so its table is kept in `BALANCE`: a rule or data change that moves it fails here.
/// Run: `cargo test -p game balance -- --ignored --nocapture`; `HACHURES_BLESS=1` writes the new table.
#[test]
#[ignore]
fn balance() {
    let mut table =
        format!("{:<32} {:>5} {:>5} {:>5} {:>7} {:>6}\n", "level", "won", "Rôle", "Noms", "towers", "sépia");
    for index in 0..LEVELS.len() {
        let mut save = Save::default();
        save.choices.insert("refuge".into(), "moulin".into());
        let level = Level::load(index, &save);
        let waves = level.waves.len();
        let mut app = app_with(level);
        app.add_systems(Update, spawn_enemies.before(walk));

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
        table += &format!(
            "{:<32} {:>5} {:>5} {:>5} {:>7} {:>6}\n    Rôle lost (wave, lost): {losses:?}\n",
            level.name, won, level.lives, level.names, towers, level.sepia
        );
    }
    println!("\n{table}");
    let path = format!("{}/{BALANCE}", env!("CARGO_MANIFEST_DIR"));
    if std::env::var_os("HACHURES_BLESS").is_some() {
        std::fs::write(&path, &table).unwrap();
    }
    let kept = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        table == kept,
        "the balance moved: diff the table above with {BALANCE}, then bless it with HACHURES_BLESS=1"
    );
}

/// The bot's turn between waves: towers until nothing useful is affordable. Returns how many.
fn bot_build(app: &mut App) -> usize {
    let mut built = 0;
    loop {
        let mut level = app.world_mut().resource_mut::<Level>();
        let mut route: Vec<usize> = (0..level.species.len())
            .filter(|&k| level.species[k].hostile && level.waves.iter().flatten().any(|&s| s == k))
            .flat_map(|k| {
                let spawns = level.terrain.grid().spawns_for(&level.species[k]);
                spawns
                    .iter()
                    .flat_map(|&sp| level.terrain.fields()[k].path(level.terrain.grid(), sp))
                    .collect::<Vec<_>>()
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
            for cell in 0..level.terrain.grid().tiles.len() {
                let covered = route.iter().filter(|&&c| level.terrain.grid().within(c, cell, stats.range)).count();
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
        let (x, z) = level.terrain.grid().coords(cell);
        app.world_mut().spawn((Tower { cell, weapon, ready_in: 0.0 }, Transform::from_xyz(x as f32, 0.8, z as f32)));
        built += 1;
    }
}

/// A Chaîneur on the first spawn, standing as `spawn_enemies` makes one: a figure, an enemy and a stride.
fn striding(app: &mut App, species: &str) -> Entity {
    let level = app.world().resource::<Level>();
    let (enemy, at) = walker(level, species, 100.0);
    let index = enemy.species;
    let id = {
        let mut commands = app.world_mut().commands();
        let look = EnemyLook::default();
        let id = spawn_figure(&mut commands, &look, index, at.translation, 1.0);
        commands.entity(id).insert((enemy, Stride::default()));
        id
    };
    app.world_mut().flush();
    id
}

/// `animate` runs after `walk`, as in the game.
fn with_animation(app: &mut App) {
    app.add_systems(Update, animate.after(walk));
}

/// The step follows the ground walked, and a pinned walker does not step.
#[test]
fn a_stride_follows_the_ground_walked() {
    let level = Level::new(Grid::parse("S.........M"), species::load());
    let mut app = app_with(level);
    with_animation(&mut app);
    let id = striding(&mut app, "chaineur");
    let start = app.world().get::<Transform>(id).unwrap().translation;
    run(&mut app, 2);
    let at = app.world().get::<Transform>(id).unwrap().translation;
    let walked = app.world().get::<Stride>(id).unwrap().walked;
    let moved = at.distance(start);
    assert!(moved > 0.5, "it walked");
    assert!((walked - moved).abs() < 0.1 * moved, "walked {walked} for {moved} moved");

    app.world_mut().get_mut::<Enemy>(id).unwrap().pinned = 1.0;
    // One more update to take the pin's first frame, then the pin holds.
    let before = app.world().get::<Stride>(id).unwrap().walked;
    (0..15).for_each(|_| app.update());
    assert_eq!(app.world().get::<Stride>(id).unwrap().walked, before, "pinned, it does not step");
}

/// A turn eases toward the heading and never passes it.
#[test]
fn turns_ease_without_overshoot() {
    let level = Level::new(Grid::parse("S...\n...M\n...."), species::load());
    let mut app = app_with(level);
    with_animation(&mut app);
    let id = striding(&mut app, "chaineur");
    // The heading is fixed while it walks to one cell, so the yaw's gap to it must only shrink.
    let (mut gap, mut cell, mut checked) = (f32::MAX, usize::MAX, 0);
    for _ in 0..200 {
        app.update();
        let Some(enemy) = app.world().get::<Enemy>(id) else { break };
        let Some(next) = enemy.next else { continue };
        if enemy.cell != cell {
            (gap, cell) = (f32::MAX, enemy.cell);
        }
        let level = app.world().resource::<Level>();
        let transform = app.world().get::<Transform>(id).unwrap();
        let to = cell_pos(level.terrain.grid(), next) - transform.translation;
        if to.xz().length() < 0.05 {
            continue;
        }
        let heading = (-to.z).atan2(to.x);
        let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
        let now = (heading - yaw + PI).rem_euclid(TAU) - PI;
        assert!(now.abs() <= gap + 1e-4, "the yaw moved away from its heading: {now} after {gap}");
        gap = now.abs();
        checked += 1;
    }
    assert!(checked > 10, "the walker was followed");
}

/// The kinds of the ghosts standing, and how many shards fly.
fn ghosts_and_shards(app: &mut App) -> (Vec<Kind>, usize) {
    let kinds = app.world_mut().query_filtered::<&Exit, Without<Enemy>>().iter(app.world()).map(|e| e.kind).collect();
    (kinds, app.world_mut().query::<&Shard>().iter(app.world()).count())
}

/// A defeated enemy goes on the frame it falls, as before; a ghost tips over, then breaks into shards.
#[test]
fn a_defeated_enemy_leaves_a_ghost_then_shards() {
    let mut level = Level::new(Grid::parse("S.....M\n...^..R"), species::load());
    level.sepia = 0;
    let tower_cell = level.terrain.grid().width + 1;
    let tower_at = cell_pos(level.terrain.grid(), tower_cell);
    let chaineur = walker(&level, "chaineur", 0.5);
    let mut app = app_with(level);
    app.add_systems(Update, exits);
    app.world_mut().spawn((
        Tower { cell: tower_cell, weapon: Weapon::Arbalete, ready_in: 0.0 },
        Transform::from_translation(tower_at),
    ));
    let id = app.world_mut().spawn(chaineur).id();
    app.update();

    assert!(app.world().get_entity(id).is_err(), "it is gone on the frame it is defeated");
    assert_eq!(ghosts_and_shards(&mut app), (vec![Kind::Fall], 0), "a ghost tips over first");
    run(&mut app, 1);
    let (ghosts, shards) = ghosts_and_shards(&mut app);
    assert!(ghosts.is_empty() && shards > 0, "then it breaks: {ghosts:?}, {shards} shards");
}

/// An enemy at its goal is gone on that frame, as before; a ghost sinks into the board, then goes.
#[test]
fn an_arrival_sinks_into_the_board() {
    let level = Level::new(Grid::parse("S.M"), species::load());
    let chaineur = walker(&level, "chaineur", 100.0);
    let mut app = app_with(level);
    app.add_systems(Update, exits);
    let id = app.world_mut().spawn(chaineur).id();
    for _ in 0..200 {
        app.update();
        if app.world().get_entity(id).is_err() {
            break;
        }
    }
    assert!(app.world().get_entity(id).is_err(), "it arrived");
    assert_eq!(ghosts_and_shards(&mut app), (vec![Kind::Sink], 0), "and sinks");
    run(&mut app, 1);
    assert_eq!(ghosts_and_shards(&mut app), (vec![], 0), "then nothing is left");
}

/// The first figure of a species is noted in the Lexique, once.
#[test]
fn a_new_species_is_noted_once() {
    let level = Level::new(Grid::parse("S...M"), species::load());
    let (first, second) = (walker(&level, "chaineur", 100.0), walker(&level, "chaineur", 100.0));
    let mut app = app_with(level);
    app.insert_resource(Save::default()).add_systems(Update, super::noted::notice);
    app.world_mut().spawn(first);
    app.update();
    let level = app.world().resource::<Level>();
    assert_eq!(level.margin.len(), 1);
    assert_eq!(level.margin[0].shown(), "Noté au Lexique : Chaîneur");
    assert!(app.world().resource::<Save>().seen.contains("species.chaineur"));
    app.world_mut().spawn(second);
    app.update();
    assert_eq!(app.world().resource::<Level>().margin.len(), 1, "already known");
}

/// A biffed tower comes down, base and weapon; the tower beside it stays.
#[test]
fn a_biffed_tower_comes_down() {
    use super::towers::{TowerBase, take_down};
    use crate::ink::TowerBiffed;
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_message::<TowerBiffed>().add_systems(Update, take_down);
    for cell in [3, 4] {
        app.world_mut().spawn(Tower { cell, weapon: Weapon::Arbalete, ready_in: 0.0 });
        app.world_mut().spawn(TowerBase(cell));
    }
    app.world_mut().write_message(TowerBiffed { cell: 3 });
    app.update();
    let world = app.world_mut();
    let towers: Vec<usize> = world.query::<&Tower>().iter(world).map(|t| t.cell).collect();
    let bases: Vec<usize> = world.query::<&TowerBase>().iter(world).map(|b| b.0).collect();
    assert_eq!((towers, bases), (vec![4], vec![4]));
}
