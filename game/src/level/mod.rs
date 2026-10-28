//! A level: its map as inked, its world's event, waves and sépia, and what it leaves in the campaign.
//! `Level` is one resource; each feature adds its own `impl Level` (inking and le Droit de Passage in `ink`).
//! Its map is a `Terrain`, which also owns the towers' reach.

pub mod data;
mod load;
mod state;
pub mod story;
pub mod waves;
pub mod world;

use bevy::prelude::*;

use crate::Step;
pub(crate) use state::finish_level;
pub use state::{Inked, Level, Walker};

pub fn plugin(app: &mut App) {
    // Once the Rôle is lost, the sheet stands still until Enter plays again.
    app.configure_sets(Update, Step::Simulate.run_if(alive))
        .add_systems(Update, finish_level.in_set(Step::Simulate));
}

/// Some of the Rôle is left: the simulation goes on.
fn alive(level: Res<Level>) -> bool {
    level.lives > 0
}

/// Tags what belongs to the level being played, to clear it when another one loads.
#[derive(Component)]
pub struct LevelEntity;

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;
    use crate::campaign::save::Save;
    use crate::level::data::LEVELS;
    use crate::map::flow::FlowField;
    use crate::map::grid::Grid;
    use crate::map::species;
    use crate::schedule::Screen;

    /// Once the Rôle is lost, the simulation stands still, even during play.
    #[test]
    fn a_lost_level_stands_still() {
        #[derive(Resource, Default)]
        struct Ticks(u32);
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, crate::schedule::plugin, plugin))
            .init_resource::<Ticks>()
            .insert_resource(Save::default())
            .insert_resource(Level::new(Grid::parse("S.M"), species::load()))
            .add_systems(Update, (|mut ticks: ResMut<Ticks>| ticks.0 += 1).in_set(Step::Simulate));
        app.world_mut().resource_mut::<NextState<Screen>>().set(Screen::Playing);
        app.update();
        assert_eq!(app.world().resource::<Ticks>().0, 1, "playing");
        app.world_mut().resource_mut::<Level>().lives = 0;
        app.update();
        assert_eq!(app.world().resource::<Ticks>().0, 1, "the Rôle is lost");
    }

    /// Every level loads, and every species it sends can reach its goal, with and without the world event,
    /// whatever was chosen.
    #[test]
    fn levels_are_playable() {
        let mut other = Save::default();
        other.choices.insert("refuge".into(), "bois".into());
        other.choices.insert("deserteurs".into(), "livrer".into());
        for save in [Save::default(), other] {
            for index in 0..LEVELS.len() {
                let level = Level::load(index, &save);
                assert!(
                    (1..=12).contains(&level.waves.len()),
                    "{}: up to 12 waves (docs)",
                    level.name
                );
                for (k, species) in level.species.iter().enumerate() {
                    if !level.waves.iter().flatten().any(|&s| s == k) {
                        continue;
                    }
                    let spawns = level.terrain.base().spawns_for(species);
                    assert!(
                        !spawns.is_empty(),
                        "{}: nowhere for {} to spawn",
                        level.name,
                        species.name
                    );
                    let event = &level.world().event;
                    let later = level.after_changes();
                    let base = level.terrain.base();
                    for map in [base.under(event), later.under(event), base.clone(), later] {
                        let field = FlowField::new(&map, species);
                        assert!(
                            spawns.iter().all(|&s| field.dist[s].is_finite()),
                            "{}: {} can't reach its goal",
                            level.name,
                            species.name
                        );
                    }
                }
            }
        }
    }

    /// Every named place stands on ground Nève can ink: otherwise its name could never cost anything.
    #[test]
    fn places_can_be_inked() {
        for index in 0..LEVELS.len() {
            let level = Level::load(index, &Save::default());
            for place in &level.places {
                for &(x, y) in &place.at {
                    let base = level.terrain.base();
                    let i = y * base.width + x;
                    assert!(
                        base.tiles[i].buildable(),
                        "{}: {} at ({x}, {y})",
                        level.name,
                        place.name
                    );
                }
            }
        }
    }

    /// Le Droit de Passage and Tab only consider the species a level sends, boss phases included.
    #[test]
    fn present_species() {
        fn names(level: &Level) -> Vec<&str> {
            level.present.iter().map(|&k| level.species[k].name.as_str()).collect()
        }
        let at = |prefix: &str| {
            let index = (0..LEVELS.len())
                .find(|&i| Level::load(i, &Save::default()).name == format!("level.{prefix}.name"))
                .unwrap();
            Level::load(index, &Save::default())
        };
        let (col, boss_level) = (at("2-1"), at("1-4"));
        assert_eq!(names(&col), ["chaineur", "gens_du_col", "harde"]);
        let boss = names(&boss_level);
        assert!(
            boss.contains(&"grande_chaine") && boss.contains(&"grande_chaine_rompue"),
            "{boss:?}"
        );
    }

    /// A replay can't rewrite the Rôle: the first win's Names and scratched-out places stand.
    #[test]
    fn a_replay_keeps_the_first_names() {
        let mut save = Save::default();
        let mut first = Level::load(1, &save);
        first.finish(&mut save);
        let mut replay = Level::load(1, &save);
        replay.names = -3;
        replay.erased = vec![0];
        replay.finish(&mut save);
        assert_eq!((save.names(), save.levels[1].erased.len()), (0, 0));
        assert!(save.levels[1].won);
    }

    /// Handing the deserters over in 2-2 drops the last two waves of 2-3, and costs a Name.
    #[test]
    fn deserters_choice_shortens_the_escort() {
        let escort = (0..LEVELS.len())
            .find(|&i| Level::load(i, &Save::default()).name == "level.2-3.name")
            .unwrap();
        let mut save = Save::default();
        save.choices.insert("deserteurs".into(), "escorter".into());
        let full = Level::load(escort, &save);
        save.choices.insert("deserteurs".into(), "livrer".into());
        let short = Level::load(escort, &save);
        assert_eq!(short.waves.len(), full.waves.len() - 2);
        assert_eq!((full.names, short.names), (0, -1));
    }
}
