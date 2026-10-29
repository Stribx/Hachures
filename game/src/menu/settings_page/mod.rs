//! The settings page (the settings themselves are in `crate::settings`). It sets one row per
//! setting; ↑ ↓ move between them, ← → or the small arrows change one, a click moves it on.

mod field;

use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{NoNav, Screen, back, button, framed, label, open, page, primary, sheet, text};
use crate::i18n::t;
use crate::menu::BUTTON_WIDTH;
use crate::settings::Settings;
use crate::theme::{BODY_SIZE, HEADING_SIZE, TITLE_SIZE, VEIL};
use field::{COLUMNS, Field};

/// Width of a row's name-and-value button, and of its small arrows: the same for every row, so the
/// arrows line up and no value moves the page. Two columns of rows fit `settings::apply`'s `FIT`.
const ROW_WIDTH: f32 = 370.0;
const ARROW: f32 = 40.0;

/// A row's value, shown at its right.
#[derive(Component)]
struct Value(Field);

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn).add_systems(
        Update,
        (adjust, show_values, preview, relabel).run_if(in_state(Screen::Settings)),
    );
}

/// Button action: open the settings page, coming back to `from`.
pub fn open_settings(from: Screen) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    open(Screen::Settings, from)
}

/// Button action: change the settings with `edit`, then save them.
fn change(
    edit: impl Fn(&mut Settings) + Send + Sync + 'static,
) -> impl Fn(On<Activate>, ResMut<Settings>) {
    move |_, mut settings| {
        edit(&mut settings);
        settings.store();
    }
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Settings, VEIL)).with_children(|page| {
        page.spawn(sheet()).with_children(|p| {
            p.spawn(text(&assets, t("menu.settings"), TITLE_SIZE));
            p.spawn(Node { column_gap: px(48), align_items: AlignItems::FlexStart, ..default() })
                .with_children(|columns| {
                    for groups in COLUMNS {
                        spawn_column(columns, &assets, groups);
                    }
                });
            p.spawn(Node { column_gap: px(16), margin: UiRect::top(px(8)), ..default() })
                .with_children(|actions| {
                    actions.spawn(button(label(&assets, t("settings.reset")))).observe(change(
                        |s| *s = Settings { language: s.language.clone(), ..default() },
                    ));
                    actions
                        .spawn(primary(BUTTON_WIDTH, label(&assets, t("common.back"))))
                        .observe(back);
                });
        });
    });
}

/// A column of the page: each group under its heading.
fn spawn_column(
    columns: &mut ChildSpawnerCommands,
    assets: &AssetServer,
    groups: &[(&str, &[Field])],
) {
    columns
        .spawn(Node { flex_direction: FlexDirection::Column, row_gap: px(8), ..default() })
        .with_children(|column| {
            for &(heading, fields) in groups {
                column
                    .spawn(text(assets, t(heading), HEADING_SIZE))
                    .insert(Node { margin: UiRect::top(px(8)), ..default() });
                // The browser chooses its own frame pacing, and WebGL2 has no ambient
                // occlusion; it keeps its MSAA.
                let web = [Field::Vsync, Field::Occlusion, Field::AntiAliasing];
                let shown =
                    fields.iter().filter(|f| !cfg!(target_arch = "wasm32") || !web.contains(f));
                for &field in shown {
                    spawn_row(column, assets, field);
                }
            }
        });
}

/// A row: the setting's name and value, and the two small arrows.
fn spawn_row(column: &mut ChildSpawnerCommands, assets: &AssetServer, field: Field) {
    let small = |s: &str| text(assets, s.to_string(), BODY_SIZE);
    column.spawn(Node { column_gap: px(6), ..default() }).with_children(|row| {
        let words = (
            Node {
                flex_grow: 1.0,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: px(24),
                ..default()
            },
            children![small(t(field.name())), (small(""), Value(field))],
        );
        row.spawn((framed(ROW_WIDTH, words), field))
            .observe(change(move |settings| field.advance(settings)));
        for (arrow, direction) in [("‹", -1), ("›", 1)] {
            row.spawn((framed(ARROW, small(arrow)), NoNav))
                .observe(change(move |settings| field.step(settings, direction)));
        }
    });
}

/// ← → change the setting whose row has the keyboard focus.
fn adjust(
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    fields: Query<&Field>,
    mut settings: ResMut<Settings>,
) {
    let dir = if keys.just_pressed(KeyCode::ArrowRight) {
        1
    } else if keys.just_pressed(KeyCode::ArrowLeft) {
        -1
    } else {
        return;
    };
    if let Some(field) = focus.get().and_then(|e| fields.get(e).ok()) {
        field.step(&mut settings, dir);
        settings.store();
    }
}

fn show_values(settings: Res<Settings>, mut values: Query<(&Value, &mut Text)>) {
    for (Value(field), mut text) in &mut values {
        text.set_if_neq(Text(field.value(&settings)));
    }
}

/// The effects or the general volume moved: a knock at the new volume.
fn preview(
    settings: Res<Settings>,
    assets: Res<AssetServer>,
    mut heard: Local<Option<(u32, u32)>>,
    mut commands: Commands,
) {
    let now = (settings.master, settings.effects);
    if heard.replace(now).is_some_and(|before| before != now) {
        commands.spawn((
            AudioPlayer::new(assets.load(crate::map::land::KNOCK)),
            PlaybackSettings::DESPAWN.with_volume(settings.effects_volume()),
        ));
    }
}

/// The language changed: the page is written again in it.
fn relabel(
    settings: Res<Settings>,
    mut shown: Local<Option<String>>,
    mut next: ResMut<NextState<Screen>>,
) {
    let language = &settings.language;
    if shown.replace(language.clone()).is_some_and(|before| before != *language) {
        next.set(Screen::Settings);
    }
}
