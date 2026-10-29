//! The demo `HACHURES_DEMO`, `HACHURES_WAVE` and `HACHURES_CALQUE` set up: towers, barricades and a
//! wave sent.

use bevy::prelude::*;

use super::env;
use crate::campaign::skills::Skill;
use crate::combat::enemies::send_wave;
use crate::combat::stand::spawn_tower;
use crate::combat::towers::Weapon;
use crate::ink::Selection;
use crate::level::Level;
use crate::map::grid::Tile;
use crate::map::terrain::Edit;
use crate::settings::Settings;

/// How many waves have been sent when `HACHURES_WAVE=arg` asks for the next one to be wave `arg`.
fn waves_done(arg: &str, waves: usize) -> u32 {
    let n: u32 = arg.parse().unwrap_or(1);
    n.saturating_sub(1).min(waves.saturating_sub(1) as u32)
}

/// One tower of each weapon on the middle row's first open cells, each turned a different way.
pub(super) fn demo(
    mut done: Local<u32>,
    mut level: ResMut<Level>,
    mut selection: ResMut<Selection>,
    settings: Res<Settings>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    // Wait for the level asked for to be loaded.
    *done += 1;
    if *done != 3 {
        return;
    }
    if env("HACHURES_CALQUE").is_some() {
        selection.calque = true;
    }
    // `HACHURES_WAVE` sends the first wave (wave `n` for a number) with or without the demo's
    // towers.
    if let Some(arg) = env("HACHURES_WAVE") {
        level.wave.number = waves_done(&arg, level.waves.len());
        send_wave(&mut level, &mut commands, &assets, &settings);
    }
    if env("HACHURES_DEMO").is_none() {
        return;
    }
    let grid = level.terrain.grid();
    let open =
        (0..grid.tiles.len()).filter(|&i| grid.tiles[i] == Tile::Sol && grid.coords(i).1 == 2);
    let weapons =
        [Weapon::Arbalete, Weapon::Mortier, Weapon::Harpon, Weapon::Lanterne, Weapon::Sonde];
    for (i, weapon) in open.step_by(3).zip(weapons) {
        spawn_tower(&mut commands, &assets, grid, i, weapon);
    }
    // And barricades to see their props: a wall of three with a branch down (a T), and one alone
    // further on. `HACHURES_FASCINES=1` learns the skill that shows on them.
    let width = grid.width;
    let wall = |i: usize| [i, i + 1, i + 2, i + 1 + width, i + 5];
    let start = (0..grid.tiles.len()).find(|&i| {
        let (x, z) = grid.coords(i);
        z == 1 && x + 5 < width && wall(i).iter().all(|&j| grid.tiles[j] == Tile::Sol)
    });
    let walls: Vec<(usize, Edit)> =
        start.into_iter().flat_map(wall).map(|j| (j, Edit::Ink(Tile::Barricade))).collect();
    if env("HACHURES_FASCINES").is_some() {
        level.skills.push(Skill::Fascines);
    }
    // Even with no wall, the cells are drawn again: the skill shows on them.
    level.alter(walls);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wave_n_is_sent_after_n_minus_one() {
        assert_eq!(waves_done("1", 5), 0);
        assert_eq!(waves_done("yes", 5), 0);
        assert_eq!(waves_done("0", 5), 0);
        assert_eq!(waves_done("3", 5), 2);
        assert_eq!(waves_done("9", 5), 4, "the last wave at most");
    }
}
