//! The campaign: the save, and loading a level.

pub mod save;
pub mod skills;

use bevy::prelude::*;

use crate::Step;
use crate::level::data::LEVELS;
use crate::level::{Level, LevelEntity};
use save::Save;

/// The campaign as saved, and the furthest level reached ready to play.
pub fn plugin(app: &mut App) {
    let save = Save::load();
    let level = Level::load(save.reached.min(LEVELS.len() - 1), &save);
    app.insert_resource(level)
        .insert_resource(save)
        .add_message::<LoadLevel>()
        .add_systems(Update, load_level.in_set(Step::Input));
}

/// Play level `.0` of `LEVELS` from its start.
#[derive(Message)]
pub struct LoadLevel(pub usize);

/// Clears the level being played and loads the one asked for.
pub fn load_level(
    mut loads: MessageReader<LoadLevel>,
    save: Res<Save>,
    mut level: ResMut<Level>,
    entities: Query<Entity, With<LevelEntity>>,
    mut commands: Commands,
) {
    let Some(&LoadLevel(index)) = loads.read().last() else { return };
    for entity in &entities {
        commands.entity(entity).despawn();
    }
    *level = Level::load(index, &save);
}
