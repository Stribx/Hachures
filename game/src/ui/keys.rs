//! The keyboard toggles: Tab for the next species, F1 for le Calque, F for double speed.

use bevy::input::keyboard::Key;
use bevy::prelude::*;

use super::widgets::letter;
use crate::ink::Selection;
use crate::level::Level;
use crate::schedule::Screen;

/// Tab: the next species. F1: le Calque. F: speed ×2. (A, the sceaux tree, is in `menu`.)
pub(super) fn toggles(
    keys: Res<ButtonInput<KeyCode>>,
    letters: Res<ButtonInput<Key>>,
    level: Res<Level>,
    mut selection: ResMut<Selection>,
    mut fast: ResMut<Fast>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        next_species(&mut selection, &level);
    }
    if keys.just_pressed(KeyCode::F1) {
        selection.calque = !selection.calque;
    }
    if letter(&letters, "f") {
        fast.0 = !fast.0;
    }
}

/// The game at double speed: waves, walkers and towers, not the menus over it.
#[derive(Resource, Default)]
pub struct Fast(pub bool);

pub(super) fn speed(fast: Res<Fast>, screen: Res<State<Screen>>, mut time: ResMut<Time<Virtual>>) {
    let ratio = if fast.0 && *screen.get() == Screen::Playing { 2.0 } else { 1.0 };
    if time.relative_speed() != ratio {
        time.set_relative_speed(ratio);
    }
}

/// Shows the route of the next species the level sends.
pub(super) fn next_species(selection: &mut Selection, level: &Level) {
    let next = level
        .present
        .iter()
        .position(|&k| k == selection.shown)
        .map_or(0, |p| (p + 1) % level.present.len());
    selection.shown = level.present[next];
}
