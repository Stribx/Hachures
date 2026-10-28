//! The wave's stamp across the middle of the sheet when one sets off.

use bevy::prelude::*;

use super::DuringPlay;
use crate::i18n::tf;
use crate::level::Level;
use crate::theme::{Face, SEAL, font};

/// The wave's stamp: it strikes at 108 % and settles, holds, and fades.
#[derive(Component)]
pub struct Stamp(f32);

const STRIKE: f32 = 0.2;
const HOLD: f32 = 1.2;
const FADE: f32 = 0.4;

/// A wave sets off: its number is stamped across the middle of the sheet.
pub fn stamp_wave(level: Res<Level>, mut stamped: Local<u32>, assets: Res<AssetServer>, mut commands: Commands) {
    let number = level.wave.number;
    if !level.wave.running || number == *stamped {
        if !level.wave.running && number < *stamped {
            // Another level, or the same one again.
            *stamped = number;
        }
        return;
    }
    *stamped = number;
    commands
        .spawn((
            DuringPlay,
            crate::level::LevelEntity,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                top: percent(32),
                justify_content: JustifyContent::Center,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|c| {
            c.spawn((
                Stamp(0.0),
                Text::new(tf("hud.wave_stamp", &[&number])),
                font(&assets, Face::Roman, 96.0),
                TextColor(SEAL.with_alpha(0.0)),
                UiTransform::IDENTITY,
            ));
        });
}

pub fn strike_stamps(
    time: Res<Time>,
    mut stamps: Query<(&mut Stamp, &mut TextColor, &mut UiTransform, &ChildOf)>,
    mut commands: Commands,
) {
    for (mut stamp, mut color, mut transform, parent) in &mut stamps {
        stamp.0 += time.delta_secs();
        let t = stamp.0;
        let strike = (t / STRIKE).min(1.0);
        let ease = 1.0 - (1.0 - strike).powi(3);
        transform.scale = Vec2::splat(1.08 - 0.08 * ease);
        let alpha = if t < STRIKE + HOLD { ease } else { 1.0 - ((t - STRIKE - HOLD) / FADE).min(1.0) };
        color.0 = SEAL.with_alpha(alpha * 0.9);
        if t > STRIKE + HOLD + FADE {
            commands.entity(parent.parent()).despawn();
        }
    }
}
