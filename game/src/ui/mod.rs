//! What the player reads and clicks: the label, Nève's margin, the bar of tools, the names on the
//! sheet, and the page at the end of a level.

pub mod bar;
mod counter_tip;
mod enemy_card;
mod hud;
mod keys;
mod margin;
mod notebook;
mod places;
mod readout;
mod stamp;
mod studio;
mod tool_tip;
pub(crate) mod widgets;

use bevy::prelude::*;

use crate::Step;
use crate::schedule::Screen;
use crate::theme::PAPER;
use margin::Margin;
use widgets::Panel;

pub fn plugin(app: &mut App) {
    app.add_plugins((studio::plugin, widgets::plugin));
    app.init_resource::<keys::Fast>()
        .add_systems(Startup, (setup, places::setup, tool_tip::setup, enemy_card::setup))
        .add_systems(
            Update,
            (
                keys::toggles.in_set(Step::Input).before(crate::overworld::onward::between_levels),
                (
                    hud::update_label,
                    counter_tip::counter_tip,
                    margin::update_margin,
                    margin::fresh_ink,
                    places::label_places,
                    places::price_tip,
                )
                    .in_set(Step::Render),
                // The card of an enemy gives way to the tip of a tool, and covers the price of the
                // cell.
                (tool_tip::tool_tip, enemy_card::enemy_card)
                    .chain()
                    .after(places::price_tip)
                    .in_set(Step::Render)
                    .run_if(in_state(Screen::Playing)),
                (stamp::stamp_wave, stamp::strike_stamps).in_set(Step::Render),
                (bar::sync, bar::look, notebook::sync)
                    .in_set(Step::Render)
                    .run_if(in_state(Screen::Playing)),
                show_during_play.run_if(state_changed::<Screen>),
                keys::speed,
            ),
        );
}

/// The label and the margin only show during play, not behind the menus.
fn show_during_play(
    screen: Res<State<Screen>>,
    mut texts: Query<&mut Visibility, With<DuringPlay>>,
) {
    let shown =
        if *screen.get() == Screen::Playing { Visibility::Inherited } else { Visibility::Hidden };
    for mut v in &mut texts {
        *v = shown;
    }
}

/// Hidden behind the menus.
#[derive(Component)]
struct DuringPlay;

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
