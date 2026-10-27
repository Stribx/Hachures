//! Le Grand Plan (docs, section 7): the Marches seen from above, Nève on the road from sheet to sheet. Left and right
//! walk her to the next sheet reached, Enter dives into the one she stands on. Each sheet won carries a red seal;
//! the road she has walked is inked over, and the new stretch is drawn as she walks it the first time.

use std::collections::VecDeque;

use bevy::input::keyboard::Key;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;

use super::card::{card, spawn_page};
use super::road::{draw_road, unroll};
use super::terrain::{WATER, height};
use super::travel::Travel;
use super::{OverworldRoot, ROAD, SHEETS};
use crate::campaign::save::Save;
use crate::level::Level;
use crate::look::{InkMaterial, ink};
use crate::schedule::{Screen, over_world};
use crate::theme::{PAPER, SEAL};
use crate::ui::widgets::letter;

pub fn plugin(app: &mut App) {
    app.add_message::<WalkTo>()
        // Under the Marches' root, once it exists.
        .add_systems(Startup, spawn.after(super::spawn))
        .add_systems(OnEnter(Screen::Plan), (spawn_page, seals))
        .add_systems(Update, (walk_to, keys.run_if(in_state(Screen::Plan)), walk, unroll, draw_road, card).chain())
        .add_systems(Update, show_over_world.run_if(state_changed::<Screen>));
}

/// Nève on the map: where she stands or walks to, in the Marches' own coordinates (under `OverworldRoot`).
#[derive(Resource)]
pub struct Pawn {
    /// The sheet she stands on, or walks to.
    pub node: usize,
    pub pos: Vec3,
    pub(super) route: VecDeque<Vec3>,
    speed: f32,
    /// Distance walked, for her step.
    walked: f32,
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
fn spot(i: usize) -> Vec3 {
    ground(SHEETS[i].center())
}

/// The ground under `p`: the top of a sheet's blocks, or the land (never under water).
pub(super) fn ground(p: Vec2) -> Vec3 {
    let on_sheet = SHEETS.iter().any(|s| s.distance(p) == 0.0);
    Vec3::new(p.x, if on_sheet { 0.25 } else { height(p).max(WATER) }, p.y)
}

/// The way from sheet `from` to sheet `to`, one leg after the other, either way.
fn route(from: usize, to: usize) -> VecDeque<Vec3> {
    let mut points = Vec::new();
    let leg = |k: usize| {
        let mut leg = vec![SHEETS[k].center()];
        leg.extend(&ROAD[k]);
        leg.push(SHEETS[k + 1].center());
        leg
    };
    if to > from {
        (from..to).for_each(|k| points.extend(leg(k)));
    } else {
        (to..from).rev().for_each(|k| points.extend(leg(k).into_iter().rev()));
    }
    points.into_iter().map(ground).collect()
}

/// Walk Nève to sheet `.0` on the Grand Plan (after a level won: on to the next one).
#[derive(Message)]
pub struct WalkTo(pub usize);

#[derive(Component)]
struct Figure;

/// Shown only over the world: Nève and the seals.
#[derive(Component)]
struct OverWorld;

/// A seal over a sheet.
#[derive(Component)]
struct Seal;

/// Materials for the seals: won, reached.
#[derive(Resource)]
struct SealInks(Handle<InkMaterial>, Handle<InkMaterial>);

fn spawn(
    level: Res<Level>,
    root: Single<Entity, With<OverworldRoot>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    let pos = spot(level.index);
    commands.insert_resource(Pawn { node: level.index, pos, route: VecDeque::new(), speed: 0.0, walked: 0.0 });
    // Nève: a long travelling coat, a head, a folded map held out.
    let coat = materials.add(ink(Color::srgb(0.3, 0.22, 0.16)));
    let paper = materials.add(ink(PAPER));
    commands
        .spawn((
            Figure,
            OverWorld,
            Transform::from_translation(pos).with_scale(Vec3::splat(1.6)),
            Visibility::default(),
            ChildOf(*root),
        ))
        .with_children(|figure| {
            figure.spawn((
                Mesh3d(meshes.add(Cone::new(0.38, 1.1))),
                MeshMaterial3d(coat.clone()),
                Transform::from_xyz(0.0, 0.55, 0.0),
            ));
            figure.spawn((
                Mesh3d(meshes.add(Sphere::new(0.2))),
                MeshMaterial3d(paper.clone()),
                Transform::from_xyz(0.0, 1.25, 0.0),
            ));
            figure.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.5, 0.04, 0.35))),
                MeshMaterial3d(paper),
                Transform::from_xyz(0.0, 0.8, 0.35).with_rotation(Quat::from_rotation_x(-0.5)),
            ));
        });
    commands.insert_resource(SealInks(
        materials.add(ink(SEAL)),
        // Reached, not won yet: a seal of bare paper.
        materials.add(ink(PAPER)),
    ));
}

/// A seal over each sheet reached, red once won; none over the blank sheets.
fn seals(
    save: Res<Save>,
    inks: Res<SealInks>,
    old: Query<Entity, With<Seal>>,
    root: Single<Entity, With<OverworldRoot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    old.iter().for_each(|e| commands.entity(e).despawn());
    let disc = meshes.add(Cylinder::new(0.6, 0.14));
    for i in 0..=save.reached.min(SHEETS.len() - 1) {
        let won = save.levels.get(i).is_some_and(|r| r.won);
        let c = SHEETS[i].center();
        commands.spawn((
            Seal,
            OverWorld,
            Mesh3d(disc.clone()),
            MeshMaterial3d(if won { inks.0.clone() } else { inks.1.clone() }),
            NotShadowCaster,
            // Standing up, facing the camera, over the sheet's far edge.
            Transform::from_xyz(c.x, 2.2, c.y - SHEETS[i].size().y / 2.0 + 0.5)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2 - 0.6)),
            ChildOf(*root),
        ));
    }
}

fn walk_to(mut walks: MessageReader<WalkTo>, mut pawn: ResMut<Pawn>, mut next: ResMut<NextState<Screen>>) {
    let Some(&WalkTo(to)) = walks.read().last() else { return };
    next.set(Screen::Plan);
    let from = pawn.node;
    if to != from {
        pawn.route = route(from, to);
        pawn.node = to;
    }
}

fn keys(
    keys: Res<ButtonInput<KeyCode>>,
    letters: Res<ButtonInput<Key>>,
    save: Res<Save>,
    pawn: Res<Pawn>,
    mut walks: MessageWriter<WalkTo>,
    mut travel: MessageWriter<Travel>,
) {
    let last = save.reached.min(SHEETS.len() - 1);
    // Q and D as printed on the keys: on AZERTY they sit where QWERTY has A and D.
    if (keys.just_pressed(KeyCode::ArrowRight) || letter(&letters, "d")) && pawn.node < last {
        walks.write(WalkTo(pawn.node + 1));
    } else if (keys.just_pressed(KeyCode::ArrowLeft) || letter(&letters, "q")) && pawn.node > 0 {
        walks.write(WalkTo(pawn.node - 1));
    } else if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) && pawn.route.is_empty() {
        travel.write(Travel { to: pawn.node, again: false });
    }
}

/// Nève walks her route: she sets off, keeps a pace, and slows to a stop; a small step bobs her up and down.
fn walk(time: Res<Time>, mut pawn: ResMut<Pawn>, mut figure: Single<&mut Transform, With<Figure>>) {
    const PACE: f32 = 7.0;
    let dt = time.delta_secs();
    let pawn = &mut *pawn;
    if let Some(&next) = pawn.route.front() {
        let left: f32 = pawn.route.iter().fold((pawn.pos, 0.0), |(p, d), &q| (q, d + p.distance(q))).1;
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

/// Nève and the seals stand on the Grand Plan and under the title, not on the board in play.
fn show_over_world(screen: Res<State<Screen>>, mut shown: Query<&mut Visibility, With<OverWorld>>) {
    let visibility = if over_world(screen) { Visibility::Inherited } else { Visibility::Hidden };
    shown.iter_mut().for_each(|mut v| *v = visibility);
}
