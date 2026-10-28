//! The page over the Grand Plan: its name and keys, and the card of the sheet Nève stands on.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::pawn::Pawn;
use super::plan::WalkTo;
use super::travel::Travel;
use crate::campaign::save::Save;
use crate::i18n::{t, tf};
use crate::level::Level;
use crate::level::data::LEVELS;
use crate::level::world::WORLDS;
use crate::schedule::Screen;
use crate::theme::{BODY_SIZE, HEADING_SIZE, INK, PAPER, SMALL_SIZE, SUBTITLE_SIZE};
use crate::ui::widgets::{Panel, framed, text};

/// The page over the Grand Plan: its name and keys, and the card of the sheet Nève stands on.
#[derive(Component)]
pub(super) struct Card(Option<usize>);

pub(super) fn spawn_page(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            DespawnOnExit(Screen::Plan),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(px(24)),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|page| {
            page.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::FlexStart,
                    padding: UiRect::axes(px(20), px(12)),
                    ..default()
                },
                Panel,
                BackgroundColor(PAPER),
            ))
            .with_children(|head| {
                head.spawn(text(&assets, t("plan.title"), HEADING_SIZE));
                head.spawn(text(&assets, t("plan.keys"), SMALL_SIZE));
            });
            page.spawn((
                Card(None),
                Node {
                    width: px(440),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    padding: UiRect::all(px(24)),
                    ..default()
                },
                Panel,
                BackgroundColor(PAPER),
            ));
        });
}

/// The card shows the sheet Nève stands on (or walks to).
pub(super) fn card(
    assets: Res<AssetServer>,
    save: Res<Save>,
    pawn: Res<Pawn>,
    mut cards: Query<(Entity, &mut Card)>,
    mut commands: Commands,
) {
    let Ok((entity, mut card)) = cards.single_mut() else { return };
    if card.0 == Some(pawn.node) {
        return;
    }
    card.0 = Some(pawn.node);
    let i = pawn.node;
    let level = Level::load(i, &save);
    let record = save.levels.get(i).cloned().unwrap_or_default();
    let most = 1 + level.challenge.is_some() as u32;
    let last = save.reached.min(LEVELS.len() - 1);
    commands.entity(entity).despawn_related::<Children>().with_children(|c| {
        c.spawn(text(&assets, t(&WORLDS[level.world].name), SMALL_SIZE))
            .insert(TextColor(INK.with_alpha(0.65)));
        c.spawn(text(&assets, t(&level.name), HEADING_SIZE));
        let earned = record.won as u32 + record.challenge as u32;
        c.spawn(text(&assets, tf("plan.seals", &[&earned, &most]), BODY_SIZE));
        for name in &record.erased {
            c.spawn(text(&assets, t(name), SMALL_SIZE))
                .insert((Strikethrough, TextColor(INK.with_alpha(0.6))));
        }
        c.spawn(Node { column_gap: px(10), margin: UiRect::top(px(10)), ..default() })
            .with_children(|row| {
                if i > 0 {
                    row.spawn(framed(0.0, text(&assets, "‹", SUBTITLE_SIZE))).observe(
                        move |_: On<Activate>, mut walks: MessageWriter<WalkTo>| {
                            walks.write(WalkTo(i - 1));
                        },
                    );
                }
                row.spawn(framed(200.0, text(&assets, t("plan.trace"), SUBTITLE_SIZE))).observe(
                    move |_: On<Activate>, mut travel: MessageWriter<Travel>| {
                        travel.write(Travel { to: i, again: false });
                    },
                );
                if i < last {
                    row.spawn(framed(0.0, text(&assets, "›", SUBTITLE_SIZE))).observe(
                        move |_: On<Activate>, mut walks: MessageWriter<WalkTo>| {
                            walks.write(WalkTo(i + 1));
                        },
                    );
                }
            });
    });
}
