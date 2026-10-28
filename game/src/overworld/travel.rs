//! The trip from the Grand Plan down into a sheet: the dive, the clouds closing, the level swapped,
//! the reveal.

use bevy::prelude::*;

use super::camera::{CameraRig, aim, drive, zoom};
use super::clouds::{ink_clouds, place_clouds, spawn_clouds};
use super::pawn::Pawn;
use crate::campaign::LoadLevel;
use crate::level::Level;
use crate::schedule::Screen;

pub fn plugin(app: &mut App) {
    app.init_resource::<CameraRig>()
        .add_message::<Travel>()
        .add_systems(Startup, spawn_clouds)
        .add_systems(Update, (start_trip, trip, aim, drive, zoom, place_clouds).chain())
        .add_systems(Update, ink_clouds);
}

/// Go and play level `.0`: from the Grand Plan the camera dives into its sheet; from anywhere else
/// the clouds just close and open on it. `again` replays it from the start even if it is the level
/// on the board.
#[derive(Message)]
pub struct Travel {
    pub to: usize,
    pub again: bool,
}

#[derive(Resource)]
pub struct Trip {
    pub to: usize,
    again: bool,
    pub phase: Phase,
    t: f32,
    /// How far the clouds have closed, from 0 (clear sky) to 1 (the world hidden).
    pub(super) cover: f32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Phase {
    Dive,
    Cover,
    Reveal,
}

const DIVE: f32 = 0.9;
const CLOSE: f32 = 0.35;
const OPEN: f32 = 0.5;

fn start_trip(
    mut travels: MessageReader<Travel>,
    screen: Res<State<Screen>>,
    trip: Option<Res<Trip>>,
    mut commands: Commands,
) {
    let Some(travel) = travels.read().last() else { return };
    if trip.is_some() {
        return;
    }
    let phase = if *screen.get() == Screen::Plan { Phase::Dive } else { Phase::Cover };
    commands.insert_resource(Trip {
        to: travel.to,
        again: travel.again,
        phase,
        t: 0.0,
        cover: 0.0,
    });
}

/// Dive, close the clouds, swap the level behind them, open them on the new board.
fn trip(
    time: Res<Time>,
    trip: Option<ResMut<Trip>>,
    level: Res<Level>,
    mut pawn: ResMut<Pawn>,
    mut load: MessageWriter<LoadLevel>,
    mut next: ResMut<NextState<Screen>>,
    mut commands: Commands,
) {
    let Some(mut trip) = trip else { return };
    // Loading the level behind the closed clouds takes a long frame: the trip's clock never leaps
    // more than a frame, so the clouds open from closed instead of snapping half open.
    trip.t += time.delta_secs().min(1.0 / 30.0);
    let ease_out = |t: f32| 1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3);
    match trip.phase {
        Phase::Dive => {
            // The clouds start closing as the camera nears the sheet.
            trip.cover = ease_out((trip.t - (DIVE - CLOSE)) / CLOSE);
            if trip.t >= DIVE {
                (trip.phase, trip.t) = (Phase::Cover, CLOSE);
            }
        }
        Phase::Cover => {
            trip.cover = ease_out(trip.t / CLOSE);
            if trip.t >= CLOSE {
                // The level on the board is resumed if it is the one asked for and still under way.
                let resume =
                    trip.to == level.index && !trip.again && !level.finished && level.lives > 0;
                if !resume {
                    load.write(LoadLevel(trip.to));
                }
                pawn.stand(trip.to);
                next.set(Screen::Playing);
                (trip.phase, trip.t) = (Phase::Reveal, 0.0);
            }
        }
        Phase::Reveal => {
            trip.cover = 1.0 - ease_out(trip.t / OPEN);
            if trip.t >= OPEN {
                commands.remove_resource::<Trip>();
            }
        }
    }
}
