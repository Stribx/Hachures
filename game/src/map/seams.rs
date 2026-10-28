//! The quilt's seams: a running stitch of cream thread where two fields of the same ground meet.

use bevy::prelude::*;

use super::cells::Cell;
use super::fields::fields;
use super::grid::{Grid, Tile};
use crate::look::{Hachure, InkMaterial};

/// The thread's colour: unbleached linen.
const THREAD: Color = Color::srgb(0.9, 0.85, 0.68);
/// Stitches along one cell's edge.
const STITCHES: usize = 4;

/// Every seam of `grid`, as the two ends of a cell's edge at the top of its ground: between two cells of different
/// fields, both still open ground of the same kind (an inked cell hides its seams, a drowned one has none).
pub(super) fn seams(grid: &Grid) -> Vec<(Vec3, Vec3)> {
    let fields = fields(grid);
    let mut seams = Vec::new();
    for i in 0..grid.tiles.len() {
        let (x, z) = grid.coords(i);
        let (x, z) = (x as f32, z as f32);
        let neighbours =
            [(i + 1, ((0.5, -0.5), (0.5, 0.5))), (i + grid.width, ((-0.5, 0.5), (0.5, 0.5)))];
        for (j, (a, b)) in neighbours {
            let beside =
                if j == i + 1 { x as usize + 1 < grid.width } else { j < grid.tiles.len() };
            let tile = grid.tiles[i];
            if !beside || !matches!(tile, Tile::Sol | Tile::SolBas) || grid.tiles[j] != tile {
                continue;
            }
            if fields[i].is_some() && fields[j].is_some() && fields[i] != fields[j] {
                let top = super::look(tile).height + 0.006;
                seams.push((Vec3::new(x + a.0, top, z + a.1), Vec3::new(x + b.0, top, z + b.1)));
            }
        }
    }
    seams
}

/// The board's stitches, one merged mesh in one thread material; nothing on a board of one field.
pub(super) fn spawn_seams(
    commands: &mut Commands,
    grid: &Grid,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<InkMaterial>,
) {
    let stitches = seams(grid).into_iter().flat_map(|(a, b)| {
        (0..STITCHES).map(move |k| {
            let at = a.lerp(b, (k as f32 + 0.5) / STITCHES as f32);
            let along = (b - a).normalize();
            Mesh::from(Cuboid::new(0.03, 0.012, 0.03).mesh())
                .scaled_by(Vec3::ONE + along.abs() * 3.5)
                .translated_by(at)
        })
    });
    let Some(mesh) = stitches.reduce(|mut all, stitch| {
        all.merge(&stitch).expect("stitches share their attributes");
        all
    }) else {
        return;
    };
    let mut thread = crate::look::ink(THREAD);
    thread.extension = Hachure::fine();
    commands.spawn((Cell, Mesh3d(meshes.add(mesh)), MeshMaterial3d(materials.add(thread))));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each seam runs between two cells of different fields, never along the board's edge or over water.
    #[test]
    fn seams_part_two_fields() {
        let grid = Grid::parse(
            "
            S...,,,,....,,,,....M
            .....,,,,.......,,,..
            ...^^^...~~~.........
            ",
        );
        let fields = fields(&grid);
        let seams = seams(&grid);
        assert!(!seams.is_empty());
        for (a, b) in seams {
            let mid = (a + b) / 2.0;
            let (dx, dz) = if a.x == b.x { (0.5, 0.0) } else { (0.0, 0.5) };
            let cell = |x: f32, z: f32| z.round() as usize * grid.width + x.round() as usize;
            let (i, j) = (cell(mid.x - dx, mid.z - dz), cell(mid.x + dx, mid.z + dz));
            assert!(
                fields[i].is_some() && fields[j].is_some() && fields[i] != fields[j],
                "{a} {b}"
            );
            assert_eq!(grid.tiles[i], grid.tiles[j]);
        }
    }
}
