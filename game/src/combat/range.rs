//! The range circle: what a tower reaches, around the tower under the cursor and around the one the hand would ink.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::towers::{Tower, Weapon};
use crate::ink::overlay::Pencil;
use crate::ink::verdict::Verdict;
use crate::ink::{Act, Selection, TOOLS};
use crate::level::Level;
use crate::map::cell_pos;
use crate::theme::INK;

/// Draws the circle flat on the board, in the pencil's dotted line, of the reach `weapon` has at `cell`,
/// with or without a tower there yet.
fn circle(gizmos: &mut Gizmos<Pencil>, level: &Level, cell: usize, weapon: Weapon, alpha: f32) {
    let flat = Isometry3d::new(cell_pos(level.terrain.grid(), cell), Quat::from_rotation_x(FRAC_PI_2));
    gizmos.circle(flat, level.terrain.reach(cell, weapon), INK.with_alpha(alpha)).resolution(64);
}

/// The reach of a tower on the hovered cell or, failing one, of the hand's tower as if inked there
/// (fainter where it is refused: `Verdict` has judged the cell this frame).
pub(super) fn draw(
    level: Res<Level>,
    selection: Res<Selection>,
    verdict: Res<Verdict>,
    towers: Query<&Tower>,
    mut gizmos: Gizmos<Pencil>,
) {
    let Some(cell) = verdict.at else { return };
    if let Some(tower) = towers.iter().find(|t| t.cell == cell) {
        circle(&mut gizmos, &level, cell, tower.weapon, 0.9);
    } else if let Act::Raise(weapon) = TOOLS[selection.tool].act {
        let alpha = if verdict.refusal.is_some() { 0.35 } else { 0.9 };
        circle(&mut gizmos, &level, cell, weapon, alpha);
    }
}
