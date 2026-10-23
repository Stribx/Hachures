//! The screens around the game: splash, title, settings and pause, and the widgets they share.

mod pause;
mod settings;
mod splash;
mod title;

use bevy::input_focus::{FocusCause, InputFocus};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::{Pressed, UiGlobalTransform};
use bevy::ui_widgets::{Activate, Button};

pub use settings::{Music, Settings};

/// Which screen is up. The game's input and simulation only run while `Playing`.
#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    /// "Erase the campaign?" before a new game.
    ConfirmNew,
    /// Replay any level reached.
    Chapters,
    Settings,
    Playing,
    Paused,
}

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .add_plugins((splash::plugin, title::plugin, pause::plugin, settings::plugin))
        .add_systems(OnEnter(Screen::Playing), reset_input)
        .add_systems(
            Update,
            (
                escape,
                clear_focus.run_if(state_changed::<Screen>),
                navigate.run_if(not(in_state(Screen::Playing))),
                button_look,
            ),
        );
}

const INK: Color = Color::srgb(0.22, 0.14, 0.08);
/// Paper laid over the map: title, settings and pause.
const VEIL: Color = Color::srgba(0.93, 0.88, 0.76, 0.88);
/// Opaque paper: the splash.
const PAPER: Color = Color::srgb(0.93, 0.88, 0.76);
const LIT: Color = Color::srgba(0.55, 0.4, 0.22, 0.25);
const PRESSED: Color = Color::srgba(0.55, 0.4, 0.22, 0.5);

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

/// Ink text in the game's font.
fn text(assets: &AssetServer, text: impl Into<String>, size: f32) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: assets.load("fonts/IMFellEnglish-Regular.ttf").into(),
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(INK),
    )
}

/// A button's label.
fn label(assets: &AssetServer, label: impl Into<String>) -> impl Bundle {
    text(assets, label, 28.0)
}

/// A framed button around `label`; attach its action with `.observe(|_: On<Activate>, ..| ..)`.
fn button(label: impl Bundle) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            min_width: px(320),
            padding: UiRect::axes(px(16), px(6)),
            border: UiRect::all(px(2)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderColor::all(INK),
        BackgroundColor(Color::NONE),
        children![label],
    )
}

/// Button action: go to `screen`.
fn go(screen: Screen) -> impl Fn(On<Activate>, ResMut<NextState<Screen>>) {
    move |_, mut next| next.set(screen)
}

/// Esc: pause, resume, or back from a page.
fn escape(
    keys: Res<ButtonInput<KeyCode>>,
    screen: Res<State<Screen>>,
    from: Option<Res<settings::SettingsFrom>>,
    mut next: ResMut<NextState<Screen>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    match screen.get() {
        Screen::Playing => next.set(Screen::Paused),
        Screen::Paused => next.set(Screen::Playing),
        Screen::Settings => next.set(from.map_or(Screen::Title, |f| f.0)),
        Screen::ConfirmNew | Screen::Chapters => next.set(Screen::Title),
        Screen::Splash | Screen::Title => {}
    }
}

/// The key or click that started play must not also act in the game.
fn reset_input(mut keys: ResMut<ButtonInput<KeyCode>>, mut mouse: ResMut<ButtonInput<MouseButton>>) {
    keys.reset_all();
    mouse.reset_all();
}

fn clear_focus(mut focus: ResMut<InputFocus>) {
    focus.clear();
}

/// Arrows or Tab move the keyboard focus through the page's buttons, in reading order;
/// Enter or Space presses the focused one.
fn navigate(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Query<(Entity, &UiGlobalTransform), With<Button>>,
    mut focus: ResMut<InputFocus>,
) {
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let step = if keys.any_just_pressed([KeyCode::ArrowDown, KeyCode::ArrowRight])
        || (keys.just_pressed(KeyCode::Tab) && !shift)
    {
        1
    } else if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::ArrowLeft])
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

/// Buttons darken under the mouse or the keyboard focus, and more while pressed.
fn button_look(
    focus: Res<InputFocus>,
    mut buttons: Query<(Entity, &Hovered, Has<Pressed>, &mut BackgroundColor), With<Button>>,
) {
    for (entity, hovered, pressed, mut background) in &mut buttons {
        background.0 = if pressed {
            PRESSED
        } else if hovered.get() || focus.get() == Some(entity) {
            LIT
        } else {
            Color::NONE
        };
    }
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
