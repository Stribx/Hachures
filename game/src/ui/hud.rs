//! The cartouche in the top left corner: the level's name and its counters, the Défi, what the world is about to
//! do, and what the cursor reads. The keys are on the bar's
//! buttons, the end of the level on its page.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::DuringPlay;
use super::widgets::{IconSlot, Panel, Skin, follow_cursor, tooltip};
use crate::campaign::skills::Skill;
use crate::i18n::{t, tf};
use crate::ink::Selection;
use crate::level::Level;
use crate::theme::{INK, INK_SOFT, PAPER, SEAL, SMALL_SIZE, SUBTITLE_SIZE, font};

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
pub struct Tip(Counter);

/// The words of the hovered counter, next to the cursor.
#[derive(Component)]
pub struct CounterTip;

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
            c.spawn((Line::Name, Text::default(), font(assets, false, SUBTITLE_SIZE), TextColor(INK)));
            c.spawn(Node { column_gap: px(22), align_items: AlignItems::Center, ..default() }).with_children(|row| {
                for counter in COUNTERS {
                    row.spawn((
                        Tip(counter),
                        Hovered::default(),
                        Node { column_gap: px(6), align_items: AlignItems::Center, ..default() },
                    ))
                    .with_children(|pair| {
                        pair.spawn((Icon(counter), IconSlot, Node { width: px(28), height: px(28), ..default() }))
                            .insert(Skin(assets.load(counter.icon()), BorderRect::ZERO));
                        pair.spawn((
                            Figure(counter),
                            Text::default(),
                            font(assets, false, SUBTITLE_SIZE),
                            TextColor(INK),
                            TextLayout::no_wrap(),
                        ));
                    });
                }
            });
            c.spawn((Line::Defi, Text::default(), font(assets, true, SMALL_SIZE), TextColor(INK_SOFT)));
            c.spawn((Line::Notice, Text::default(), font(assets, true, SMALL_SIZE), TextColor(SEAL)));
            c.spawn((Line::Readout, Text::default(), font(assets, false, SMALL_SIZE), TextColor(INK_SOFT)));
        });
}

pub fn update_label(
    level: Res<Level>,
    selection: Res<Selection>,
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
            Counter::Vague => (t("hud.wave").to_string(), format!("{} / {}", level.wave.number, level.waves.len())),
        };
        text.0 = if drawn { value } else { format!("{word} {value}") };
    }
    for (line, mut text, mut node) in &mut lines {
        text.0 = match line {
            Line::Name => t(&level.name).to_string(),
            Line::Defi => level.challenge.as_ref().map(|c| tf("hud.challenge", &[&t(&c.text)])).unwrap_or_default(),
            Line::Notice => notice(&level),
            Line::Readout => readout(&level, &selection),
        };
        // An empty line takes no room.
        node.display = if text.0.is_empty() { Display::None } else { Display::Flex };
    }
}

/// What a counter is, for a player who has only the icon and the number.
fn tip_text(counter: Counter, level: &Level) -> String {
    match counter {
        Counter::Sepia => t("hud.tip.sepia").to_string(),
        Counter::Cible => tf("hud.tip.cible", &[&capitalized(t(&level.world().defends))]),
        Counter::Noms => t("hud.tip.names").to_string(),
        Counter::Vague => t("hud.tip.wave").to_string(),
    }
}

/// Names the counter under the cursor, next to it.
pub fn counter_tip(
    level: Res<Level>,
    window: Single<&Window, With<PrimaryWindow>>,
    pairs: Query<(&Tip, &Hovered)>,
    tip: Single<(&mut Text, &mut Node, &mut Visibility), With<CounterTip>>,
) {
    let (mut text, mut node, mut visibility) = tip.into_inner();
    let hovered = pairs.iter().find(|(_, hovered)| hovered.get()).map(|(Tip(counter), _)| *counter);
    let (Some(counter), Some(cursor)) = (hovered, window.cursor_position()) else {
        *visibility = Visibility::Hidden;
        return;
    };
    text.0 = tip_text(counter, &level);
    follow_cursor(&mut node, cursor);
    *visibility = Visibility::Inherited;
}

fn notice(level: &Level) -> String {
    let event = &level.world().event;
    let mut lines = Vec::new();
    if let Some(phase) = &level.boss {
        lines.push(tf("hud.boss_phase", &[&t(&level.boss_file().name), &t(phase)]));
    }
    if level.event_on {
        lines.push(t(&event.during).to_string());
    } else if level.event_next {
        lines.push(t(&event.announce).to_string());
    }
    lines.join("\n")
}

fn readout(level: &Level, selection: &Selection) -> String {
    let Some(i) = selection.hovered.filter(|_| !level.finished) else { return String::new() };
    let mut lines = Vec::new();
    // Lecture fine: what the pencil line costs the shown species, before and after.
    if level.has(Skill::LectureFine) && level.refusal(i, selection.tool).is_none() {
        let (what_if, field) = level.what_if(i, selection.tool, selection.shown);
        if let Some(&spawn) = what_if.spawns_for(&level.species[selection.shown]).first() {
            let (before, after) = (level.terrain.fields()[selection.shown].dist[spawn], field.dist[spawn]);
            lines.push(tf("hud.line_cost", &[&before, &after]));
        }
    }
    if selection.calque {
        let tile = level.terrain.grid().tiles[i];
        let costs: Vec<String> = level
            .present
            .iter()
            .map(|&k| {
                let (name, cost, goal) =
                    (t(&level.species[k].label), level.species[k].cost(tile), level.terrain.fields()[k].dist[i]);
                tf("hud.calque_species", &[&name, &cost, &goal])
            })
            .collect();
        lines.push(tf("hud.calque", &[&t(tile.name()), &costs.join(", ")]));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::grid::Grid;
    use crate::map::species;

    #[test]
    fn a_hovered_counter_is_named_at_the_cursor() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(Level::new(Grid::parse("S.M"), species::load()));
        let mut window = Window::default();
        window.set_cursor_position(Some(Vec2::new(100.0, 50.0)));
        app.world_mut().spawn((window, PrimaryWindow));
        let tip = app.world_mut().spawn((CounterTip, Text::default(), Node::default(), Visibility::Hidden)).id();
        let pair = app.world_mut().spawn((Tip(Counter::Sepia), Hovered(true))).id();
        app.add_systems(Update, counter_tip);

        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Inherited);
        assert_eq!(app.world().get::<Text>(tip).unwrap().0, t("hud.tip.sepia"));
        assert_eq!(app.world().get::<Node>(tip).unwrap().left, px(118));

        app.world_mut().entity_mut(pair).insert(Hovered(false));
        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Hidden);
    }
}
