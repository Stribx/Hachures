//! The Lexique's sheet on the entry picked: its picture, its name, what it is, and its numbers.

use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;

use super::{LexiqueView, beside_its_bar, picture, tint};
use crate::campaign::lexique::{EntryKind, entries, species_lines, tool_lines};
use crate::campaign::save::Save;
use crate::campaign::skills::Skill;
use crate::i18n::t;
use crate::level::Level;
use crate::theme::{BODY_SIZE, HEADING_SIZE, INK_SOFT, SMALL_SIZE};
use crate::ui::studio::{SpeciesIcons, ToolIcons};
use crate::ui::widgets::text;

/// A part of the sheet.
#[derive(Component, Clone, Copy)]
pub(super) enum Detail {
    Name,
    Text,
    Lines,
}

/// The sheet itself: a column of fixed size that scrolls when the entry says more than it holds.
#[derive(Component)]
pub(super) struct Sheet;

/// The sheet's picture of the entry picked, large.
#[derive(Component)]
pub(super) struct SheetPicture;

/// Side of the sheet's picture, in pixels.
const PICTURE: f32 = 200.0;

pub(super) fn spawn(row: &mut ChildSpawnerCommands, assets: &AssetServer) {
    let area = (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            width: px(520),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        ScrollArea,
        Sheet,
    );
    let sheet = beside_its_bar(row, area);
    row.commands().entity(sheet).with_children(|sheet| {
        sheet.spawn((
            // Its full size even when the words below overflow the sheet.
            Node { width: px(PICTURE), height: px(PICTURE), flex_shrink: 0.0, ..default() },
            ImageNode::default(),
            SheetPicture,
        ));
        sheet.spawn((text(assets, "", HEADING_SIZE), Detail::Name));
        sheet.spawn((text(assets, "", BODY_SIZE), Detail::Text));
        sheet.spawn((text(assets, "", SMALL_SIZE), Detail::Lines)).insert(TextColor(INK_SOFT));
    });
}

/// The three parts for the entry picked; an entry not yet met is only said to be so.
fn words(view: &LexiqueView, save: &Save, level: &Level) -> [String; 3] {
    let Some(entry) = entries(view.tab, &level.species).into_iter().nth(view.entry) else {
        return Default::default();
    };
    if !save.seen.contains(&entry.id) {
        return [t("lexique.locked").into(), t("lexique.locked_text").into(), String::new()];
    }
    let lines = match entry.kind {
        EntryKind::Tool(tool) => {
            // The prices of a species not yet met would give it away.
            let met: Vec<_> =
                level.species.iter().filter(|s| save.seen.contains(&s.label)).cloned().collect();
            tool_lines(
                tool,
                level.tool_cost(tool),
                &level.tower_stats,
                &met,
                level.has(Skill::Equerre),
            )
        }
        EntryKind::Species(s) => species_lines(&level.species[s]),
    };
    let lines: Vec<String> = lines.iter().map(|l| l.shown()).collect();
    [t(&entry.id).into(), t(&entry.text()).into(), lines.join("\n")]
}

pub(super) fn show(
    view: Res<LexiqueView>,
    save: Res<Save>,
    level: Res<Level>,
    (tools, species): (Res<ToolIcons>, Res<SpeciesIcons>),
    mut parts: Query<(&Detail, &mut Text)>,
    mut shown: Single<&mut ImageNode, With<SheetPicture>>,
    mut scroll: Single<&mut ScrollPosition, With<Sheet>>,
) {
    // Another entry reads from its top.
    if view.is_changed() && scroll.y != 0.0 {
        scroll.y = 0.0;
    }
    let words = words(&view, &save, &level);
    for (part, mut text) in &mut parts {
        let new = &words[*part as usize];
        if text.0 != *new {
            text.0 = new.clone();
        }
    }
    let Some(entry) = entries(view.tab, &level.species).into_iter().nth(view.entry) else { return };
    let image = picture(entry.kind, &tools, &species);
    if shown.image != image {
        shown.image = image;
    }
    let color = tint(save.seen.contains(&entry.id));
    if shown.color != color {
        shown.color = color;
    }
}
