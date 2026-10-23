//! Between two levels: the sceaux tree, the level's choice, and on to the next level (or the same one again).

pub mod save;
pub mod skills;

use bevy::prelude::*;

use crate::Step;
use crate::level::data::LEVELS;
use crate::level::{Level, LevelEntity};
use crate::ui::TreeOpen;
use save::Save;

pub fn plugin(app: &mut App) {
    app.add_message::<LoadLevel>().add_systems(Update, (between_levels, load_level).chain().in_set(Step::Input));
}

/// Play level `.0` of `LEVELS` from its start.
#[derive(Message)]
pub struct LoadLevel(pub usize);

/// Once won, digits buy skills (tree open) or answer the level's choice. Enter goes on to the
/// next level once won and the choice made, or plays the same one again once lost.
pub fn between_levels(
    keys: Res<ButtonInput<KeyCode>>,
    tree: Res<TreeOpen>,
    mut level: ResMut<Level>,
    mut save: ResMut<Save>,
    mut load: MessageWriter<LoadLevel>,
) {
    let digit = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3].iter().position(|&k| keys.just_pressed(k));
    if let (true, Some(digit)) = (level.finished, digit) {
        if tree.0 {
            if let Some(node) = skills::next_in(digit, &save.skills).filter(|n| save.sceaux() >= n.cost) {
                save.skills.push(node.skill);
                save.store();
            }
        } else if level.choice.is_some() {
            level.choose(digit, &mut save);
            save.store();
        }
        return;
    }

    if keys.just_pressed(KeyCode::Enter) {
        let won = level.finished && level.chosen();
        let next = if won { level.index + 1 } else { level.index };
        if (won || level.lives == 0) && next < LEVELS.len() {
            load.write(LoadLevel(next));
        }
    }
}

/// Clears the level being played and loads the one asked for.
pub fn load_level(
    mut loads: MessageReader<LoadLevel>,
    save: Res<Save>,
    mut level: ResMut<Level>,
    mut tree: ResMut<TreeOpen>,
    entities: Query<Entity, With<LevelEntity>>,
    mut commands: Commands,
) {
    let Some(&LoadLevel(index)) = loads.read().last() else { return };
    for entity in &entities {
        commands.entity(entity).despawn();
    }
    *level = Level::load(index, &save);
    // The tree is bought from between levels; play starts on the map.
    tree.0 = false;
}
