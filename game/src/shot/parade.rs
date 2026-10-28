//! Dev tool: `HACHURES_PARADE=1` stands one still figure of every species in a row, for art review.

use bevy::prelude::*;

use super::env;
use crate::combat::enemies::{BOSS_SCALE, SCALE};
use crate::combat::figure::{EnemyLook, spawn_figure};
use crate::level::{Level, LevelEntity};
use crate::map::cell_pos;
use crate::map::grid::{Grid, Tile};

/// `n` distinct plain-Sol cells without a marker: a run of `n` in one row, the row nearest the middle first, else the
/// first `n` in reading order (fewer if the map has fewer).
pub fn parade_cells(grid: &Grid, n: usize) -> Vec<usize> {
    let free = |i: usize| {
        grid.tiles[i] == Tile::Sol
            && ![&grid.spawns, &grid.fuyard_spawns, &grid.bornes].iter().any(|marks| marks.contains(&i))
            && !grid.goals.values().any(|&g| g == i)
    };
    let mut rows: Vec<usize> = (0..grid.height).collect();
    rows.sort_by_key(|&z| (z.abs_diff(grid.height / 2), z));
    if n > 0 {
        for z in rows {
            let row: Vec<usize> = (z * grid.width..(z + 1) * grid.width).collect();
            if let Some(run) = row.windows(n).find(|run| run.iter().all(|&i| free(i))) {
                return run.to_vec();
            }
        }
    }
    (0..grid.tiles.len()).filter(|&i| free(i)).take(n).collect()
}

/// Once the level is loaded, one figure of each species in a row: no `Enemy`, so none walks, is shot at or holds a wave.
pub fn parade(mut done: Local<u32>, level: Res<Level>, look: Res<EnemyLook>, mut commands: Commands) {
    // Wait for the level asked for to be loaded, as `demo` does.
    *done += 1;
    if *done != 3 || env("HACHURES_PARADE").is_none() {
        return;
    }
    let grid = level.terrain.grid();
    let boss = level.world().boss.as_ref();
    let cells = parade_cells(grid, level.species.len());
    for (index, (species, cell)) in level.species.iter().zip(cells).enumerate() {
        let at = cell_pos(grid, cell);
        let is_boss = boss.is_some_and(|b| b.phases.iter().any(|p| p.species == species.name));
        let figure = spawn_figure(&mut commands, &look, index, at, if is_boss { BOSS_SCALE } else { SCALE });
        commands.entity(figure).insert(LevelEntity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str = "\
S..^....
.#....F.
..~..M..
.b......
........";

    fn is_free(grid: &Grid, i: usize) -> bool {
        grid.tiles[i] == Tile::Sol
            && !grid.spawns.contains(&i)
            && !grid.fuyard_spawns.contains(&i)
            && !grid.goals.values().any(|&g| g == i)
    }

    #[test]
    fn cells_are_distinct_free_sol() {
        let grid = Grid::parse(MAP);
        let cells = parade_cells(&grid, 5);
        assert_eq!(cells.len(), 5);
        let mut sorted = cells.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 5);
        assert!(cells.iter().all(|&i| is_free(&grid, i)));
    }

    #[test]
    fn cells_are_deterministic() {
        let grid = Grid::parse(MAP);
        assert_eq!(parade_cells(&grid, 5), parade_cells(&grid, 5));
    }

    #[test]
    fn cells_run_along_the_row_nearest_the_middle() {
        let grid = Grid::parse(MAP);
        // Middle row is 2 (of 5) and has no run of 5; row 1 neither; row 3 has one from its third cell.
        let cells = parade_cells(&grid, 5);
        assert!(cells.windows(2).all(|w| w[1] == w[0] + 1));
        assert_eq!(grid.coords(cells[0]), (2, 3));
    }

    #[test]
    fn without_a_run_the_first_cells_in_reading_order() {
        let grid = Grid::parse("S.^.\n.^.^\n^.^.");
        assert_eq!(parade_cells(&grid, 3), vec![1, 3, 4]);
    }
}
