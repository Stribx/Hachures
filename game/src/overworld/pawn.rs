//! Nève on the Grand Plan: the Pawn that stands and walks, her figure, and the way from sheet to
//! sheet.

use std::collections::VecDeque;

use bevy::prelude::*;

use super::terrain::{WATER, height};
use super::{ROAD, SHEETS};

/// Nève on the map: where she stands or walks to, in the Marches' own coordinates (under
/// `OverworldRoot`).
#[derive(Resource)]
pub struct Pawn {
    /// The sheet she stands on, or walks to.
    pub node: usize,
    pub pos: Vec3,
    pub(super) route: VecDeque<Vec3>,
    pub(super) speed: f32,
    /// Distance walked, for her step.
    pub(super) walked: f32,
}

impl Pawn {
    /// Put her on sheet `i`, without walking.
    pub fn stand(&mut self, i: usize) {
        self.node = i;
        self.pos = spot(i);
        self.route.clear();
    }
}

/// Where Nève stands on sheet `i`: its middle, on top of the blocks.
pub(super) fn spot(i: usize) -> Vec3 {
    ground(SHEETS[i].center())
}

/// The ground under `p`: the top of a sheet's blocks, or the land (never under water).
pub(super) fn ground(p: Vec2) -> Vec3 {
    let on_sheet = SHEETS.iter().any(|s| s.distance(p) == 0.0);
    Vec3::new(p.x, if on_sheet { 0.25 } else { height(p).max(WATER) }, p.y)
}

/// The way from sheet `from` to sheet `to`, one leg after the other, either way.
pub(super) fn route(from: usize, to: usize) -> VecDeque<Vec3> {
    let mut points = Vec::new();
    let leg = |k: usize| {
        let mut leg = vec![SHEETS[k].center()];
        leg.extend(&ROAD[k]);
        leg.push(SHEETS[k + 1].center());
        leg
    };
    if to > from {
        for k in from..to {
            points.extend(leg(k));
        }
    } else {
        for k in (to..from).rev() {
            points.extend(leg(k).into_iter().rev());
        }
    }
    points.into_iter().map(ground).collect()
}

#[derive(Component)]
pub(super) struct Figure;

/// Nève walks her route: she sets off, keeps a pace, and slows to a stop; a small step bobs her up
/// and down.
pub(super) fn walk(
    time: Res<Time>,
    mut pawn: ResMut<Pawn>,
    mut figure: Single<&mut Transform, With<Figure>>,
) {
    const PACE: f32 = 7.0;
    let dt = time.delta_secs();
    let pawn = &mut *pawn;
    if let Some(&next) = pawn.route.front() {
        let left: f32 =
            pawn.route.iter().fold((pawn.pos, 0.0), |(p, d), &q| (q, d + p.distance(q))).1;
        pawn.speed = (pawn.speed + 18.0 * dt).min(PACE).min(1.2 + left * 2.5);
        let step = pawn.speed * dt;
        let to = next - pawn.pos;
        if to.length() <= step {
            pawn.pos = next;
            pawn.route.pop_front();
        } else {
            pawn.pos += to.normalize() * step;
            if to.xz().length() > 0.01 {
                let facing = Vec3::new(to.x, 0.0, to.z);
                figure.look_to(-facing, Vec3::Y);
            }
        }
        pawn.walked += step;
    } else {
        pawn.speed = 0.0;
    }
    let bob = if pawn.route.is_empty() { 0.0 } else { (pawn.walked * 5.0).sin().abs() * 0.18 };
    figure.translation = pawn.pos + Vec3::Y * bob;
}
