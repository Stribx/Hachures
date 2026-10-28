//! What the player reads and clicks: the label, Nève's margin, the bar of tools, the names on the sheet,
//! and the page at the end of a level.

pub mod bar;
mod enemy_card;
mod hud;
mod notebook;
mod places;
mod stamp;
mod studio;
mod tool_tip;
pub(crate) mod widgets;

use bevy::input::keyboard::Key;
use bevy::prelude::*;

use crate::Step;
use crate::ink::Selection;
use crate::level::Level;
use crate::schedule::Screen;
use crate::theme::{INK, PAPER, SMALL_SIZE, font};
use widgets::{Panel, letter};

pub fn plugin(app: &mut App) {
    app.add_plugins((studio::plugin, widgets::plugin));
    app.init_resource::<Fast>()
        .add_systems(Startup, (setup, places::setup, tool_tip::setup, enemy_card::setup))
        .add_systems(
            Update,
            (
                toggles.in_set(Step::Input).before(crate::overworld::onward::between_levels),
                (hud::update_label, hud::counter_tip, update_margin, fresh_ink, places::label_places, places::tooltip)
                    .in_set(Step::Render),
                // The card of an enemy gives way to the tip of a tool, and covers the price of the cell.
                (tool_tip::tool_tip, enemy_card::enemy_card)
                    .chain()
                    .after(places::tooltip)
                    .in_set(Step::Render)
                    .run_if(in_state(Screen::Playing)),
                (stamp::stamp_wave, stamp::strike_stamps).in_set(Step::Render),
                (bar::sync, bar::look, notebook::sync).in_set(Step::Render).run_if(in_state(Screen::Playing)),
                show_during_play.run_if(state_changed::<Screen>),
                speed,
            ),
        );
}

/// The label and the margin only show during play, not behind the menus.
fn show_during_play(screen: Res<State<Screen>>, mut texts: Query<&mut Visibility, With<DuringPlay>>) {
    let shown = if *screen.get() == Screen::Playing { Visibility::Inherited } else { Visibility::Hidden };
    for mut v in &mut texts {
        *v = shown;
    }
}

/// Hidden behind the menus.
#[derive(Component)]
struct DuringPlay;

/// Nève's notebook, down the right side: a strip of paper with her latest notes.
#[derive(Component)]
struct Margin;

/// A note just written: its ink comes up (300 ms).
#[derive(Component)]
struct Fresh(f32);

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    hud::spawn(&mut commands, &assets);
    commands.spawn((
        Margin,
        DuringPlay,
        Visibility::Hidden,
        Panel,
        BackgroundColor(PAPER),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            right: px(16),
            width: px(360),
            flex_direction: FlexDirection::Column,
            row_gap: px(12),
            padding: UiRect::axes(px(20), px(16)),
            display: Display::None,
            ..default()
        },
    ));
}

/// Tab: the next species. F1: le Calque. F: speed ×2. (A, the sceaux tree, is in `menu`.)
fn toggles(
    keys: Res<ButtonInput<KeyCode>>,
    letters: Res<ButtonInput<Key>>,
    level: Res<Level>,
    mut selection: ResMut<Selection>,
    mut fast: ResMut<Fast>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        next_species(&mut selection, &level);
    }
    if keys.just_pressed(KeyCode::F1) {
        selection.calque = !selection.calque;
    }
    if letter(&letters, "f") {
        fast.0 = !fast.0;
    }
}

/// The game at double speed: waves, walkers and towers, not the menus over it.
#[derive(Resource, Default)]
pub struct Fast(pub bool);

fn speed(fast: Res<Fast>, screen: Res<State<Screen>>, mut time: ResMut<Time<Virtual>>) {
    let ratio = if fast.0 && *screen.get() == Screen::Playing { 2.0 } else { 1.0 };
    if time.relative_speed() != ratio {
        time.set_relative_speed(ratio);
    }
}

/// Shows the route of the next species the level sends.
fn next_species(selection: &mut Selection, level: &Level) {
    let next = level.present.iter().position(|&k| k == selection.shown).map_or(0, |p| (p + 1) % level.present.len());
    selection.shown = level.present[next];
}

/// The latest notes of Nève's notebook, the newest in full ink, the older ones fading back.
fn update_margin(
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
    let grew = written.is_some_and(|(index, len, _)| index == level.index && len < level.margin.len());
    *written = now;
    let (margin, mut node) = margin.into_inner();
    node.display = if level.margin.is_empty() { Display::None } else { Display::Flex };
    let recent = &level.margin[level.margin.len().saturating_sub(SHOWN)..];
    let font = font(&assets, true, SMALL_SIZE);
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

fn fresh_ink(time: Res<Time>, mut notes: Query<(Entity, &mut Fresh, &mut TextColor)>, mut commands: Commands) {
    for (entity, mut fresh, mut color) in &mut notes {
        fresh.0 = (fresh.0 + time.delta_secs() / 0.3).min(1.0);
        color.0 = INK.with_alpha(1.0 - (1.0 - fresh.0).powi(3));
        if fresh.0 >= 1.0 {
            commands.entity(entity).remove::<Fresh>();
        }
    }
}
