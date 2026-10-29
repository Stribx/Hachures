//! Les commandes: every key of the game on one sheet, read-only. Open from the title and the pause
//! page.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{Screen, back, label, open, page, primary, sheet, text};
use crate::i18n::{n_, t};
use crate::menu::BUTTON_WIDTH;
use crate::theme::{BODY_SIZE, HEADING_SIZE, INK_SOFT, TITLE_SIZE, VEIL};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Controls), spawn);
}

/// Button action: open the controls, coming back to `from`.
pub fn open_controls(from: Screen) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    open(Screen::Controls, from)
}

/// (key, what it does), by where it is used. Arrows are named, not drawn: IM Fell has no arrow
/// glyphs.
const KEYS: [(&str, &[(&str, &str)]); 3] = [
    (
        n_("controls.group.sheet"),
        &[
            (n_("controls.key.digits"), n_("controls.action.pick_tool")),
            (n_("controls.key.backspace"), n_("controls.action.biffer")),
            (n_("controls.key.click"), n_("controls.action.ink")),
            (n_("controls.key.space"), n_("controls.action.send_wave")),
            (n_("controls.key.f"), n_("controls.action.speed")),
            (n_("controls.key.tab"), n_("controls.action.next_route")),
            (n_("controls.key.f1"), n_("controls.action.calque")),
            (n_("controls.key.a"), n_("controls.action.sceaux")),
            (n_("controls.key.l"), n_("controls.action.lexique")),
            (n_("controls.key.wheel"), n_("controls.action.zoom")),
            (n_("controls.key.enter"), n_("controls.action.next_or_retry")),
        ],
    ),
    (
        n_("controls.group.plan"),
        &[
            (n_("controls.key.arrows_q_d"), n_("controls.action.walk")),
            (n_("controls.key.enter"), n_("controls.action.trace")),
        ],
    ),
    (
        n_("controls.group.everywhere"),
        &[
            (n_("controls.key.esc"), n_("controls.action.pause_back")),
            (n_("controls.key.arrows_tab"), n_("controls.action.choose_button")),
            (n_("controls.key.enter"), n_("controls.action.press")),
            (n_("controls.key.f3"), n_("controls.action.performance")),
            (n_("controls.key.f12"), n_("controls.action.screenshot")),
        ],
    ),
];

/// The F12 capture is the desktop build's: the web build leaves its row out.
const DESKTOP_ONLY: &str = n_("controls.key.f12");

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Controls, VEIL)).with_children(|page| {
        page.spawn(sheet()).with_children(|p| {
            p.spawn(text(&assets, t("menu.controls"), TITLE_SIZE));
            p.spawn(Node { column_gap: px(48), align_items: AlignItems::FlexStart, ..default() })
                .with_children(|columns| {
                    // The board's keys on the left, the rest on the right.
                    for groups in [&KEYS[..1], &KEYS[1..]] {
                        columns
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: px(6),
                                ..default()
                            })
                            .with_children(|column| {
                                for &(heading, keys) in groups {
                                    column
                                        .spawn(text(&assets, t(heading), HEADING_SIZE))
                                        .insert(Node { margin: UiRect::top(px(8)), ..default() });
                                    for &(key, action) in keys {
                                        if key == DESKTOP_ONLY && cfg!(target_arch = "wasm32") {
                                            continue;
                                        }
                                        column
                                            .spawn(Node { column_gap: px(16), ..default() })
                                            .with_children(|row| {
                                                row.spawn(text(&assets, t(key), BODY_SIZE))
                                                    .insert(Node { width: px(130), ..default() });
                                                row.spawn(text(&assets, t(action), BODY_SIZE))
                                                    .insert(TextColor(INK_SOFT));
                                            });
                                    }
                                }
                            });
                    }
                });
            p.spawn(primary(BUTTON_WIDTH, label(&assets, t("common.back")))).observe(back);
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The row the web build leaves out is one of the sheet's rows: a typo would show it on the
    /// web.
    #[test]
    fn the_desktop_only_key_is_a_row() {
        let rows: Vec<&str> =
            KEYS.iter().flat_map(|&(_, keys)| keys.iter().map(|&(key, _)| key)).collect();
        assert!(rows.contains(&DESKTOP_ONLY), "{DESKTOP_ONLY} not in {rows:?}");
    }
}
