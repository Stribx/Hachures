//! Le Grand Plan (docs, section 7): the Marches seen from above, Nève on the road from sheet to sheet. Left and right
//! walk her to the next sheet reached, Enter dives into the one she stands on. Each sheet won carries a red seal;
//! the road she has walked is inked over, and the new stretch is drawn as she walks it the first time.

use std::collections::VecDeque;

use bevy::input::keyboard::Key;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;

use super::card::{card, spawn_page};
use super::pawn::{Figure, Pawn, route, spot, walk};
use super::road::{draw_road, unroll};
use super::travel::Travel;
use super::{OverworldRoot, SHEETS};
use crate::campaign::save::Save;
use crate::level::Level;
use crate::look::{InkMaterial, ink};
use crate::schedule::{Screen, over_world};
use crate::theme::{PAPER, SEAL};
use crate::ui::widgets::letter;

pub fn plugin(app: &mut App) {
    app.add_message::<WalkTo>()
        // Under the Marches' root, once it exists.
        .add_systems(Startup, spawn.after(super::marches::spawn))
        .add_systems(OnEnter(Screen::Plan), (spawn_page, seals))
        .add_systems(
            Update,
            (walk_to, keys.run_if(in_state(Screen::Plan)), walk, unroll, draw_road, card).chain(),
        )
        .add_systems(Update, show_over_world.run_if(state_changed::<Screen>));
}

/// Walk Nève to sheet `.0` on the Grand Plan (after a level won: on to the next one).
#[derive(Message)]
pub struct WalkTo(pub usize);

/// Shown only over the world: Nève and the seals.
#[derive(Component)]
struct OverWorld;

/// A seal over a sheet.
#[derive(Component)]
struct Seal;

/// Materials for the seals of the sheets won and of those merely reached.
#[derive(Resource)]
struct SealInks {
    won: Handle<InkMaterial>,
    reached: Handle<InkMaterial>,
}

fn spawn(
    level: Res<Level>,
    root: Single<Entity, With<OverworldRoot>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    let pos = spot(level.index);
    commands.insert_resource(Pawn {
        node: level.index,
        pos,
        route: VecDeque::new(),
        speed: 0.0,
        walked: 0.0,
    });
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
    commands.insert_resource(SealInks {
        won: materials.add(ink(SEAL)),
        // Reached, not won yet: a seal of bare paper.
        reached: materials.add(ink(PAPER)),
    });
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
    for entity in &old {
        commands.entity(entity).despawn();
    }
    let disc = meshes.add(Cylinder::new(0.6, 0.14));
    for i in 0..=save.reached.min(SHEETS.len() - 1) {
        let won = save.levels.get(i).is_some_and(|r| r.won);
        let c = SHEETS[i].center();
        commands.spawn((
            Seal,
            OverWorld,
            Mesh3d(disc.clone()),
            MeshMaterial3d(if won { inks.won.clone() } else { inks.reached.clone() }),
            NotShadowCaster,
            // Standing up, facing the camera, over the sheet's far edge.
            Transform::from_xyz(c.x, 2.2, c.y - SHEETS[i].size().y / 2.0 + 0.5)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2 - 0.6)),
            ChildOf(*root),
        ));
    }
}

fn walk_to(
    mut walks: MessageReader<WalkTo>,
    mut pawn: ResMut<Pawn>,
    mut next: ResMut<NextState<Screen>>,
) {
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

/// Nève and the seals stand on the Grand Plan and under the title, not on the board in play.
fn show_over_world(screen: Res<State<Screen>>, mut shown: Query<&mut Visibility, With<OverWorld>>) {
    let visibility = if over_world(screen) { Visibility::Inherited } else { Visibility::Hidden };
    for mut slot in &mut shown {
        *slot = visibility;
    }
}
