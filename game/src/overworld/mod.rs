//! Les Marches: one long strip of land that every level's sheet is cut out of. Hills, lakes, fields and woods
//! around the boards, the road from sheet to sheet, and each sheet not being played drawn as a miniature.
//!
//! The level being played always stays at the origin (combat, picking and the overlay all count from there);
//! the rest of the world is shifted under it instead, through `OverworldRoot`.

mod bocage;
mod camera;
mod card;
mod clouds;
mod marches;
pub mod onward;
mod parcels;
pub mod plan;
mod props;
mod road;
mod sheets;
mod terrain;
pub mod travel;

use bevy::prelude::*;

use crate::Step;
pub use sheets::{ROAD, SHEETS};

pub fn plugin(app: &mut App) {
    app.add_plugins((travel::plugin, plan::plugin))
        .add_systems(Startup, (marches::spawn, bocage::spawn_bocage).chain())
        .add_systems(Update, marches::follow_level.before(camera::aim))
        .add_systems(Update, onward::between_levels.before(crate::campaign::load_level).in_set(Step::Input));
}

/// Everything around the board being played; shifted so that board sits at the origin.
#[derive(Component)]
pub struct OverworldRoot;
