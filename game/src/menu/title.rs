//! The main menu over the Marches, and the confirmation before a new game erases the campaign.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::carnet::written;
use super::open;
use super::{Screen, button, go, label, page, primary, text};
use crate::campaign::LoadLevel;
use crate::campaign::save::Save;
use crate::i18n::{t, tp};
use crate::level::Level;
use crate::menu::BUTTON_WIDTH;
use crate::theme::{HEADING_SIZE, INK_SOFT, PAPER, SMALL_SIZE, SUBTITLE_SIZE, VEIL};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn)
        .add_systems(OnEnter(Screen::ConfirmNew), spawn_confirm);
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>, save: Res<Save>, level: Res<Level>) {
    let new = save.is_new();
    let mut page = commands.spawn(page(Screen::Title, Color::NONE));
    page.entry::<Node>().and_modify(|mut node| {
        node.align_items = AlignItems::FlexStart;
        node.padding = UiRect::left(px(64));
    });
    page.with_children(|page| {
        let mut p = page.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(14),
                padding: UiRect::axes(px(40), px(36)),
                ..default()
            },
            BackgroundColor(PAPER),
            super::Panel,
            UiTransform::IDENTITY,
            super::Rise::default(),
        ));
        p.with_children(|p| {
            p.spawn(text(&assets, crate::NAME, 72.0));
            p.spawn(text(&assets, t("title.tagline"), SUBTITLE_SIZE));
            // A new campaign starts with the prologue; one under way goes back to the Grand Plan.
            p.spawn(primary(
                BUTTON_WIDTH,
                label(&assets, if new { t("title.play") } else { t("title.continue") }),
            ))
            .observe(go(if new { Screen::Playing } else { Screen::Plan }));
            if !new {
                let n = save.sceaux();
                let progress = tp("title.progress", n, &[&t(&level.name), &n]);
                p.spawn(text(&assets, progress, SMALL_SIZE)).insert(TextColor(INK_SOFT));
                p.spawn(button(label(&assets, t("sceaux.title"))))
                    .observe(open(Screen::Sceaux, Screen::Title));
                p.spawn(button(label(&assets, t("menu.lexique"))))
                    .observe(open(Screen::Lexique, Screen::Title));
                if written(&save) {
                    p.spawn(button(label(&assets, t("title.carnet"))))
                        .observe(open(Screen::Carnet, Screen::Title));
                    if save.ending().is_some() {
                        p.spawn(button(label(&assets, t("title.editor"))))
                            .observe(open(Screen::Editor, Screen::Title));
                    }
                }
            }
            p.spawn(button(label(&assets, t("menu.controls"))))
                .observe(open(Screen::Controls, Screen::Title));
            p.spawn(button(label(&assets, t("menu.settings"))))
                .observe(open(Screen::Settings, Screen::Title));
            p.spawn(button(label(&assets, t("menu.credits"))))
                .observe(open(Screen::Credits, Screen::Title));
            if !new {
                p.spawn(button(label(&assets, t("title.new_game"))))
                    .observe(go(Screen::ConfirmNew));
            }
            // A browser tab is closed, not quit.
            #[cfg(not(target_arch = "wasm32"))]
            p.spawn(button(label(&assets, t("title.quit")))).observe(
                |_: On<Activate>, mut exit: MessageWriter<AppExit>| {
                    exit.write(AppExit::Success);
                },
            );
        });
        page.spawn(version_label(&assets)).insert(TextColor(INK_SOFT));
    });
}

fn version_label(assets: &AssetServer) -> impl Bundle {
    (
        text(assets, concat!("v", env!("CARGO_PKG_VERSION")), SMALL_SIZE),
        Node { position_type: PositionType::Absolute, right: px(16), bottom: px(12), ..default() },
    )
}

fn spawn_confirm(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::ConfirmNew, VEIL)).with_children(|p| {
        p.spawn(text(&assets, t("title.confirm_new"), HEADING_SIZE));
        p.spawn(button(label(&assets, t("title.confirm_yes")))).observe(new_game);
        p.spawn(button(label(&assets, t("title.confirm_no")))).observe(go(Screen::Title));
    });
}

fn new_game(
    _: On<Activate>,
    mut save: ResMut<Save>,
    mut load: MessageWriter<LoadLevel>,
    mut next: ResMut<NextState<Screen>>,
) {
    *save = Save::default();
    save.store();
    load.write(LoadLevel(0));
    next.set(Screen::Playing);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `text` already carries a `TextColor`: a second one in the bundle panics on spawn.
    #[test]
    fn the_version_label_spawns() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default())).init_asset::<Font>();
        let assets = app.world().resource::<AssetServer>().clone();
        app.world_mut().spawn(version_label(&assets));
    }
}
