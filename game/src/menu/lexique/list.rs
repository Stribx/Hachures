//! The Lexique's tabs and the list of the current tab's entries.

use bevy::input_focus::InputFocus;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::LexiqueView;
use crate::campaign::lexique::{TABS, Tab, entries};
use crate::campaign::save::Save;
use crate::i18n::t;
use crate::level::Level;
use crate::menu::label;
use crate::theme::{INK, SMALL_SIZE};
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

/// A row of tab buttons: choosing one shows its entries, from the first.
pub(super) fn spawn_tabs(page: &mut ChildSpawnerCommands, assets: &AssetServer) {
    page.spawn(Node { column_gap: px(12), ..default() }).with_children(|tabs| {
        for &(tab, name) in &TABS {
            tabs.spawn((framed(160.0, label(assets, t(name))), TabButton(tab), Outline::default()))
                .observe(move |_: On<Activate>, mut view: ResMut<LexiqueView>| *view = LexiqueView { tab, entry: 0 });
        }
    });
}

pub(super) fn spawn_list(row: &mut ChildSpawnerCommands) {
    row.spawn((
        Node { flex_direction: FlexDirection::Column, row_gap: px(3), width: px(300), ..default() },
        EntryList::default(),
    ));
}

/// Draws the rows again when the tab is not the one they were drawn for (tabs differ in length).
pub(super) fn rebuild(
    mut commands: Commands,
    assets: Res<AssetServer>,
    view: Res<LexiqueView>,
    level: Res<Level>,
    mut list: Single<(Entity, &mut EntryList)>,
) {
    let (list, drawn) = &mut *list;
    if drawn.0 == Some(view.tab) {
        return;
    }
    drawn.0 = Some(view.tab);
    let count = entries(view.tab, &level.species).len();
    commands.entity(*list).despawn_children().with_children(|rows| {
        for i in 0..count {
            rows.spawn((framed(0.0, text(&assets, "", SMALL_SIZE)), EntryRow(i), Outline::default()))
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

/// Names the rows (an entry not yet met is `???`, faded), and rings the tab shown and the entry picked.
pub(super) fn show(
    view: Res<LexiqueView>,
    save: Res<Save>,
    level: Res<Level>,
    mut tabs: Query<(&TabButton, &mut Outline), Without<EntryRow>>,
    mut rows: Query<(&EntryRow, &Children, &mut Outline), Without<TabButton>>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
) {
    for (tab, mut outline) in &mut tabs {
        *outline = ring(tab.0 == view.tab);
    }
    let entries = entries(view.tab, &level.species);
    for (&EntryRow(i), children, mut outline) in &mut rows {
        *outline = ring(i == view.entry);
        let Some(entry) = entries.get(i) else { continue };
        let Some(&child) = children.first() else { continue };
        let Ok((mut text, mut color)) = texts.get_mut(child) else { continue };
        let seen = save.seen.contains(&entry.id);
        let (words, ink) = if seen { (t(&entry.id), INK) } else { (t("lexique.locked"), INK.with_alpha(0.4)) };
        if text.0 != words {
            text.0 = words.to_string();
        }
        color.0 = ink;
    }
}

fn ring(on: bool) -> Outline {
    Outline::new(px(2), px(3), if on { INK } else { Color::NONE })
}
