//! Nève's tools: what she may ink where, what it costs, and the click that inks it.

mod biffer;
mod ghost;
mod inking;
pub mod overlay;
mod passage;
mod rename;
pub mod selection;
mod tools;
pub mod verdict;

use bevy::prelude::*;

use crate::Step;
use crate::campaign::load_level;
use crate::combat::towers::Weapon;
use crate::map::land::fibres;
pub(crate) use ghost::ghost_ink;
pub(crate) use inking::inking;
pub use selection::Selection;
pub use tools::{Act, TOOLS, Tool, tool_index};
pub use verdict::Verdict;

pub fn plugin(app: &mut App) {
    app.add_plugins((overlay::plugin, selection::plugin))
        .add_message::<TowerInked>()
        .add_message::<TowerBiffed>()
        .init_resource::<Verdict>()
        .add_systems(
            Update,
            inking
                .in_set(Step::Input)
                .after(selection::hover)
                .after(selection::reset)
                .after(load_level),
        )
        .add_systems(Update, verdict::judge.after(inking).before(Step::Render))
        .add_systems(Update, ghost::ghost.after(fibres).in_set(Step::Render));
}

/// A tower was inked on `cell`; combat stands it up.
#[derive(Message)]
pub struct TowerInked {
    pub cell: usize,
    pub weapon: Weapon,
}

/// The tower on `cell` was biffed; combat takes it down.
#[derive(Message)]
pub struct TowerBiffed {
    pub cell: usize,
}
