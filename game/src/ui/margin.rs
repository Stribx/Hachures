//! Nève's margin: the strip of notes down the right side, the newest in fresh ink.

use bevy::prelude::*;

use crate::level::Level;
use crate::theme::{Face, INK, SMALL_SIZE, font};

/// Nève's notebook, down the right side: a strip of paper with her latest notes.
#[derive(Component)]
pub(super) struct Margin;

/// A note just written: its ink comes up (300 ms).
#[derive(Component)]
pub(super) struct Fresh(f32);

/// The latest notes of Nève's notebook, the newest in full ink, the older ones fading back.
pub(super) fn update_margin(
    level: Res<Level>,
    assets: Res<AssetServer>,
    mut written: Local<Option<(usize, usize, &'static str)>>,
    margin: Single<(Entity, &mut Node), With<Margin>>,
    mut commands: Commands,
) {
    const SHOWN: usize = 5;
    // Written again in another language too.
    let now = Some((level.index, level.margin.len(), crate::i18n::code()));
    if *written == now {
        return;
    }
    let grew =
        written.is_some_and(|(index, len, _)| index == level.index && len < level.margin.len());
    *written = now;
    let (margin, mut node) = margin.into_inner();
    node.display = if level.margin.is_empty() { Display::None } else { Display::Flex };
    let recent = &level.margin[level.margin.len().saturating_sub(SHOWN)..];
    let font = font(&assets, Face::Italic, SMALL_SIZE);
    commands.entity(margin).despawn_related::<Children>().with_children(|m| {
        for (k, note) in recent.iter().enumerate() {
            let newest = k + 1 == recent.len();
            let mut line = m.spawn((
                Text::new(note.shown()),
                font.clone(),
                TextColor(INK.with_alpha(if newest { 1.0 } else { 0.55 })),
            ));
            if newest && grew {
                line.insert((Fresh(0.0), TextColor(INK.with_alpha(0.0))));
            }
        }
    });
}

pub(super) fn fresh_ink(
    time: Res<Time>,
    mut notes: Query<(Entity, &mut Fresh, &mut TextColor)>,
    mut commands: Commands,
) {
    for (entity, mut fresh, mut color) in &mut notes {
        fresh.0 = (fresh.0 + time.delta_secs() / 0.3).min(1.0);
        color.0 = INK.with_alpha(1.0 - (1.0 - fresh.0).powi(3));
        if fresh.0 >= 1.0 {
            commands.entity(entity).remove::<Fresh>();
        }
    }
}
