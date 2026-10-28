//! The bar along the bottom: the level's tools, the next wave, le Calque, the route shown, the sceaux and the Lexique.
//! Everything can be played with the mouse; each button names its key.

use bevy::ecs::system::SystemParam;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use super::DuringPlay;
use super::keys::{Fast, next_species};
use super::studio::ToolIcons;
use super::widgets::{Panel, SLIP_RADIUS, framed, primary, text};
use crate::combat::enemies::send_wave;
use crate::i18n::{t, tf};
use crate::ink::{Act, Selection, TOOLS};
use crate::level::{Level, LevelEntity};
use crate::menu::{open_lexique, open_sceaux};
use crate::schedule::Screen;
use crate::settings::Settings;
use crate::theme::{BODY_SIZE, INK, INK_SOFT, PAPER, SEAL, SLIP, SMALL_SIZE};

#[derive(Component)]
pub struct Bar;

/// The button of `TOOLS[.0]`.
#[derive(Component)]
pub struct ToolButton(pub usize);

/// The name of `TOOLS[.0]`.
#[derive(Component)]
pub struct ToolLabel(usize);

/// The cost of `TOOLS[.0]`, in sépia.
#[derive(Component)]
pub struct ToolCost(usize);

/// The label naming the species whose route is drawn.
#[derive(Component)]
pub struct RouteLabel;

/// The label of the speed button.
#[derive(Component)]
pub struct SpeedLabel;

type SpeedText = (With<SpeedLabel>, Without<ToolCost>, Without<RouteLabel>);

/// Writes the bar for the level being played, and takes it away once the level is over (its page takes over).
pub fn sync(
    mut commands: Commands,
    assets: Res<AssetServer>,
    icons: Res<ToolIcons>,
    level: Res<Level>,
    bar: Query<Entity, With<Bar>>,
) {
    let over = level.finished || level.lives == 0;
    match (bar.single(), over) {
        (Ok(bar), true) => commands.entity(bar).despawn(),
        (Err(_), false) => spawn(&mut commands, &assets, &icons, &level),
        _ => {}
    }
}

fn spawn(commands: &mut Commands, assets: &AssetServer, icons: &ToolIcons, level: &Level) {
    commands
        .spawn((
            Bar,
            LevelEntity,
            DuringPlay,
            // The map under the bar is not inked by a click on it.
            Hovered::default(),
            Panel,
            BackgroundColor(PAPER),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(16),
                left: px(16),
                right: px(16),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                row_gap: px(8),
                padding: UiRect::axes(px(14), px(10)),
                ..default()
            },
        ))
        .with_children(|bar| {
            spawn_tools(bar, assets, icons, level);
            spawn_actions(bar, assets);
        });
}

/// The tools: a card each, its key in the corner, its cost under its name.
fn spawn_tools(bar: &mut ChildSpawnerCommands, assets: &AssetServer, icons: &ToolIcons, level: &Level) {
    bar.spawn(Node { column_gap: px(8), ..default() }).with_children(|tools| {
        for &tool in &level.tools {
            spawn_card(tools, assets, icons, tool);
        }
    });
}

/// The card of `TOOLS[index]`.
fn spawn_card(tools: &mut ChildSpawnerCommands, assets: &AssetServer, icons: &ToolIcons, index: usize) {
    let tool = &TOOLS[index];
    tools
        .spawn((
            Button,
            Hovered::default(),
            ToolButton(index),
            Outline::default(),
            UiTransform::IDENTITY,
            BackgroundColor(SLIP),
            Node {
                width: px(92),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                // Room at the top for the key.
                padding: UiRect::new(px(4), px(4), px(20), px(6)),
                border_radius: BorderRadius::all(px(SLIP_RADIUS)),
                ..default()
            },
        ))
        .observe(move |_: On<Activate>, mut selection: ResMut<Selection>| selection.tool = index)
        .with_children(|card| {
            card.spawn((
                text(assets, (index + 1).to_string(), SMALL_SIZE),
                Node { position_type: PositionType::Absolute, top: px(2), left: px(6), ..default() },
            ))
            .insert(TextColor(INK_SOFT));
            // The piece itself, filmed in the studio.
            card.spawn((Node { width: px(48), height: px(48), ..default() }, ImageNode::new(icons.0[index].clone())));
            card.spawn((text(assets, t(tool.label), SMALL_SIZE), ToolLabel(index)));
            card.spawn((text(assets, "", SMALL_SIZE), ToolCost(index)));
        });
}

/// What the turn asks: the next wave first, in the seal's red.
fn spawn_actions(bar: &mut ChildSpawnerCommands, assets: &AssetServer) {
    let small = |s: &str| text(assets, s.to_string(), SMALL_SIZE);
    bar.spawn(Node { column_gap: px(8), align_items: AlignItems::Center, ..default() }).with_children(|actions| {
        actions.spawn(primary(0.0, text(assets, t("bar.send_wave"), BODY_SIZE))).observe(
            |_: On<Activate>,
             mut level: ResMut<Level>,
             mut commands: Commands,
             assets: Res<AssetServer>,
             settings: Res<Settings>| { send_wave(&mut level, &mut commands, &assets, &settings) },
        );
        actions
            .spawn(framed(0.0, (small(""), SpeedLabel)))
            .observe(|_: On<Activate>, mut fast: ResMut<Fast>| fast.0 = !fast.0);
        actions
            .spawn(framed(0.0, small(t("bar.calque"))))
            .observe(|_: On<Activate>, mut selection: ResMut<Selection>| selection.calque = !selection.calque);
        actions.spawn(framed(0.0, (small(""), RouteLabel))).observe(
            |_: On<Activate>, level: Res<Level>, mut selection: ResMut<Selection>| next_species(&mut selection, &level),
        );
        actions.spawn(framed(0.0, small(t("bar.sceaux")))).observe(open_sceaux(Screen::Playing));
        actions.spawn(framed(0.0, small(t("bar.lexique")))).observe(open_lexique(Screen::Playing));
    });
}

/// What the bar reads: the level, what is picked, the speed.
#[derive(SystemParam)]
pub struct Hand<'w> {
    level: Res<'w, Level>,
    selection: Res<'w, Selection>,
    fast: Res<'w, Fast>,
}

/// The tool in hand is ringed in ink; tools out of sépia fade, their cost in red. Costs follow
/// the skills.
pub fn look(
    hand: Hand,
    mut buttons: Query<(&ToolButton, &mut Outline)>,
    mut labels: Query<(&ToolLabel, &mut TextColor)>,
    mut costs: Query<(&ToolCost, &mut Text, &mut TextColor), Without<ToolLabel>>,
    mut route: Query<&mut Text, (With<RouteLabel>, Without<ToolCost>)>,
    mut speed: Query<&mut Text, SpeedText>,
) {
    let Hand { level, selection, fast } = hand;
    for (&ToolButton(t), mut outline) in &mut buttons {
        let color = if t == selection.tool { INK } else { Color::NONE };
        *outline = Outline::new(px(2), px(3), color);
    }
    for (&ToolLabel(t), mut color) in &mut labels {
        color.0 = if level.sepia >= level.tool_cost(t) { INK } else { INK.with_alpha(0.4) };
    }
    for (&ToolCost(tool), mut text, mut color) in &mut costs {
        let cost = level.tool_cost(tool);
        text.0 = if TOOLS[tool].act == Act::Biffer { t("bar.refund").to_string() } else { tf("bar.cost", &[&cost]) };
        color.0 = if level.sepia >= cost { INK_SOFT } else { SEAL };
    }
    for mut text in &mut route {
        text.0 = tf("bar.route", &[&t(&level.species[selection.shown].label)]);
    }
    for mut text in &mut speed {
        text.0 = tf("bar.speed", &[&if fast.0 { "×2" } else { "×1" }]);
    }
}
