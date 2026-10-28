//! Towers, enemies, waves and La Grande Chaîne.

pub mod boss;
mod draw;
pub mod enemies;
mod enemy;
pub mod figure;
mod noted;
mod range;
mod shards;
#[cfg(test)]
mod tests;
pub mod towers;

use bevy::prelude::*;

use crate::Step;
use draw::Shots;
pub use enemy::Enemy;

pub fn plugin(app: &mut App) {
    app.init_resource::<Shots>().add_systems(Startup, figure::setup).add_systems(
        Update,
        (
            enemies::start_waves.in_set(Step::Input),
            (towers::raise_towers, towers::take_down).in_set(Step::Input).after(crate::ink::inking),
            boss::drop_bornes.in_set(Step::Input).after(crate::ink::inking),
            (enemies::spawn_enemies, noted::notice, enemies::walk, towers::fire, boss::boss_phases, shards::shards)
                .chain()
                .in_set(Step::Simulate)
                .before(crate::level::finish_level),
            figure::animate.in_set(Step::Simulate).after(enemies::walk),
            figure::drag_chain.in_set(Step::Simulate).after(figure::animate),
            (figure::exits, shards::blots).in_set(Step::Simulate),
            (draw::draw, range::draw).in_set(Step::Render),
        ),
    );
}
