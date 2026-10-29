//! Le Lexique: what the tools and the species do, a tab each, read back from what the player has
//! met. Open from the title, the pause page, or with L during play.

mod detail;
mod list;

use bevy::input::keyboard::Key;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{BackTo, Screen, back, open, page, primary, sheet, text};
use crate::campaign::lexique::{EntryKind, Tab};
use crate::i18n::t;
use crate::menu::BUTTON_WIDTH;
use crate::theme::{BODY_SIZE, TITLE_SIZE, VEIL};
use crate::ui::scrollbar::scrollbar;
use crate::ui::studio::{SpeciesIcons, ToolIcons};
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

/// Height of the list and the sheet, in pixels: nine rows, and a sheet's picture and first lines.
const BODY_HEIGHT: f32 = 430.0;

/// Spawns the scroll area `area` in `row`, its scrollbar beside it; returns the area.
fn beside_its_bar(row: &mut ChildSpawnerCommands, area: impl Bundle) -> Entity {
    let mut spawned = Entity::PLACEHOLDER;
    row.spawn(Node { column_gap: px(6), ..default() }).with_children(|pair| {
        spawned = pair.spawn(area).id();
        pair.spawn(scrollbar(spawned));
    });
    spawned
}

/// The filmed piece or figurine of `kind`.
fn picture(kind: EntryKind, tools: &ToolIcons, species: &SpeciesIcons) -> Handle<Image> {
    let pictures = match kind {
        EntryKind::Tool(tool) => tools.0.get(tool),
        EntryKind::Species(s) => species.0.get(s),
    };
    pictures.cloned().unwrap_or_default()
}

/// A picture's tint: its own colours once met; before, a faded silhouette that keeps the surprise.
fn tint(seen: bool) -> Color {
    if seen { Color::WHITE } else { Color::BLACK.with_alpha(0.25) }
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(LexiqueView::default());
    commands.spawn(page(Screen::Lexique, VEIL)).with_children(|page| {
        page.spawn(sheet()).with_children(|p| {
            p.spawn(text(&assets, t("lexique.title"), TITLE_SIZE));
            list::spawn_tabs(p, &assets);
            // A fixed size, whatever the tab or the entry: the list and the sheet scroll inside.
            p.spawn(Node { column_gap: px(32), height: px(BODY_HEIGHT), ..default() })
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
    use crate::ui::studio::{SpeciesIcons, ToolIcons};

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
            .init_asset::<Font>()
            .init_asset::<Image>()
            .init_state::<Screen>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<Key>>()
            .init_resource::<Save>()
            .init_resource::<LexiqueView>()
            .insert_resource(Level::new(Grid::parse("S.M"), species::load()))
            .add_systems(Update, (super::super::escape, key_l));
        let species = app.world().resource::<Level>().species.len();
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        let mut blank = |n: usize| (0..n).map(|_| images.add(Image::default())).collect();
        let (tools, species) = (ToolIcons(blank(TOOLS.len())), SpeciesIcons(blank(species)));
        app.insert_resource(tools).insert_resource(species);
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

    /// The `C` of each row's parts, in row order.
    fn in_rows<C: Component + Clone>(app: &mut App) -> Vec<C> {
        let world = app.world_mut();
        let mut rows: Vec<(usize, Entity)> =
            world.query::<(Entity, &list::EntryRow)>().iter(world).map(|(e, r)| (r.0, e)).collect();
        rows.sort();
        rows.iter().map(|&(_, row)| find::<C>(world, row).unwrap()).collect()
    }

    /// The `C` on `entity` or the first of its descendants that has one.
    fn find<C: Component + Clone>(world: &World, entity: Entity) -> Option<C> {
        let own = world.get::<C>(entity).cloned();
        let children = world.get::<Children>(entity);
        own.or_else(|| children?.iter().find_map(|child| find(world, child)))
    }

    /// The list under a refresh, one name per row.
    fn rows(app: &mut App) -> Vec<String> {
        app.world_mut().spawn(list::EntryList::default());
        app.add_systems(Update, (list::rebuild, list::show).chain());
        app.update();
        in_rows::<Text>(app).into_iter().map(|t| t.0).collect()
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

    #[test]
    fn every_row_shows_its_entrys_picture() {
        let mut app = app();
        rows(&mut app);
        let images = |app: &mut App| -> Vec<_> {
            in_rows::<ImageNode>(app).into_iter().map(|p| p.image).collect()
        };
        assert_eq!(images(&mut app), app.world().resource::<ToolIcons>().0);
        app.world_mut().resource_mut::<LexiqueView>().tab = Tab::Especes;
        app.update();
        assert_eq!(images(&mut app), app.world().resource::<SpeciesIcons>().0);
    }

    #[test]
    fn a_locked_row_is_a_faded_silhouette_and_a_seen_one_its_colours() {
        let mut app = app();
        let arbalete = tool_index("arbalète").unwrap();
        app.world_mut().resource_mut::<Save>().seen.insert(TOOLS[arbalete].label.to_string());
        rows(&mut app);
        let tints: Vec<Color> = in_rows::<ImageNode>(&mut app).iter().map(|p| p.color).collect();
        assert_eq!(tints[arbalete], Color::WHITE);
        assert_ne!(tint(false), Color::WHITE);
        assert!(tints.iter().enumerate().all(|(i, &c)| i == arbalete || c == tint(false)));
    }

    #[test]
    fn the_sheet_shows_the_picked_entrys_picture() {
        let mut app = app();
        app.add_systems(OnEnter(Screen::Lexique), spawn).add_systems(Update, detail::show);
        let arbalete = tool_index("arbalète").unwrap();
        app.world_mut().resource_mut::<Save>().seen.insert(TOOLS[arbalete].label.to_string());
        go(&mut app, Screen::Lexique);
        let picture = |app: &mut App| {
            let world = app.world_mut();
            world.query_filtered::<&ImageNode, With<detail::SheetPicture>>().single(world).cloned()
        };
        app.world_mut().resource_mut::<LexiqueView>().entry = arbalete;
        app.update();
        let shown = picture(&mut app).unwrap();
        assert_eq!(shown.image, app.world().resource::<ToolIcons>().0[arbalete]);
        assert_eq!(shown.color, Color::WHITE);
        app.world_mut().resource_mut::<LexiqueView>().entry = 0;
        app.update();
        let shown = picture(&mut app).unwrap();
        assert_eq!(shown.image, app.world().resource::<ToolIcons>().0[0]);
        assert_eq!(shown.color, tint(false));
    }

    #[test]
    fn picking_another_entry_scrolls_the_sheet_back_to_the_top() {
        let mut app = app();
        app.add_systems(OnEnter(Screen::Lexique), spawn).add_systems(Update, detail::show);
        go(&mut app, Screen::Lexique);
        let scrolled = |app: &mut App, to: Option<f32>| {
            let world = app.world_mut();
            let mut sheets = world.query_filtered::<&mut ScrollPosition, With<detail::Sheet>>();
            let mut at = sheets.single_mut(world).unwrap();
            if let Some(y) = to {
                at.y = y;
            }
            at.y
        };
        scrolled(&mut app, Some(120.0));
        app.update();
        assert_eq!(scrolled(&mut app, None), 120.0, "nothing picked, the sheet stays put");
        app.world_mut().resource_mut::<LexiqueView>().entry = 3;
        app.update();
        assert_eq!(scrolled(&mut app, None), 0.0);
    }
}
