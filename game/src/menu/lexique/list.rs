//! The Lexique's tabs and the list of the current tab's entries.

use bevy::input_focus::InputFocus;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, ScrollArea};

use super::{LexiqueView, beside_its_bar, picture, tint};
use crate::campaign::lexique::{Tab, entries};
use crate::campaign::save::Save;
use crate::i18n::t;
use crate::level::Level;
use crate::menu::label;
use crate::theme::{INK, SMALL_SIZE};
use crate::ui::studio::{SpeciesIcons, ToolIcons};
use crate::ui::widgets::{framed, text};

/// The button of a tab.
#[derive(Component)]
pub(super) struct TabButton(Tab);

/// The column of entries, and the tab they were drawn for.
#[derive(Component, Default)]
pub(super) struct EntryList(Option<Tab>);

/// The button of entry `.0` of the current tab.
#[derive(Component)]
pub(super) struct EntryRow(pub usize);

/// The name on row `.0`.
#[derive(Component)]
pub(super) struct RowName(usize);

/// The picture on row `.0`.
#[derive(Component)]
pub(super) struct RowPicture(usize);

/// Side of a row's picture, in pixels.
const THUMBNAIL: f32 = 30.0;

/// A row of tab buttons: choosing one shows its entries, from the first.
pub(super) fn spawn_tabs(page: &mut ChildSpawnerCommands, assets: &AssetServer) {
    page.spawn(Node { column_gap: px(12), ..default() }).with_children(|tabs| {
        for tab in Tab::ALL {
            tabs.spawn((
                framed(160.0, label(assets, t(tab.name()))),
                TabButton(tab),
                Outline::default(),
            ))
            .observe(move |_: On<Activate>, mut view: ResMut<LexiqueView>| {
                *view = LexiqueView { tab, entry: 0 }
            });
        }
    });
}

pub(super) fn spawn_list(row: &mut ChildSpawnerCommands) {
    beside_its_bar(
        row,
        (
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(3),
                width: px(300),
                // Room for the ring of the row picked (`ring`), which the scroll would cut.
                padding: UiRect::all(px(5)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            ScrollArea,
            EntryList::default(),
        ),
    );
}

/// Draws the rows again when the tab is not the one they were drawn for (tabs differ in length):
/// each the entry's picture, then its name.
pub(super) fn rebuild(
    mut commands: Commands,
    assets: Res<AssetServer>,
    view: Res<LexiqueView>,
    level: Res<Level>,
    (tools, species): (Res<ToolIcons>, Res<SpeciesIcons>),
    mut list: Single<(Entity, &mut EntryList)>,
) {
    let (list, drawn) = &mut *list;
    if drawn.0 == Some(view.tab) {
        return;
    }
    drawn.0 = Some(view.tab);
    let entries = entries(view.tab, &level.species);
    commands.entity(*list).despawn_children().with_children(|rows| {
        for (i, entry) in entries.iter().enumerate() {
            let thumbnail = (
                Node { width: px(THUMBNAIL), height: px(THUMBNAIL), ..default() },
                ImageNode::new(picture(entry.kind, &tools, &species)),
                RowPicture(i),
            );
            let name = (text(&assets, "", SMALL_SIZE), RowName(i));
            let content = (
                Node {
                    width: percent(100),
                    column_gap: px(10),
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![thumbnail, name],
            );
            rows.spawn((framed(0.0, content), EntryRow(i), Outline::default()))
                .observe(move |_: On<Activate>, mut view: ResMut<LexiqueView>| view.entry = i);
        }
    });
}

/// Pointing at a row or moving the keyboard focus onto it picks it, as a click does.
pub(super) fn pick(
    focus: Res<InputFocus>,
    rows: Query<&EntryRow>,
    moved: Query<(&EntryRow, &Hovered), Changed<Hovered>>,
    mut view: ResMut<LexiqueView>,
) {
    for (row, hovered) in &moved {
        if hovered.get() {
            view.entry = row.0;
        }
    }
    if focus.is_changed()
        && let Some(row) = focus.get().and_then(|f| rows.get(f).ok())
    {
        view.entry = row.0;
    }
}

/// Names the rows (an entry not yet met is `???`, faded, its picture a silhouette), and rings the
/// tab shown and the entry picked.
pub(super) fn show(
    view: Res<LexiqueView>,
    save: Res<Save>,
    level: Res<Level>,
    mut tabs: Query<(&TabButton, &mut Outline), Without<EntryRow>>,
    mut rows: Query<(&EntryRow, &mut Outline), Without<TabButton>>,
    mut names: Query<(&RowName, &mut Text, &mut TextColor)>,
    mut pictures: Query<(&RowPicture, &mut ImageNode)>,
) {
    for (tab, mut outline) in &mut tabs {
        *outline = ring(tab.0 == view.tab);
    }
    for (&EntryRow(i), mut outline) in &mut rows {
        *outline = ring(i == view.entry);
    }
    let entries = entries(view.tab, &level.species);
    let seen = |i: usize| entries.get(i).map(|e| (e, save.seen.contains(&e.id)));
    for (&RowName(i), mut text, mut color) in &mut names {
        let Some((entry, seen)) = seen(i) else { continue };
        let (words, ink) =
            if seen { (t(&entry.id), INK) } else { (t("lexique.locked"), INK.with_alpha(0.4)) };
        if text.0 != words {
            text.0 = words.to_string();
        }
        color.0 = ink;
    }
    for (&RowPicture(i), mut image) in &mut pictures {
        let Some((_, seen)) = seen(i) else { continue };
        if image.color != tint(seen) {
            image.color = tint(seen);
        }
    }
}

fn ring(on: bool) -> Outline {
    Outline::new(px(2), px(3), if on { INK } else { Color::NONE })
}
