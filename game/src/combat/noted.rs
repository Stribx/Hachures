//! The Lexique learns a species the first time one of its figures stands on the board.

use bevy::prelude::*;

use super::Enemy;
use crate::campaign::save::Save;
use crate::i18n::n_;
use crate::level::Level;
use crate::level::story::MarginNote;

/// Notes the species on the board that are new to the Lexique, in Nève's margin, and keeps the
/// save. Looks at every enemy: a boss changes species between its phases.
pub(super) fn notice(enemies: Query<&Enemy>, mut level: ResMut<Level>, mut save: ResMut<Save>) {
    let mut new = false;
    for enemy in &enemies {
        let label = &level.species[enemy.species].label;
        // Asked first, so a species already known leaves the save unchanged for change detection.
        if !save.seen.contains(label) {
            save.discover(label);
            let line = n_("lexique.noted").to_string();
            let label = label.clone();
            level.margin.push(MarginNote { line, name: Some(label) });
            new = true;
        }
    }
    if new {
        save.store();
    }
}
