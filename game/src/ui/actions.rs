//! The bar's actions, at its right: the next wave, the speed, le Calque, the route shown, the
//! sceaux and the Lexique, each naming its key.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::keys::{Fast, next_species};
use super::widgets::{framed, primary, text};
use crate::combat::enemies::send_wave;
use crate::i18n::t;
use crate::ink::Selection;
use crate::level::Level;
use crate::menu::open;
use crate::schedule::Screen;
use crate::settings::Settings;
use crate::theme::{BODY_SIZE, SMALL_SIZE};

/// The label naming the species whose route is drawn.
#[derive(Component)]
pub struct RouteLabel;

/// The label of the speed button.
#[derive(Component)]
pub struct SpeedLabel;

/// Widths of the two buttons whose words change, so the actions beside them never move: room for
/// "Vitesse ×2 · F" and for the longest species, "Route : La Grande Chaîne rompue · Tab".
const SPEED_WIDTH: f32 = 160.0;
const ROUTE_WIDTH: f32 = 340.0;

/// What the turn asks: the next wave first, in the seal's red.
pub(super) fn spawn_actions(bar: &mut ChildSpawnerCommands, assets: &AssetServer) {
    let small = |s: &str| text(assets, s.to_string(), SMALL_SIZE);
    // A narrow window wraps the actions onto two lines, from the right like the bar's end.
    let row = Node {
        column_gap: px(8),
        row_gap: px(8),
        align_items: AlignItems::Center,
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::FlexEnd,
        ..default()
    };
    bar.spawn(row).with_children(|actions| {
        actions.spawn(primary(0.0, text(assets, t("bar.send_wave"), BODY_SIZE))).observe(
            |_: On<Activate>,
             mut level: ResMut<Level>,
             mut commands: Commands,
             assets: Res<AssetServer>,
             settings: Res<Settings>| {
                send_wave(&mut level, &mut commands, &assets, &settings)
            },
        );
        actions
            .spawn(framed(SPEED_WIDTH, (small(""), SpeedLabel)))
            .observe(|_: On<Activate>, mut fast: ResMut<Fast>| fast.0 = !fast.0);
        actions.spawn(framed(0.0, small(t("bar.calque")))).observe(
            |_: On<Activate>, mut selection: ResMut<Selection>| {
                selection.calque = !selection.calque
            },
        );
        actions.spawn(framed(ROUTE_WIDTH, (small(""), RouteLabel))).observe(
            |_: On<Activate>, level: Res<Level>, mut selection: ResMut<Selection>| {
                next_species(&mut selection, &level)
            },
        );
        actions
            .spawn(framed(0.0, small(t("bar.sceaux"))))
            .observe(open(Screen::Sceaux, Screen::Playing));
        actions
            .spawn(framed(0.0, small(t("bar.lexique"))))
            .observe(open(Screen::Lexique, Screen::Playing));
    });
}
