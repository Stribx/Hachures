//! Le Lexique: what the tools and the species do, a tab each, read back from what the player has met.
//! Open from the title, the pause page, or with L during play.

mod detail;
mod list;

use bevy::input::keyboard::Key;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{BackTo, Screen, back, open, page, primary, sheet, text};
use crate::campaign::lexique::Tab;
use crate::i18n::t;
use crate::menu::BUTTON_WIDTH;
use crate::theme::{BODY_SIZE, TITLE_SIZE, VEIL};
use crate::ui::widgets::letter;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Lexique), spawn).add_systems(
        Update,
        (list::rebuild, list::pick, list::show, detail::show)
            .chain()
            .run_if(in_state(Screen::Lexique)),
    );
    app.add_systems(Update, key_l);
}

/// Button action: open the Lexique, coming back to `from`.
pub fn open_lexique(from: Screen) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    open(Screen::Lexique, from)
}

/// L during play opens the Lexique; L again closes it.
fn key_l(
    keys: Res<ButtonInput<Key>>,
    screen: Res<State<Screen>>,
    from: Option<Res<BackTo>>,
    mut commands: Commands,
    mut next: ResMut<NextState<Screen>>,
) {
    if !letter(&keys, "l") {
        return;
    }
    match screen.get() {
        Screen::Playing => {
            commands.insert_resource(BackTo(Screen::Playing));
            next.set(Screen::Lexique);
        }
        Screen::Lexique => next.set(from.map_or(Screen::Title, |f| f.0)),
        _ => {}
    }
}

/// The tab shown and the entry picked in it.
#[derive(Resource, PartialEq, Default)]
struct LexiqueView {
    tab: Tab,
    entry: usize,
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(LexiqueView::default());
    commands.spawn(page(Screen::Lexique, VEIL)).with_children(|page| {
        page.spawn(sheet()).with_children(|p| {
            p.spawn(text(&assets, t("lexique.title"), TITLE_SIZE));
            list::spawn_tabs(p, &assets);
            p.spawn(Node { column_gap: px(32), align_items: AlignItems::FlexStart, ..default() })
                .with_children(|row| {
                    list::spawn_list(row);
                    detail::spawn(row, &assets);
                });
            p.spawn(primary(BUTTON_WIDTH, text(&assets, t("common.back"), BODY_SIZE)))
                .observe(back);
        });
    });
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;
    use crate::campaign::save::Save;
    use crate::ink::{TOOLS, tool_index};
    use crate::level::Level;
    use crate::map::grid::Grid;
    use crate::map::species;

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
            .init_asset::<Font>()
            .init_state::<Screen>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<Key>>()
            .init_resource::<Save>()
            .init_resource::<LexiqueView>()
            .insert_resource(Level::new(Grid::parse("S.M"), species::load()))
            .add_systems(Update, (super::super::escape, key_l));
        app.update();
        app
    }

    fn go(app: &mut App, screen: Screen) {
        app.world_mut().resource_mut::<NextState<Screen>>().set(screen);
        app.update();
    }

    /// Types `key` for one frame, then lets the new state apply.
    fn tap(app: &mut App, code: KeyCode, key: Key) {
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(code);
        app.world_mut().resource_mut::<ButtonInput<Key>>().press(key.clone());
        app.update();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().release(code);
        app.world_mut().resource_mut::<ButtonInput<Key>>().release(key);
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear();
        app.world_mut().resource_mut::<ButtonInput<Key>>().clear();
        app.update();
    }

    fn screen(app: &App) -> Screen {
        *app.world().resource::<State<Screen>>().get()
    }

    #[test]
    fn escape_on_the_page_goes_back() {
        let mut app = app();
        app.insert_resource(BackTo(Screen::Paused));
        go(&mut app, Screen::Lexique);
        tap(&mut app, KeyCode::Escape, Key::Escape);
        assert_eq!(screen(&app), Screen::Paused);
    }

    #[test]
    fn l_opens_the_page_in_play_and_closes_it() {
        let mut app = app();
        go(&mut app, Screen::Playing);
        tap(&mut app, KeyCode::KeyL, Key::Character("l".into()));
        assert_eq!(screen(&app), Screen::Lexique);
        tap(&mut app, KeyCode::KeyL, Key::Character("l".into()));
        assert_eq!(screen(&app), Screen::Playing);
    }

    /// The list under a refresh, one name per row.
    fn rows(app: &mut App) -> Vec<String> {
        app.world_mut().spawn(list::EntryList::default());
        app.add_systems(Update, (list::rebuild, list::show).chain());
        app.update();
        let world = app.world_mut();
        let mut rows: Vec<(usize, Entity)> =
            world.query::<(Entity, &list::EntryRow)>().iter(world).map(|(e, r)| (r.0, e)).collect();
        rows.sort();
        rows.iter()
            .map(|&(_, row)| {
                let child = world.get::<Children>(row).unwrap()[0];
                world.get::<Text>(child).unwrap().0.clone()
            })
            .collect()
    }

    #[test]
    fn a_locked_entry_reads_question_marks_and_a_seen_one_its_name() {
        let mut app = app();
        let arbalete = tool_index("arbalète").unwrap();
        app.world_mut().resource_mut::<Save>().seen.insert(TOOLS[arbalete].label.to_string());
        let names = rows(&mut app);
        assert_eq!(names.len(), TOOLS.len());
        assert_eq!(names[arbalete], t(TOOLS[arbalete].label));
        assert!(
            names.iter().enumerate().all(|(i, n)| i == arbalete || n == t("lexique.locked")),
            "{names:?}"
        );
    }

    #[test]
    fn picking_a_tab_lists_its_entries() {
        let mut app = app();
        assert_eq!(rows(&mut app).len(), TOOLS.len());
        let species = app.world().resource::<Level>().species.len();
        app.world_mut().resource_mut::<LexiqueView>().tab = Tab::Especes;
        app.update();
        let world = app.world_mut();
        assert_eq!(world.query::<&list::EntryRow>().iter(world).count(), species);
    }
}
