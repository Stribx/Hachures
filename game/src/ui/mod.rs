//! What the player reads and toggles: the label, Nève's margin, the sceaux tree, le Calque.

mod hud;

use bevy::prelude::*;

use crate::Step;
use crate::campaign::between_levels;
use crate::level::Level;
use crate::menu::Screen;

pub fn plugin(app: &mut App) {
    app.init_resource::<TreeOpen>().add_systems(Startup, setup).add_systems(
        Update,
        (
            toggles.in_set(Step::Input).before(between_levels),
            (hud::update_label, update_margin).in_set(Step::Render),
            show_during_play.run_if(state_changed::<Screen>),
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

/// The skill tree is shown instead of the usual panel.
#[derive(Resource, Default)]
pub struct TreeOpen(pub bool);

#[derive(Component)]
struct Hud;

/// Hidden behind the menus.
#[derive(Component)]
struct DuringPlay;

/// Nève's notebook, down the right side.
#[derive(Component)]
struct Margin;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Hud,
        DuringPlay,
        Visibility::Hidden,
        Text::default(),
        TextFont {
            font: assets.load("fonts/IMFellEnglish-Regular.ttf").into(),
            font_size: FontSize::Px(24.0),
            ..default()
        },
        Node { margin: UiRect::all(Val::Px(16.0)), ..default() },
    ));
    commands.spawn((
        Margin,
        DuringPlay,
        Visibility::Hidden,
        Text::default(),
        TextFont {
            font: assets.load("fonts/IMFellEnglish-Italic.ttf").into(),
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.85, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            right: Val::Px(16.0),
            width: Val::Px(360.0),
            ..default()
        },
    ));
}

/// A: the sceaux tree. Tab: the next species. F1: le Calque.
fn toggles(keys: Res<ButtonInput<KeyCode>>, mut level: ResMut<Level>, mut tree: ResMut<TreeOpen>) {
    if keys.just_pressed(KeyCode::KeyA) {
        tree.0 = !tree.0;
    }
    if keys.just_pressed(KeyCode::Tab) {
        let next = level.present.iter().position(|&k| k == level.shown).map_or(0, |p| (p + 1) % level.present.len());
        level.shown = level.present[next];
    }
    if keys.just_pressed(KeyCode::F1) {
        level.calque = !level.calque;
    }
}

/// The latest notes of Nève's notebook.
fn update_margin(level: Res<Level>, mut margin: Single<&mut Text, With<Margin>>) {
    let recent = &level.margin[level.margin.len().saturating_sub(8)..];
    margin.0 = recent.join("\n\n");
}
