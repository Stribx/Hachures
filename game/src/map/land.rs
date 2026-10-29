//! A piece inked by hand drops into its cell, and the land answers with a sound and a puff of
//! fibres.

use std::f32::consts::PI;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::grid::Tile;
use crate::look::{self, InkMaterial};
use crate::settings::Settings;

/// Light wood on wood: a barricade landing, and the settings page trying the effects volume.
pub(crate) const KNOCK: &str = "audio/sfx/impact/impactWood_light_000.ogg";

/// The sound of a piece landing in the model, by what it is made of.
pub(super) fn landing_sound(tile: Tile) -> Option<&'static str> {
    match tile {
        Tile::Barricade => Some(KNOCK),
        Tile::Boue => Some("audio/sfx/impact/impactSoft_medium_000.ogg"),
        Tile::Ronces => Some("audio/sfx/foley/footstep_grass_000.ogg"),
        _ => None,
    }
}

/// How high a piece starts above its place, and how long it falls and settles, in seconds.
const DROP: f32 = 0.6;
const FALL: f32 = 0.16;
const SETTLE: f32 = 0.14;

/// A piece just inked: it drops into its cell like a pin pushed into the model, squashes a little
/// as it lands, and the land answers with a puff of fibres and the piece's sound.
#[derive(Component)]
pub(crate) struct Land {
    elapsed: f32,
    rest: Transform,
    sound: Option<&'static str>,
    landed: bool,
}

impl Land {
    /// Landing at `rest`, making `sound` as it does.
    pub(crate) fn new(rest: Transform, sound: Option<&'static str>) -> Self {
        Land { elapsed: 0.0, rest, sound, landed: false }
    }
}

/// A fibre of the model's ground, knocked up by a landing piece: it flies, falls and shrinks away.
#[derive(Component)]
pub(crate) struct Fibre {
    velocity: Vec3,
    life: f32,
}

/// What a landing makes use of: its sound, and the fibres' mesh and material, made once.
#[derive(SystemParam)]
pub(super) struct Landing<'w, 's> {
    assets: Res<'w, AssetServer>,
    settings: Res<'w, Settings>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<InkMaterial>>,
    fibre: Local<'s, Option<(Handle<Mesh>, Handle<InkMaterial>)>>,
}

impl Landing<'_, '_> {
    /// Plays `sound`, never twice quite the same pitch.
    fn play(&self, commands: &mut Commands, sound: &'static str, elapsed: f32) {
        let pitch = 0.88 + 0.24 * (elapsed * 7.31).fract();
        commands.spawn((
            AudioPlayer::new(self.assets.load(sound)),
            PlaybackSettings::DESPAWN.with_volume(self.settings.effects_volume()).with_speed(pitch),
        ));
    }

    /// Knocks up seven fibres at `at`, spread round from the angle `t`.
    fn puff(&mut self, commands: &mut Commands, at: Vec3, t: f32) {
        let (mesh, material) = self
            .fibre
            .get_or_insert_with(|| {
                // Earth-coloured, to show on the pale ground.
                let paper = look::fine(Color::srgb(0.55, 0.45, 0.33));
                (self.meshes.add(Cuboid::from_length(0.035)), self.materials.add(paper))
            })
            .clone();
        for k in 0..7 {
            let angle = k as f32 / 7.0 * 2.0 * PI + t;
            commands.spawn((
                Fibre { velocity: Vec3::new(angle.cos() * 1.1, 1.3, angle.sin() * 1.1), life: 1.0 },
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(at + Vec3::Y * 0.03)
                    .with_rotation(Quat::from_rotation_y(angle)),
            ));
        }
    }
}

pub(super) fn land(
    time: Res<Time>,
    mut pieces: Query<(Entity, &mut Land, &mut Transform, &GlobalTransform)>,
    mut commands: Commands,
    mut landing: Landing,
) {
    for (entity, mut land, mut transform, global) in &mut pieces {
        land.elapsed += time.delta_secs();
        let rest = land.rest;
        if land.elapsed < FALL {
            // Falling faster and faster.
            let k = land.elapsed / FALL;
            transform.translation = rest.translation + Vec3::Y * DROP * (1.0 - k * k);
            continue;
        }
        if !land.landed {
            land.landed = true;
            if let Some(sound) = land.sound {
                landing.play(&mut commands, sound, time.elapsed_secs());
            }
            landing.puff(&mut commands, global.translation(), land.elapsed);
        }
        // Squashed on landing, springing back.
        let s = 1.0 - ((land.elapsed - FALL) / SETTLE).min(1.0);
        let squash = Vec3::new(1.0 + 0.1 * s * s, 1.0 - 0.2 * s * s, 1.0 + 0.1 * s * s);
        transform.translation = rest.translation;
        transform.scale = rest.scale * squash;
        if s <= 0.0 {
            *transform = rest;
            commands.entity(entity).remove::<Land>();
        }
    }
}

pub(crate) fn fibres(
    time: Res<Time>,
    mut fibres: Query<(Entity, &mut Fibre, &mut Transform)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut fibre, mut transform) in &mut fibres {
        fibre.velocity.y -= 7.0 * dt;
        transform.translation += fibre.velocity * dt;
        fibre.life -= dt / 0.35;
        transform.scale = Vec3::splat(fibre.life.max(0.0));
        if fibre.life <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
