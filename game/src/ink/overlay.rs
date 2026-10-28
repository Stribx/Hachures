//! What Nève reads on the sheet: routes in ink, the pencil line, le Calque and the world event's shiver.

use bevy::prelude::*;

use super::{Selection, Verdict};
use crate::Step;
use crate::level::Level;
use crate::map::flow::FlowField;
use crate::map::grid::{Grid, Tile};
use crate::map::{cell_pos, look};
use crate::schedule::over_world;
use crate::theme::{INK, SEPIA};

/// Gizmos for the dotted pencil line (preview of the detour).
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct Pencil;

pub fn plugin(app: &mut App) {
    app.insert_gizmo_config(
        Pencil,
        GizmoConfig { line: GizmoLineConfig { style: GizmoLineStyle::Dotted, width: 4.0, ..default() }, ..default() },
    )
    .add_systems(Update, draw.in_set(Step::Render).run_if(not(over_world)));
}

fn draw(
    time: Res<Time>,
    level: Res<Level>,
    selection: Res<Selection>,
    verdict: Res<Verdict>,
    mut gizmos: Gizmos,
    mut pencil: Gizmos<Pencil>,
) {
    relief(&mut gizmos, &level);
    shiver(&mut gizmos, &level, time.elapsed_secs());
    calque(&mut gizmos, &level, &selection);
    route(&mut gizmos, &level, selection.shown);
    if let Some(i) = verdict.at {
        cursor(&mut gizmos, &level, i, verdict.refusal.is_some());
    }
    if let Some((what_if, field)) = &verdict.pencil {
        trait_de_crayon(&mut pencil, &level, selection.shown, what_if, field);
    }
}

/// Hachures: slanted strokes on top of a cell's block, as if cut by hand. Each cell draws its own
/// from a hash of its index: 2 to 4 strokes, each a little longer or steeper, the same every frame.
fn hachures(gizmos: &mut Gizmos, level: &Level, i: usize, color: Color, shift: Vec3) {
    let grid = level.terrain.grid();
    let at = cell_pos(grid, i);
    let top = Vec3::new(at.x, look(grid.tiles[i]).0 + 0.02, at.z) + shift;
    let h = (i as u32).wrapping_mul(2_654_435_761);
    let n = 2 + h % 3;
    for s in 0..n {
        let bits = (h >> (4 + 5 * s)) & 31;
        let x = -0.25 + 0.5 * s as f32 / (n - 1) as f32;
        let half = 0.1 + 0.05 * (bits & 3) as f32 / 3.0;
        let slope = 0.8 + 0.4 * (bits >> 2) as f32 / 7.0;
        let d = Vec3::new(half, 0.0, -half * slope);
        gizmos.line(top + Vec3::X * x - d, top + Vec3::X * x + d, color);
    }
}

/// The relief, engraved: rock in ink (brambles have their own arches, `map::prop`).
fn relief(gizmos: &mut Gizmos, level: &Level) {
    let grid = level.terrain.grid();
    for i in (0..grid.tiles.len()).filter(|&i| grid.tiles[i] == Tile::Roc) {
        hachures(gizmos, level, i, INK, Vec3::ZERO);
    }
}

/// What the next wave changes shivers: the cells of the world event in its colour,
/// those of a scripted change (an avalanche…) in white.
fn shiver(gizmos: &mut Gizmos, level: &Level, elapsed: f32) {
    let grid = level.terrain.grid();
    let shiver = Vec3::X * (elapsed * 25.0).sin() * 0.03;
    let event = &level.world().event;
    if level.event_next && !level.event_on {
        let (r, g, b) = event.color;
        for i in (0..grid.tiles.len()).filter(|&i| grid.hit_by(event, i)) {
            hachures(gizmos, level, i, Color::srgb(r, g, b), shiver);
        }
    }
    for change in level.changes.iter().filter(|c| c.wave == level.wave.number + 1) {
        for i in change.cells(grid.width) {
            hachures(gizmos, level, i, Color::WHITE, shiver);
        }
    }
}

/// Le Calque: the shown species' flow field as arrows.
fn calque(gizmos: &mut Gizmos, level: &Level, selection: &Selection) {
    if !selection.calque {
        return;
    }
    let grid = level.terrain.grid();
    let field = &level.terrain.fields()[selection.shown];
    let at = |i| cell_pos(grid, i);
    for i in 0..grid.tiles.len() {
        if let Some(n) = field.next_step(grid, i) {
            gizmos.arrow(at(i), at(i).lerp(at(n), 0.6), SEPIA);
        }
    }
}

/// Current route of the shown species, in ink.
fn route(gizmos: &mut Gizmos, level: &Level, shown: usize) {
    let grid = level.terrain.grid();
    let field = &level.terrain.fields()[shown];
    for &spawn in grid.spawns_for(&level.species[shown]) {
        gizmos.linestrip(field.path(grid, spawn).into_iter().map(|i| cell_pos(grid, i)), INK);
    }
}

/// The grid only shows under the hand, fading out two cells around it: the relief stays one piece. The cell itself
/// is framed, in red when the tool is refused there.
fn cursor(gizmos: &mut Gizmos, level: &Level, i: usize, refused: bool) {
    let grid = level.terrain.grid();
    let (hx, hz) = grid.coords(i);
    for j in 0..grid.tiles.len() {
        let (x, z) = grid.coords(j);
        let near = (x.abs_diff(hx).max(z.abs_diff(hz))) as f32;
        if near <= 2.0 {
            let top = Vec3::new(x as f32, look(grid.tiles[j]).0 + 0.01, z as f32);
            let c = |dx: f32, dz: f32| top + Vec3::new(dx * 0.5, 0.0, dz * 0.5);
            let fade = INK.with_alpha(0.35 * (1.0 - near / 3.0));
            gizmos.linestrip([c(-1., -1.), c(1., -1.), c(1., 1.), c(-1., 1.), c(-1., -1.)], fade);
        }
    }
    let corner = |dx: f32, dz: f32| cell_pos(grid, i) + Vec3::new(dx * 0.48, 0.0, dz * 0.48);
    gizmos.linestrip(
        [corner(-1., -1.), corner(1., -1.), corner(1., 1.), corner(-1., 1.), corner(-1., -1.)],
        if refused { Color::srgb(0.9, 0.1, 0.1) } else { INK },
    );
}

/// Le trait de crayon: the route of species `shown` if the current tool were inked under the hand.
fn trait_de_crayon(pencil: &mut Gizmos<Pencil>, level: &Level, shown: usize, what_if: &Grid, field: &FlowField) {
    let grid = level.terrain.grid();
    for &spawn in grid.spawns_for(&level.species[shown]) {
        pencil.linestrip(field.path(what_if, spawn).into_iter().map(|i| cell_pos(grid, i)), Color::srgb(0.3, 0.2, 0.1));
    }
}
