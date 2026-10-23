//! What Nève reads on the sheet: routes in ink, the pencil line, le Calque and the world event's shiver.

use bevy::prelude::*;

use crate::Step;
use crate::level::Level;
use crate::map::cell_pos;

/// Gizmos for the dotted pencil line (preview of the detour).
#[derive(Default, Reflect, GizmoConfigGroup)]
struct Pencil;

pub fn plugin(app: &mut App) {
    app.insert_gizmo_config(
        Pencil,
        GizmoConfig {
            line: GizmoLineConfig {
                style: GizmoLineStyle::Dotted,
                width: 4.0,
                ..default()
            },
            ..default()
        },
    )
    .add_systems(Update, draw.in_set(Step::Render));
}

fn draw(time: Res<Time>, level: Res<Level>, mut gizmos: Gizmos, mut pencil: Gizmos<Pencil>) {
    let grid = &level.grid;
    let at = |i| cell_pos(grid, i);
    let species = &level.species[level.shown];
    let field = &level.fields[level.shown];

    // What the next wave changes shivers: the cells of the world event in its colour,
    // those of a scripted change (an avalanche…) in white.
    let shiver = Vec3::X * (time.elapsed_secs() * 25.0).sin() * 0.03;
    let mut hachures = |i: usize, color: Color| {
        for k in [-0.25, 0.0, 0.25] {
            let from = at(i) + Vec3::new(k - 0.12, -0.2, 0.12) + shiver;
            gizmos.line(from, from + Vec3::new(0.24, 0.0, -0.24), color);
        }
    };
    let event = &level.world().event;
    if level.event_next && !level.event_on {
        let (r, g, b) = event.color;
        for i in (0..grid.tiles.len()).filter(|&i| grid.hit_by(event, i)) {
            hachures(i, Color::srgb(r, g, b));
        }
    }
    for change in level.changes.iter().filter(|c| c.wave == level.wave.number + 1) {
        for i in change.cells(grid.width) {
            hachures(i, Color::WHITE);
        }
    }

    // Le Calque: the shown species' flow field as arrows.
    if level.calque {
        for i in 0..grid.tiles.len() {
            if let Some(n) = field.next(grid, i) {
                gizmos.arrow(at(i), at(i).lerp(at(n), 0.6), Color::WHITE);
            }
        }
    }

    // Current route of the shown species, in ink.
    for &spawn in grid.spawns_for(species) {
        gizmos.linestrip(field.path(grid, spawn).into_iter().map(at), Color::BLACK);
    }

    let Some(i) = level.hovered else { return };
    let refused = level.refusal(i).is_some();
    let corner = |dx: f32, dz: f32| at(i) + Vec3::new(dx * 0.48, 0.0, dz * 0.48);
    gizmos.linestrip(
        [corner(-1., -1.), corner(1., -1.), corner(1., 1.), corner(-1., 1.), corner(-1., -1.)],
        if refused { Color::srgb(0.9, 0.1, 0.1) } else { Color::WHITE },
    );

    // Le trait de crayon: the route if the current tool were inked here.
    // ponytail: recomputed every frame; cache per hovered cell if maps get big.
    if !refused {
        let (what_if, field) = level.what_if(i);
        for &spawn in grid.spawns_for(species) {
            pencil.linestrip(field.path(&what_if, spawn).into_iter().map(at), Color::srgb(0.3, 0.2, 0.1));
        }
    }
}
