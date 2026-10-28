//! The camera's rig: it eases toward where each screen wants it, smoothly at both ends, and the wheel zooms it.

use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::post_process::dof::DepthOfField;
use bevy::prelude::*;

use super::plan::Pawn;
use super::travel::{Phase, Trip};
use super::{OverworldRoot, SHEETS};
use crate::level::Level;
use crate::schedule::Screen;

/// Where the camera is and where it is going: eye and target ease toward their goals, smoothly at both ends.
#[derive(Resource)]
pub struct CameraRig {
    pub eye: Vec3,
    pub look: Vec3,
    eye_velocity: Vec3,
    look_velocity: Vec3,
    goal_eye: Vec3,
    goal_look: Vec3,
    /// Seconds to get most of the way there.
    smooth: f32,
    /// The player's wheel on the level's framing, from 0.8 (closer) to 1.3.
    zoom: f32,
}

impl Default for CameraRig {
    fn default() -> Self {
        let (eye, look) = (Vec3::new(0.0, 60.0, 60.0), Vec3::ZERO);
        CameraRig {
            eye,
            look,
            eye_velocity: Vec3::ZERO,
            look_velocity: Vec3::ZERO,
            goal_eye: eye,
            goal_look: look,
            smooth: 0.4,
            zoom: 1.0,
        }
    }
}

impl CameraRig {
    fn aim(&mut self, eye: Vec3, look: Vec3, smooth: f32) {
        (self.goal_eye, self.goal_look, self.smooth) = (eye, look, smooth);
    }

    /// The world moved under the camera by `by`: move along with it, unseen.
    pub fn shift(&mut self, by: Vec3) {
        self.eye += by;
        self.look += by;
    }
}

/// Unity's SmoothDamp: a critically damped spring, so a move starts and ends gently.
#[must_use]
fn smooth_damp(current: Vec3, goal: Vec3, velocity: &mut Vec3, smooth: f32, dt: f32) -> Vec3 {
    let omega = 2.0 / smooth;
    let x = omega * dt;
    let decay = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = current - goal;
    let temp = (*velocity + omega * change) * dt;
    *velocity = (*velocity - omega * temp) * decay;
    goal + (change + temp) * decay
}

/// The camera over a sheet whose middle is at `center`.
pub fn framing(center: Vec3, zoom: f32) -> (Vec3, Vec3) {
    (center + Vec3::new(0.0, 13.0, 9.0) * zoom, center)
}

/// Where each screen wants the camera: over the board in play, behind Nève on the Grand Plan, drifting high over
/// the Marches on the title, diving into a sheet during a trip.
pub(super) fn aim(
    screen: Res<State<Screen>>,
    level: Res<Level>,
    pawn: Res<Pawn>,
    trip: Option<Res<Trip>>,
    root: Single<&Transform, With<OverworldRoot>>,
    time: Res<Time>,
    mut rig: ResMut<CameraRig>,
) {
    let world = |p: Vec3| p + root.translation;
    let sheet_center = |i: usize| {
        let c = SHEETS[i].center();
        world(Vec3::new(c.x, 0.0, c.y))
    };
    if let Some(trip) = trip.filter(|t| t.phase == Phase::Dive) {
        let (eye, look) = framing(sheet_center(trip.to), 1.0);
        rig.aim(eye, look, 0.35);
        return;
    }
    match screen.get() {
        Screen::Plan => {
            let look = world(pawn.pos) + Vec3::new(0.0, 0.0, -2.0);
            rig.aim(look + Vec3::new(0.0, 26.0, 22.0), look, 0.45);
        }
        Screen::Splash | Screen::Title | Screen::ConfirmNew => {
            let look = world(pawn.pos) + Vec3::new(6.0, 0.0, -6.0);
            let turn = Quat::from_rotation_y((time.elapsed_secs() * 0.03).sin() * 0.35);
            rig.aim(look + turn * Vec3::new(-6.0, 22.0, 26.0), look, 1.5);
        }
        _ => {
            let grid = level.terrain.grid();
            let (w, h) = (grid.width as f32, grid.height as f32);
            let (eye, look) = framing(Vec3::new((w - 1.0) / 2.0, 0.0, (h - 1.0) / 2.0), rig.zoom);
            rig.aim(eye, look, 0.35);
        }
    }
}

pub(super) fn drive(
    time: Res<Time>,
    mut rig: ResMut<CameraRig>,
    camera: Single<(&mut Transform, &mut DistanceFog, Option<&mut DepthOfField>), With<Camera3d>>,
) {
    let dt = time.delta_secs();
    let rig = &mut *rig;
    rig.eye = smooth_damp(rig.eye, rig.goal_eye, &mut rig.eye_velocity, rig.smooth, dt);
    rig.look = smooth_damp(rig.look, rig.goal_look, &mut rig.look_velocity, rig.smooth, dt);
    let (mut transform, mut fog, mut lens) = camera.into_inner();
    *transform = Transform::from_translation(rig.eye).looking_at(rig.look, Vec3::Y);
    // The paper takes the far land, however high the camera.
    let distance = rig.eye.distance(rig.look);
    fog.falloff = FogFalloff::Linear { start: distance * 1.7, end: distance * 4.5 };
    // In focus: what the camera looks at. The lens opens as the camera rises, so the Grand Plan seen from high up
    // blurs at its edges as much as a board does seen from close: a model on a table, from any height.
    // The settings may have taken the lens off.
    if let Some(lens) = lens.as_deref_mut() {
        lens.focal_distance = distance;
        lens.aperture_f_stops = PLAY_F_STOPS * (framing(Vec3::ZERO, 1.0).0.length() / distance).powi(2);
    }
}

/// The wheel brings the board closer or further, in play.
pub(super) fn zoom(
    screen: Res<State<Screen>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    mut rig: ResMut<CameraRig>,
) {
    let turned: f32 = wheel.read().map(|w| w.y.signum()).sum();
    if *screen.get() == Screen::Playing && turned != 0.0 {
        rig.zoom = (rig.zoom - turned * 0.06).clamp(0.8, 1.3);
    }
}

/// The lens at the level's framing: the board stays sharp from its near row to its far one.
const PLAY_F_STOPS: f32 = 0.2;
