//! The campaign: the save, and loading a level.

pub mod lexique;
pub mod save;
pub mod skills;

use bevy::prelude::*;

use crate::Step;
use crate::i18n::n_;
use crate::ink::TOOLS;
use crate::level::data::LEVELS;
use crate::level::story::MarginNote;
use crate::level::{Level, LevelEntity};
use save::Save;

/// The campaign as saved, and the furthest level reached ready to play.
pub fn plugin(app: &mut App) {
    let mut save = Save::load();
    let mut level = Level::load(save.reached.min(LEVELS.len() - 1), &save);
    // Not stored yet: a player who has done nothing leaves no save behind.
    meet_tools(&mut save, &mut level);
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
    mut save: ResMut<Save>,
    mut level: ResMut<Level>,
    entities: Query<Entity, With<LevelEntity>>,
    mut commands: Commands,
) {
    let Some(&LoadLevel(index)) = loads.read().last() else { return };
    for entity in &entities {
        commands.entity(entity).despawn();
    }
    *level = Level::load(index, &save);
    if meet_tools(&mut save, &mut level) {
        save.store();
    }
}

/// The tools in the bar of `level` join the Lexique, each new one noted in the margin. True if any was new.
fn meet_tools(save: &mut Save, level: &mut Level) -> bool {
    let mut new = false;
    for t in level.tools.clone() {
        if save.discover(TOOLS[t].label) {
            let line = n_("lexique.noted").to_string();
            level.margin.push(MarginNote { line, name: Some(TOOLS[t].label.to_string()) });
            new = true;
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_a_level_meets_its_tools() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_message::<LoadLevel>()
            .insert_resource(Save::default())
            .insert_resource(Level::load(0, &Save::default()))
            .add_systems(Update, load_level);
        app.world_mut().write_message(LoadLevel(1));
        app.update();
        let seen = &app.world().resource::<Save>().seen;
        let level = app.world().resource::<Level>();
        assert_eq!(level.index, 1);
        assert_eq!(seen.len(), 3, "1-1 shows three tools");
        assert!(level.tools.iter().all(|&t| seen.contains(TOOLS[t].label)));
    }
}
