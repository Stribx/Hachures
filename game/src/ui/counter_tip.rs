//! The counter tip: the words of the counter under the cursor, next to it.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::hud::{Counter, CounterTip, Tip, capitalized};
use super::widgets::show_tip;
use crate::i18n::{t, tf};
use crate::level::Level;

/// What a counter is, for a player who has only the icon and the number.
fn tip_text(counter: Counter, level: &Level) -> String {
    match counter {
        Counter::Sepia => t("hud.tip.sepia").to_string(),
        Counter::Cible => tf("hud.tip.cible", &[&capitalized(t(&level.world().defends))]),
        Counter::Noms => t("hud.tip.names").to_string(),
        Counter::Vague => t("hud.tip.wave").to_string(),
    }
}

/// Names the counter under the cursor, next to it.
pub fn counter_tip(
    level: Res<Level>,
    window: Single<&Window, With<PrimaryWindow>>,
    pairs: Query<(&Tip, &Hovered)>,
    tip: Single<(&mut Text, &mut Node, &mut Visibility), With<CounterTip>>,
) {
    let (mut text, mut node, mut visibility) = tip.into_inner();
    let hovered = pairs.iter().find(|(_, hovered)| hovered.get()).map(|(Tip(counter), _)| *counter);
    let words = hovered.map(|counter| tip_text(counter, &level));
    show_tip(words, window.cursor_position(), &mut text, &mut node, &mut visibility);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hovered_counter_is_named_at_the_cursor() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(Level::parse("S.M"));
        let mut window = Window::default();
        window.set_cursor_position(Some(Vec2::new(100.0, 50.0)));
        app.world_mut().spawn((window, PrimaryWindow));
        let tip = app
            .world_mut()
            .spawn((CounterTip, Text::default(), Node::default(), Visibility::Hidden))
            .id();
        let pair = app.world_mut().spawn((Tip(Counter::Sepia), Hovered(true))).id();
        app.add_systems(Update, counter_tip);

        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Inherited);
        assert_eq!(app.world().get::<Text>(tip).unwrap().0, t("hud.tip.sepia"));
        assert_eq!(app.world().get::<Node>(tip).unwrap().left, px(118));

        app.world_mut().entity_mut(pair).insert(Hovered(false));
        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Hidden);
    }
}
