//! The cartouche's lines of news: what the world is about to do, and what the cursor reads.

use crate::campaign::skills::Skill;
use crate::i18n::{t, tf};
use crate::ink::{Selection, Verdict};
use crate::level::Level;

pub(super) fn notice(level: &Level) -> String {
    let event = &level.world().event;
    let mut lines = Vec::new();
    if let Some(phase) = &level.boss {
        lines.push(tf("hud.boss_phase", &[&t(&level.boss_file().name), &t(phase)]));
    }
    if level.event_on {
        lines.push(t(&event.during).to_string());
    } else if level.event_next {
        lines.push(t(&event.announce).to_string());
    }
    lines.join("\n")
}

pub(super) fn readout(level: &Level, selection: &Selection, verdict: &Verdict) -> String {
    let Some(i) = verdict.at else { return String::new() };
    let mut lines = Vec::new();
    // Lecture fine: what the pencil line costs the shown species, before and after.
    if level.has(Skill::LectureFine)
        && let Some((what_if, field)) = &verdict.pencil
        && let Some(&spawn) = what_if.spawns_for(&level.species[selection.shown]).first()
    {
        let (before, after) =
            (level.terrain.fields()[selection.shown].dist[spawn], field.dist[spawn]);
        lines.push(tf("hud.line_cost", &[&before, &after]));
    }
    if selection.calque {
        let tile = level.terrain.grid().tiles[i];
        let costs: Vec<String> = level
            .present
            .iter()
            .map(|&k| {
                let (name, cost, goal) = (
                    t(&level.species[k].label),
                    level.species[k].cost(tile),
                    level.terrain.fields()[k].dist[i],
                );
                tf("hud.calque_species", &[&name, &cost, &goal])
            })
            .collect();
        lines.push(tf("hud.calque", &[&t(tile.name()), &costs.join(", ")]));
    }
    lines.join("\n")
}
