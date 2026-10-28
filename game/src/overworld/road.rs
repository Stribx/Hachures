//! The road walked on the Grand Plan: how much of it is inked, and its dotted line.

use std::sync::LazyLock;

use bevy::prelude::*;

use super::plan::{self, Pawn};
use super::{OverworldRoot, ROAD, SHEETS};
use crate::campaign::save::Save;
use crate::ink::overlay::Pencil;
use crate::schedule::Screen;
use crate::theme::INK;

/// How many legs of the road are inked: it catches up with the sheets reached, one leg as Nève walks it.
#[derive(Resource)]
pub(super) struct Unrolled(f32);

pub(super) fn unroll(
    time: Res<Time>,
    save: Res<Save>,
    pawn: Res<Pawn>,
    unrolled: Option<ResMut<Unrolled>>,
    mut commands: Commands,
) {
    let reached = save.reached.min(SHEETS.len() - 1) as f32;
    let Some(mut unrolled) = unrolled else {
        commands.insert_resource(Unrolled(reached));
        return;
    };
    // The new stretch is drawn while she walks it.
    if !pawn.route.is_empty() || unrolled.0 > reached {
        unrolled.0 = (unrolled.0 + time.delta_secs() / 1.6).min(reached);
    }
}

/// The road's dotted line, just over the ground: worked out once, as finding the ground under a point means
/// measuring its distance to every stretch of the road (it was redone for every point, every frame).
static INKED_ROAD: LazyLock<Vec<Vec<Vec3>>> =
    LazyLock::new(|| ROAD.iter().map(|leg| leg.iter().map(|&p| plan::ground(p) + Vec3::Y * 0.15).collect()).collect());

/// The road walked, in dotted ink, on the Grand Plan.
pub(super) fn draw_road(
    screen: Res<State<Screen>>,
    unrolled: Option<Res<Unrolled>>,
    root: Single<&Transform, With<OverworldRoot>>,
    mut pencil: Gizmos<Pencil>,
) {
    let Some(unrolled) = unrolled else { return };
    if !matches!(screen.get(), Screen::Plan) {
        return;
    }
    for (k, leg) in INKED_ROAD.iter().enumerate() {
        let drawn = (unrolled.0 - k as f32).clamp(0.0, 1.0);
        let n = (drawn * (leg.len() - 1) as f32).round() as usize;
        if n == 0 {
            continue;
        }
        pencil.linestrip(leg[..=n].iter().map(|&p| p + root.translation), INK);
    }
}
