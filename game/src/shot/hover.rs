//! Dev tool: points at things without a mouse, for shots of the reading aids. `HACHURES_HOVER=tower|enemy|<cell>`
//! puts the cursor on the first tower, the first enemy or a cell; `HACHURES_TOOL=n` takes tool `n` of `TOOLS`;
//! `HACHURES_CARD=n` hovers the bar's card for tool `n`.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;

use super::env;
use crate::combat::Enemy;
use crate::combat::towers::Tower;
use crate::ink::Selection;
use crate::level::Level;
use crate::ui::bar::ToolButton;

pub(super) fn hover(
    level: Res<Level>,
    mut selection: ResMut<Selection>,
    towers: Query<&Tower>,
    enemies: Query<&Transform, With<Enemy>>,
    cards: Query<(Entity, &ToolButton)>,
    mut commands: Commands,
) {
    if let Some(t) = env("HACHURES_TOOL").and_then(|n| n.parse::<usize>().ok()) {
        selection.tool = t;
    }
    if let Some(t) = env("HACHURES_CARD").and_then(|n| n.parse::<usize>().ok()) {
        // `Hovered` is immutable: picking replaces it, and so does this.
        for (entity, card) in &cards {
            commands.entity(entity).insert(Hovered(card.0 == t));
        }
    }
    let width = level.terrain.grid().width;
    let cell = match env("HACHURES_HOVER").as_deref() {
        Some("tower") => towers.iter().next().map(|t| t.cell),
        Some("enemy") => {
            enemies.iter().next().map(|p| p.translation.z.round() as usize * width + p.translation.x.round() as usize)
        }
        Some(n) => n.parse().ok(),
        None => return,
    };
    if cell.is_some() {
        selection.hovered = cell;
    }
}
