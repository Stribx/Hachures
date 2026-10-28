//! The cartouche in the top left corner: the level's name and its counters, the Défi, what the
//! world is about to do, and what the cursor reads. The keys are on the bar's
//! buttons, the end of the level on its page.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;

use super::DuringPlay;
use super::readout::{notice, readout};
use super::widgets::{IconSlot, Panel, Skin, tooltip};
use crate::i18n::{t, tf};
use crate::ink::{Selection, Verdict};
use crate::level::Level;
use crate::theme::{Face, INK, INK_SOFT, PAPER, SEAL, SMALL_SIZE, SUBTITLE_SIZE, font};

/// `text` with a capital first letter: "le Moulin" → "Le Moulin".
pub fn capitalized(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |c| c.to_uppercase().chain(chars).collect())
}

/// The four counters, in their order on the cartouche.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Counter {
    Sepia,
    Cible,
    Noms,
    Vague,
}

const COUNTERS: [Counter; 4] = [Counter::Sepia, Counter::Cible, Counter::Noms, Counter::Vague];

impl Counter {
    fn icon(self) -> &'static str {
        match self {
            Counter::Sepia => "ui/icon-sepia.png",
            Counter::Cible => "ui/icon-cible.png",
            Counter::Noms => "ui/icon-noms.png",
            Counter::Vague => "ui/icon-vague.png",
        }
    }
}

/// A counter's figure, after its icon (or after its word, until the icon is drawn).
#[derive(Component)]
pub struct Figure(Counter);

/// A counter's icon.
#[derive(Component)]
pub struct Icon(Counter);

/// A counter's icon and figure together: what the cursor hovers to learn its name.
#[derive(Component)]
pub struct Tip(pub(super) Counter);

/// The words of the hovered counter, next to the cursor.
#[derive(Component)]
pub(super) struct CounterTip;

#[derive(Component)]
pub enum Line {
    Name,
    Defi,
    /// The world event coming or under way, the boss's phase: in the seal's red.
    Notice,
    /// What the cursor reads: the pencil line's cost, le Calque.
    Readout,
}

pub fn spawn(commands: &mut Commands, assets: &AssetServer) {
    commands.spawn((CounterTip, tooltip(assets)));
    commands
        .spawn((
            DuringPlay,
            Visibility::Hidden,
            Panel,
            BackgroundColor(PAPER),
            Node {
                position_type: PositionType::Absolute,
                top: px(16),
                left: px(16),
                max_width: px(620),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                padding: UiRect::axes(px(20), px(14)),
                ..default()
            },
        ))
        .with_children(|c| {
            c.spawn((
                Line::Name,
                Text::default(),
                font(assets, Face::Roman, SUBTITLE_SIZE),
                TextColor(INK),
            ));
            c.spawn(Node { column_gap: px(22), align_items: AlignItems::Center, ..default() })
                .with_children(|row| spawn_counters(row, assets));
            c.spawn((
                Line::Defi,
                Text::default(),
                font(assets, Face::Italic, SMALL_SIZE),
                TextColor(INK_SOFT),
            ));
            c.spawn((
                Line::Notice,
                Text::default(),
                font(assets, Face::Italic, SMALL_SIZE),
                TextColor(SEAL),
            ));
            c.spawn((
                Line::Readout,
                Text::default(),
                font(assets, Face::Roman, SMALL_SIZE),
                TextColor(INK_SOFT),
            ));
        });
}

/// The four counters, each an icon and its figure, in a row.
fn spawn_counters(row: &mut ChildSpawnerCommands, assets: &AssetServer) {
    for counter in COUNTERS {
        row.spawn((
            Tip(counter),
            Hovered::default(),
            Node { column_gap: px(6), align_items: AlignItems::Center, ..default() },
        ))
        .with_children(|pair| {
            pair.spawn((
                Icon(counter),
                IconSlot,
                Node { width: px(28), height: px(28), ..default() },
            ))
            .insert(Skin { image: assets.load(counter.icon()), slice: BorderRect::ZERO });
            pair.spawn((
                Figure(counter),
                Text::default(),
                font(assets, Face::Roman, SUBTITLE_SIZE),
                TextColor(INK),
                TextLayout::no_wrap(),
            ));
        });
    }
}

pub fn update_label(
    level: Res<Level>,
    selection: Res<Selection>,
    verdict: Res<Verdict>,
    icons: Query<(&Icon, Has<ImageNode>)>,
    mut figures: Query<(&Figure, &mut Text), Without<Line>>,
    mut lines: Query<(&Line, &mut Text, &mut Node)>,
) {
    let defends = capitalized(t(&level.world().defends));
    for (Figure(counter), mut text) in &mut figures {
        let drawn = icons.iter().any(|(icon, drawn)| icon.0 == *counter && drawn);
        // Lives are the defended place's, not the Rôle des Noms: two counters, two words.
        let (word, value) = match counter {
            Counter::Sepia => (t("hud.sepia").to_string(), level.sepia.to_string()),
            Counter::Cible => (defends.clone(), level.lives.to_string()),
            Counter::Noms => (t("hud.names").to_string(), level.names.to_string()),
            Counter::Vague => (
                t("hud.wave").to_string(),
                format!("{} / {}", level.wave.number, level.waves.len()),
            ),
        };
        text.set_if_neq(Text(if drawn { value } else { format!("{word} {value}") }));
    }
    for (line, mut text, mut node) in &mut lines {
        let words = match line {
            Line::Name => t(&level.name).to_string(),
            Line::Defi => level
                .challenge
                .as_ref()
                .map(|c| tf("hud.challenge", &[&t(&c.text)]))
                .unwrap_or_default(),
            Line::Notice => notice(&level),
            Line::Readout => readout(&level, &selection, &verdict),
        };
        // An empty line takes no room.
        let display = if words.is_empty() { Display::None } else { Display::Flex };
        if node.display != display {
            node.display = display;
        }
        text.set_if_neq(Text(words));
    }
}
