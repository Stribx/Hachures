//! The cartouche's lines of news: what the world is about to do, and what the cursor reads.

use crate::campaign::skills::Skill;
use crate::i18n::{t, tf};
use crate::ink::{Selection, Verdict};
use crate::level::Level;

pub(super) fn notice(level: &Level) -> String {
    let mut lines = Vec::new();
    if let Some(phase) = &level.boss {
        lines.push(tf("hud.boss_phase", &[&t(&level.boss_file().name), &t(phase)]));
    }
    if let Some(event) = &level.world().event {
        if level.event_on {
            lines.push(t(&event.during).to_string());
        } else if level.event_next {
            lines.push(t(&event.announce).to_string());
        }
    }
    let next = level.wave.number + 1;
    let announced =
        level.changes.iter().filter(|c| c.wave == next).filter_map(|c| c.announce.as_ref());
    for announce in announced {
        let line = t(announce).to_string();
        if !lines.contains(&line) {
            lines.push(line);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::data::Change;
    use crate::map::grid::{Grid, Tile};
    use crate::map::species;

    /// The wave before a collapse, the cartouche says so, once however many cells fall.
    #[test]
    fn a_collapse_is_announced() {
        let mut level = Level::new(Grid::parse("S...M\n....."), species::load());
        let collapse = |x| Change {
            wave: 1,
            at: vec![(x, 1)],
            becomes: Tile::Gouffre,
            announce: Some("world.2.announce".into()),
            color: None,
        };
        level.changes = vec![collapse(1), collapse(3)];
        assert_eq!(notice(&level), t("world.2.announce"));
        level.changes[0].wave = 2;
        level.changes[1].wave = 2;
        assert_eq!(notice(&level), "", "not two waves ahead");
    }
}
