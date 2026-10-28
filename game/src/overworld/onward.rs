//! What comes after a level: the choice's digits, and Enter on to the next sheet or the same one again.

use bevy::prelude::*;

use crate::campaign::save::Save;
use crate::level::Level;
use crate::level::data::LEVELS;
use crate::overworld::plan::WalkTo;
use crate::overworld::travel::Travel;

/// Once won, digits answer the level's choice (the page at the end of the level has buttons too). Enter goes on to
/// the next level on the Grand Plan once won and the choice made, or plays the same one again once lost.
pub fn between_levels(
    keys: Res<ButtonInput<KeyCode>>,
    mut level: ResMut<Level>,
    mut save: ResMut<Save>,
    mut travel: MessageWriter<Travel>,
    mut walks: MessageWriter<WalkTo>,
) {
    let digit = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3]
        .iter()
        .position(|&k| keys.just_pressed(k));
    if let (true, Some(digit)) = (level.finished, digit) {
        level.choose(digit, &mut save);
        save.store();
        return;
    }

    if keys.just_pressed(KeyCode::Enter) {
        // Won: back on the Grand Plan, Nève walks on to the next sheet. Lost: the same sheet again.
        if level.finished && level.chosen() && level.index + 1 < LEVELS.len() {
            walks.write(WalkTo(level.index + 1));
        } else if level.lives == 0 {
            travel.write(Travel { to: level.index, again: true });
        }
    }
}
