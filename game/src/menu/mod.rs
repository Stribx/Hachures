//! The screens around the game: splash, title, pause, settings, sceaux, lexique, carnet, controls
//! and credits.

mod backdrop;
mod carnet;
mod controls;
mod credits;
mod editor;
mod lexique;
mod pause;
mod sceaux;
mod settings_page;
mod splash;
mod title;
mod warmup;

use bevy::ecs::system::SystemParam;
use bevy::input::keyboard::Key;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use bevy::ui_widgets::{Activate, Button};

// The pages build with the shared widgets; `super::{text, ..}` reaches them from here.
use crate::schedule::Screen;
use crate::theme::{PAPER, SUBTITLE_SIZE};
use crate::ui::widgets::{Panel, Rise, framed, letter, primary, text};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        backdrop::plugin,
        splash::plugin,
        title::plugin,
        pause::plugin,
        settings_page::plugin,
        sceaux::plugin,
        lexique::plugin,
        carnet::plugin,
        controls::plugin,
        credits::plugin,
        editor::plugin,
        warmup::plugin,
    ))
    .add_systems(OnEnter(Screen::Playing), reset_input)
    .add_systems(
        Update,
        (
            escape,
            clear_focus.run_if(state_changed::<Screen>),
            // On the Grand Plan the arrows walk Nève.
            navigate.run_if(not(in_state(Screen::Playing))).run_if(not(in_state(Screen::Plan))),
        ),
    );
}

/// A full-screen page shown while in `screen`, its content in a centered column.
fn page(screen: Screen, background: Color) -> impl Bundle {
    (
        DespawnOnExit(screen),
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(14),
            ..default()
        },
        BackgroundColor(background),
    )
}

/// A button's label.
fn label(assets: &AssetServer, label: impl Into<String>) -> impl Bundle {
    text(assets, label, SUBTITLE_SIZE)
}

/// How wide a page's buttons are, framed or primary: all the same, and room for the longest label,
/// a pause button asking "Sûr ? Recommencer le niveau", on one line.
pub(crate) const BUTTON_WIDTH: f32 = 380.0;

/// A framed button around `label`; attach its action with `.observe(|_: On<Activate>, ..| ..)`.
fn button(label: impl Bundle) -> impl Bundle {
    framed(BUTTON_WIDTH, label)
}

/// A button the keyboard focus passes over: the small arrows beside a settings row, reached by ← →
/// instead.
#[derive(Component)]
pub(crate) struct NoNav;

/// Button action: go to `screen`.
fn go(screen: Screen) -> impl Fn(On<Activate>, ResMut<NextState<Screen>>) {
    move |_, mut next| next.set(screen)
}

/// Where the settings, sceaux, lexique, carnet, controls and credits pages go back to.
#[derive(Resource)]
pub struct BackTo(pub Screen);

/// Button action: open `page`, coming back to `from`.
pub fn open(
    page: Screen,
    from: Screen,
) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    move |_, mut commands, mut next| {
        commands.insert_resource(BackTo(from));
        next.set(page);
    }
}

/// What `toggle_page` reads and sets.
#[derive(SystemParam)]
struct PageKeys<'w, 's> {
    keys: Res<'w, ButtonInput<Key>>,
    screen: Res<'w, State<Screen>>,
    from: Option<Res<'w, BackTo>>,
    commands: Commands<'w, 's>,
    next: ResMut<'w, NextState<Screen>>,
}

/// System: the key `key` during play opens `page`; pressed again there, it closes it.
fn toggle_page(key: &'static str, page: Screen) -> impl Fn(PageKeys) {
    move |mut p| {
        if !letter(&p.keys, key) {
            return;
        }
        match p.screen.get() {
            Screen::Playing => {
                p.commands.insert_resource(BackTo(Screen::Playing));
                p.next.set(page);
            }
            shown if *shown == page => {
                let back = p.from.as_ref().map_or(Screen::Title, |f| f.0);
                p.next.set(back);
            }
            _ => {}
        }
    }
}

/// Button action: back to where the page was opened from.
fn back(_: On<Activate>, from: Res<BackTo>, mut next: ResMut<NextState<Screen>>) {
    next.set(from.0);
}

/// A page's sheet of paper, rising as it comes in, its content in a centered column.
fn sheet() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(14),
            padding: UiRect::axes(px(40), px(28)),
            ..default()
        },
        BackgroundColor(PAPER),
        Panel,
        UiTransform::IDENTITY,
        Rise::default(),
    )
}

/// Esc: pause, resume, or back from a page.
fn escape(
    keys: Res<ButtonInput<KeyCode>>,
    screen: Res<State<Screen>>,
    from: Option<Res<BackTo>>,
    mut next: ResMut<NextState<Screen>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    match screen.get() {
        Screen::Playing => next.set(Screen::Paused),
        Screen::Paused => next.set(Screen::Playing),
        Screen::Settings
        | Screen::Sceaux
        | Screen::Lexique
        | Screen::Carnet
        | Screen::Editor
        | Screen::Controls
        | Screen::Credits => next.set(from.map_or(Screen::Title, |f| f.0)),
        Screen::ConfirmNew | Screen::Plan => next.set(Screen::Title),
        Screen::Splash | Screen::Title => {}
    }
}

/// The key or click that started play must not also act in the game.
fn reset_input(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut letters: ResMut<ButtonInput<Key>>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
) {
    keys.reset_all();
    letters.reset_all();
    mouse.reset_all();
}

fn clear_focus(mut focus: ResMut<InputFocus>) {
    focus.clear();
}

/// The buttons the keyboard focus goes through.
type Navigable = (With<Button>, Without<NoNav>);

/// Arrows or Tab move the keyboard focus through the page's buttons, in reading order;
/// Enter or Space presses the focused one. On the settings page ← → change a setting instead.
fn navigate(
    keys: Res<ButtonInput<KeyCode>>,
    screen: Res<State<Screen>>,
    buttons: Query<(Entity, &UiGlobalTransform), Navigable>,
    mut focus: ResMut<InputFocus>,
) {
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let sideways = *screen.get() != Screen::Settings;
    let step = if keys.just_pressed(KeyCode::ArrowDown)
        || (sideways && keys.just_pressed(KeyCode::ArrowRight))
        || (keys.just_pressed(KeyCode::Tab) && !shift)
    {
        1
    } else if keys.just_pressed(KeyCode::ArrowUp)
        || (sideways && keys.just_pressed(KeyCode::ArrowLeft))
        || (keys.just_pressed(KeyCode::Tab) && shift)
    {
        -1
    } else {
        return;
    };
    let mut order: Vec<(Entity, Vec2)> = buttons.iter().map(|(e, t)| (e, t.translation)).collect();
    if order.is_empty() {
        return;
    }
    order.sort_by(|a, b| a.1.y.total_cmp(&b.1.y).then(a.1.x.total_cmp(&b.1.x)));
    let len = order.len() as isize;
    let next = match focus.get().and_then(|f| order.iter().position(|o| o.0 == f)) {
        Some(i) => (i as isize + step).rem_euclid(len),
        None if step > 0 => 0,
        None => len - 1,
    };
    focus.set(order[next as usize].0, FocusCause::Navigated);
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;

    /// Esc pauses the game, and Esc again resumes it.
    #[test]
    fn escape_pauses_and_resumes() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin))
            .init_state::<Screen>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(Update, escape);
        app.world_mut().resource_mut::<NextState<Screen>>().set(Screen::Playing);
        app.update();
        let press_escape = |app: &mut App| {
            app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Escape);
            app.update();
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::Escape);
            keys.clear();
            // The new state applies on the next frame.
            app.update();
            *app.world().resource::<State<Screen>>().get()
        };
        assert_eq!(press_escape(&mut app), Screen::Paused);
        assert_eq!(press_escape(&mut app), Screen::Playing);
    }
}
