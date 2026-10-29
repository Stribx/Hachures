//! Le Théodolite de Givre (docs, section 6, monde 2): the water it freezes, the col it seals, the
//! ice it thaws.

use bevy::prelude::*;

use super::boss::Boss;
use crate::i18n::n_;
use crate::level::Level;
use crate::level::story::MarginNote;
use crate::map::flow::can_place;
use crate::map::grid::{Grid, Tile};
use crate::map::terrain::Edit;

/// La Visée [T]: the open water on the line from `from` through `to`, on to the map's edge.
pub(super) fn sight_line(grid: &Grid, from: usize, to: usize) -> impl Iterator<Item = usize> + '_ {
    let ((fx, fz), (tx, tz)) = (grid.coords(from), grid.coords(to));
    let (dx, dz) = (tx as isize - fx as isize, tz as isize - fz as isize);
    let (w, h) = (grid.width as isize, grid.height as isize);
    (1..)
        .map(move |k| (tx as isize + dx * k, tz as isize + dz * k))
        .take_while(move |&(x, z)| (dx, dz) != (0, 0) && (0..w).contains(&x) && (0..h).contains(&z))
        .map(move |(x, z)| (z * w + x) as usize)
        .filter(move |&i| grid.tiles[i] == Tile::Eau)
}

impl Level {
    /// Le Dégel forcé [T]: the ice thaws back to water, cell by cell, but never so as to close the
    /// map (le Droit de Passage).
    pub(super) fn thaw_all(&mut self) {
        let grid = self.terrain.grid();
        let ice: Vec<usize> =
            (0..grid.tiles.len()).filter(|&i| grid.tiles[i] == Tile::Glace).collect();
        for i in ice {
            let grid = self.terrain.grid();
            if can_place(grid, &self.species, &self.present, i, Tile::Eau, &self.walkers) {
                self.alter([(i, Edit::Thaw)]);
            }
        }
    }
}

/// La Triangulation [S]: while the boss and every borne it planted stand, the col seals; held
/// for the phase's `seal` seconds, the level is lost.
pub(super) fn seal_col(time: Res<Time>, mut level: ResMut<Level>, mut bosses: Query<&mut Boss>) {
    let Some(boss_file) = level.world().boss.as_ref() else { return };
    for mut boss in &mut bosses {
        let phase = &boss_file.phases[boss.phase];
        let held = phase.seal > 0.0
            && boss.planted == phase.bornes
            && level.terrain.base().bornes.len() as u32 >= phase.bornes;
        boss.held = if held { boss.held + time.delta_secs() } else { 0.0 };
        if held && boss.held >= phase.seal && level.lives > 0 {
            level.lives = 0;
            let line = n_("note.col_sealed").to_string();
            level.margin.push(MarginNote { line, name: None });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::Enemy;
    use crate::combat::figure::chain::{Link, drag_chain};
    use crate::combat::stand::Tower;
    use crate::combat::tests::{app_with, run, walker};
    use crate::combat::towers::Weapon;
    use crate::map;
    use crate::map::species::index_of;

    /// A world-2 sheet, where the Théodolite is the boss.
    fn sheet(map: &str) -> Level {
        let mut level = Level::parse(map);
        level.world = 1;
        level
    }

    /// The Théodolite on the first spawn, in phase `phase`, run with the boss systems.
    fn theodolite(level: Level, phase: usize) -> (App, Entity) {
        let boss = walker(&level, "theodolite", 1000.0);
        let mut app = app_with(level);
        app.add_systems(Update, seal_col.after(crate::combat::boss::boss_phases));
        let boss = app.world_mut().spawn((boss, Boss { phase, ..default() })).id();
        (app, boss)
    }

    fn tiles(app: &App) -> Vec<Tile> {
        app.world().resource::<Level>().terrain.grid().tiles.clone()
    }

    #[test]
    fn the_sight_line_runs_through_the_water_ahead() {
        let grid = Grid::parse("S...~~.~\n........\n....~...");
        assert_eq!(sight_line(&grid, 0, 1).collect::<Vec<_>>(), vec![4, 5, 7]);
        assert_eq!(sight_line(&grid, 4, 12).collect::<Vec<_>>(), vec![20]);
        assert_eq!(sight_line(&grid, 7, 6).collect::<Vec<_>>(), vec![5, 4], "westward too");
        assert_eq!(sight_line(&grid, 3, 3).count(), 0, "no axis, no line");
    }

    #[test]
    fn visee_freezes_a_line_of_water() {
        let (mut app, _) = theodolite(sheet("S.......M~~\n..........."), 0);
        run(&mut app, 8);
        let tiles = tiles(&app);
        assert_eq!((tiles[9], tiles[10]), (Tile::Glace, Tile::Glace), "the water ahead freezes");
        let level = app.world().resource::<Level>();
        assert_eq!(level.terrain.base().map[9], Tile::Eau, "ice inked on the water drawn there");
    }

    #[test]
    fn theodolite_phases() {
        let (mut app, boss) = theodolite(sheet(&format!("S{}M", ".".repeat(40))), 0);
        app.world_mut().get_mut::<Enemy>(boss).unwrap().hp = 500.0;
        run(&mut app, 1);
        assert_eq!(
            app.world().resource::<Level>().boss.as_deref(),
            Some("boss.theodolite.triangulation")
        );
        app.world_mut().get_mut::<Enemy>(boss).unwrap().hp = 200.0;
        run(&mut app, 1);
        let level = app.world().resource::<Level>();
        assert_eq!(level.boss.as_deref(), Some("boss.theodolite.degel"));
        let enlise = index_of(&level.species, "theodolite_enlise").unwrap();
        assert_eq!(app.world().get::<Enemy>(boss).unwrap().species, enlise, "the tower sinks in");
    }

    #[test]
    fn masts_held_seal_the_col() {
        let (mut app, _) = theodolite(sheet(&format!("S{}M", ".".repeat(60))), 1);
        run(&mut app, 10);
        let level = app.world().resource::<Level>();
        assert_eq!(level.terrain.base().bornes.len(), 2, "two signal masts");
        assert!(level.lives > 0);
        run(&mut app, 40);
        let level = app.world().resource::<Level>();
        assert_eq!(level.lives, 0, "three points held: the col is sealed");
        assert_eq!(level.margin.last().unwrap().line, "note.col_sealed");
    }

    #[test]
    fn a_mast_down_stops_the_seal() {
        let (mut app, _) = theodolite(sheet(&format!("S{}M", ".".repeat(60))), 1);
        run(&mut app, 10);
        let mut level = app.world_mut().resource_mut::<Level>();
        let mast = level.terrain.base().bornes[0];
        level.alter([(mast, Edit::DropBorne)]);
        run(&mut app, 40);
        assert!(app.world().resource::<Level>().lives > 0);
    }

    /// Le Dégel: the Gel's ice and the Visée's alike turn back to water, except the one cell
    /// that would close the map.
    #[test]
    fn degel_turns_all_glace_to_water() {
        let mut level = sheet("S.........M\n~~~~~~~~~~~\n...^^~^^^^.");
        level.event_on = true;
        let event = level.world().event.as_ref();
        level.terrain.set_event(event, &level.species);
        level.alter([(16, Edit::Ink(Tile::Glace))]);
        level.thaw_all();
        let tiles = &level.terrain.grid().tiles;
        assert!((11..22).all(|i| tiles[i] == Tile::Eau), "{:?}", &tiles[11..22]);
        let mut level = sheet("S..~..M");
        level.event_on = true;
        let event = level.world().event.as_ref();
        level.terrain.set_event(event, &level.species);
        level.thaw_all();
        assert_eq!(level.terrain.grid().tiles[3], Tile::Glace, "the only way stays frozen");
    }

    #[test]
    fn a_walker_on_thawed_ice_steps_off() {
        let level = sheet("S...M\n~~~~~");
        let mut app = app_with(level);
        let chaineur = index_of(&app.world().resource::<Level>().species, "chaineur").unwrap();
        let at = map::cell_pos(app.world().resource::<Level>().terrain.grid(), 7);
        let walker = app
            .world_mut()
            .spawn((Enemy::new(chaineur, 100.0, 7), Transform::from_translation(at)))
            .id();
        app.world_mut().resource_mut::<Level>().alter([(7, Edit::Land(Tile::Eau))]);
        run(&mut app, 10);
        assert!(app.world().get_entity(walker).is_err(), "it waded out and reached the Cible");
    }

    #[test]
    fn lanterne_reveals_the_crack() {
        let hit = |phase: usize| {
            let level = sheet("S.....M");
            let tower_at = map::cell_pos(level.terrain.grid(), 1);
            let (mut app, boss) = theodolite(level, phase);
            app.world_mut().get_mut::<Enemy>(boss).unwrap().slowed = 1.0;
            app.world_mut().spawn((
                Tower { cell: 1, weapon: Weapon::Arbalete, ready_in: 0.0 },
                Transform::from_translation(tower_at),
            ));
            app.update();
            1000.0 - app.world().get::<Enemy>(boss).unwrap().hp
        };
        assert_eq!(hit(2), 2.0 * hit(1), "lit by the Lanterne, the cracked lens takes double");
    }

    #[test]
    fn only_the_grande_chaine_drags_a_chain() {
        let (mut app, _) = theodolite(sheet("S...................M"), 0);
        app.add_systems(Update, drag_chain);
        run(&mut app, 2);
        assert_eq!(app.world_mut().query::<&Link>().iter(app.world()).count(), 0);
    }
}
