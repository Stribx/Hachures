//! The title card at launch: two seconds, or until any key or click.

use bevy::prelude::*;

use super::{PAPER, Screen, page, text};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), spawn).add_systems(Update, leave.run_if(in_state(Screen::Splash)));
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Splash, PAPER)).with_children(|p| {
        p.spawn(text(&assets, "Hachures", 96.0));
        p.spawn(text(&assets, "un tower defense cartographique", 28.0));
    });
}

fn leave(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut shown: Local<f32>,
    mut next: ResMut<NextState<Screen>>,
) {
    *shown += time.delta_secs();
    if *shown > 2.0 || keys.get_just_pressed().next().is_some() || mouse.get_just_pressed().next().is_some() {
        next.set(Screen::Title);
    }
}
