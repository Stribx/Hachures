//! The card of the enemy under the cursor: its name, its health and what the Lexique says of it.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::hud::capitalized;
use super::places::PriceTip;
use super::tool_tip::ToolTip;
use super::widgets::{follow_cursor, tooltip};
use crate::campaign::lexique::species_lines;
use crate::combat::Enemy;
use crate::combat::boss::Borne;
use crate::i18n::{number, t, tf};
use crate::ink::Selection;
use crate::level::Level;
use crate::map;

/// The words of the enemy standing on the hovered cell, next to the cursor.
#[derive(Component)]
pub struct EnemyCard;

/// How far from the hovered cell's centre an enemy still counts as on it, in tiles.
const NEAR: f32 = 0.5;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((EnemyCard, tooltip(&assets)));
}

/// The tip with the hovered cell's price (`places`), hidden under the card.
type PriceTipQuery<'w, 's> =
    Query<'w, 's, &'static mut Visibility, (With<PriceTip>, Without<ToolTip>, Without<EnemyCard>)>;

/// Writes the card of the enemy nearest to the hovered cell's centre, unless a tool's tip is up.
pub fn enemy_card(
    level: Res<Level>,
    selection: Res<Selection>,
    window: Single<&Window, With<PrimaryWindow>>,
    enemies: Query<(&Enemy, &Transform), Without<Borne>>,
    tool_tip: Single<&Visibility, (With<ToolTip>, Without<EnemyCard>)>,
    card: Single<(&mut Text, &mut Node, &mut Visibility), With<EnemyCard>>,
    mut price: PriceTipQuery,
) {
    let (mut text, mut node, mut visibility) = card.into_inner();
    let centre = selection.hovered.map(|i| map::cell_pos(level.terrain.grid(), i));
    let nearest = centre.and_then(|c| {
        let flat = |p: Vec3| Vec2::new(p.x, p.z);
        enemies
            .iter()
            .map(|(enemy, t)| (enemy, flat(t.translation).distance(flat(c))))
            .filter(|&(_, d)| d <= NEAR)
            .min_by(|a, b| a.1.total_cmp(&b.1))
    });
    let (Some((enemy, _)), Some(cursor), true) = (nearest, window.cursor_position(), **tool_tip == Visibility::Hidden)
    else {
        *visibility = Visibility::Hidden;
        return;
    };
    let species = &level.species[enemy.species()];
    let (hp, max) = enemy.health();
    let mut lines =
        vec![capitalized(t(&species.label)), tf("lexique.hp_now", &[&number(hp.max(0.0).ceil()), &number(max)])];
    lines.extend(species_lines(species).iter().map(|l| l.shown()));
    let words = lines.join("\n");
    if text.0 != words {
        text.0 = words;
    }
    follow_cursor(&mut node, cursor);
    *visibility = Visibility::Inherited;
    // The cell's price would sit under the card, at the same spot.
    for mut tip in &mut price {
        *tip = Visibility::Hidden;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::grid::Grid;
    use crate::map::species;

    fn board(hovered: Option<usize>) -> (App, Entity) {
        let mut app = App::new();
        let level = Level::new(Grid::parse("S.M"), species::load());
        let chaineur = species::index_of(&level.species, "chaineur").unwrap();
        let at = map::cell_pos(level.terrain.grid(), 1);
        app.add_plugins(MinimalPlugins).insert_resource(Selection { hovered, ..Selection::for_level(&level) });
        app.world_mut().spawn((Enemy::new(chaineur, 100.0, 1), Transform::from_translation(at)));
        app.insert_resource(level);
        let mut window = Window::default();
        window.set_cursor_position(Some(Vec2::new(100.0, 50.0)));
        app.world_mut().spawn((window, PrimaryWindow));
        app.world_mut().spawn((ToolTip, Visibility::Hidden));
        let card = app.world_mut().spawn((EnemyCard, Text::default(), Node::default(), Visibility::Hidden)).id();
        app.add_systems(Update, enemy_card);
        (app, card)
    }

    #[test]
    fn an_enemy_on_the_hovered_cell_has_a_card() {
        let (mut app, card) = board(Some(1));
        app.update();
        assert_eq!(*app.world().get::<Visibility>(card).unwrap(), Visibility::Inherited);
        let words = &app.world().get::<Text>(card).unwrap().0;
        assert!(words.starts_with(&format!("Chaîneur\n{}", tf("lexique.hp_now", &[&100, &100]))), "{words}");
        assert!(words.contains(&tf("lexique.weak", &[&t("tool.arbalete")])), "{words}");
    }

    #[test]
    fn no_enemy_there_hides_the_card() {
        let (mut app, card) = board(Some(2));
        app.update();
        assert_eq!(*app.world().get::<Visibility>(card).unwrap(), Visibility::Hidden);
        let (mut app, card) = board(None);
        app.update();
        assert_eq!(*app.world().get::<Visibility>(card).unwrap(), Visibility::Hidden);
    }
}
