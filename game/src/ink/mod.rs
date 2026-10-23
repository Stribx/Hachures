//! Nève's tools: what she may ink where, what it costs, and the click that inks it.

mod overlay;

use bevy::prelude::*;

use crate::Step;
use crate::campaign::load_level;
use crate::campaign::skills::Skill;
use crate::combat::towers::{Weapon, spawn_tower};
use crate::level::Level;
use crate::map::flow::{FlowField, can_place};
use crate::map::grid::{Grid, Tile};
use crate::map::hover;
use crate::ui::TreeOpen;

pub fn plugin(app: &mut App) {
    app.add_plugins(overlay::plugin).add_systems(Update, inking.in_set(Step::Input).after(hover).after(load_level));
}

pub struct Tool {
    pub key: KeyCode,
    pub name: &'static str,
    pub tile: Tile,
    /// Sépia (docs, section 8).
    pub cost: u32,
    pub weapon: Option<Weapon>,
}

const fn tool(key: KeyCode, name: &'static str, tile: Tile, cost: u32, weapon: Option<Weapon>) -> Tool {
    Tool { key, name, tile, cost, weapon }
}

pub const TOOLS: [Tool; 7] = [
    tool(KeyCode::Digit1, "barricade", Tile::Barricade, 10, None),
    tool(KeyCode::Digit2, "boue", Tile::Boue, 15, None),
    tool(KeyCode::Digit3, "ronces", Tile::Ronces, 20, None),
    tool(KeyCode::Digit4, "arbalète", Tile::Tour, 50, Some(Weapon::Arbalete)),
    tool(KeyCode::Digit5, "mortier", Tile::Tour, 90, Some(Weapon::Mortier)),
    tool(KeyCode::Digit6, "harpon", Tile::Tour, 70, Some(Weapon::Harpon)),
    // Not priced in the docs: tune with the balance bot.
    tool(KeyCode::Digit7, "lanterne", Tile::Tour, 60, Some(Weapon::Lanterne)),
];

/// Index in `TOOLS` of the tool called `name`.
pub fn tool_index(name: &str) -> Option<usize> {
    TOOLS.iter().position(|t| t.name == name)
}

/// Number keys pick a tool; a click inks it on the hovered cell.
fn inking(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    assets: Res<AssetServer>,
    tree: Res<TreeOpen>,
    mut level: ResMut<Level>,
    mut commands: Commands,
) {
    if level.finished || tree.0 {
        return;
    }
    if let Some(&tool) = level.tools.iter().find(|&&t| keys.just_pressed(TOOLS[t].key)) {
        level.tool = tool;
    }
    let Some(i) = level.hovered else { return };
    if !mouse.just_pressed(MouseButton::Left) || level.refusal(i).is_some() {
        return;
    }
    level.ink(i);
    if let Some(weapon) = TOOLS[level.tool].weapon {
        spawn_tower(&mut commands, &assets, &level.grid, i, weapon);
    }
}

impl Level {
    /// Sépia cost of tool `t`, skills included.
    pub fn tool_cost(&self, t: usize) -> u32 {
        let tool = &TOOLS[t];
        let factor = if tool.weapon == Some(Weapon::Arbalete) && self.has(Skill::MineDePlomb) {
            0.9
        } else if tool.tile == Tile::Barricade && self.has(Skill::Fascines) {
            0.7
        } else {
            1.0
        };
        (tool.cost as f32 * factor).round() as u32
    }

    /// Inks the current tool at `i`, paid in sépia. The caller has checked `refusal`.
    pub fn ink(&mut self, i: usize) {
        let tool = &TOOLS[self.tool];
        self.sepia -= self.tool_cost(self.tool);
        if !self.used.contains(&self.tool) {
            self.used.push(self.tool);
        }
        self.base.tiles[i] = tool.tile;
        if let Some(weapon) = tool.weapon {
            self.towers.push((i, weapon));
        }
        self.refresh();
    }

    /// The map and the shown species' field if the current tool were inked at `i`.
    pub fn what_if(&self, i: usize) -> (Grid, FlowField) {
        let mut grid = self.grid.clone();
        grid.tiles[i] = TOOLS[self.tool].tile;
        let field = FlowField::new(&grid, &self.species[self.shown]);
        (grid, field)
    }

    /// Why the current tool can't be inked at `i`, if it can't.
    pub fn refusal(&self, i: usize) -> Option<&'static str> {
        if !self.grid.tiles[i].buildable() {
            Some("on n'encre que la terre ferme")
        } else if self.sepia < self.tool_cost(self.tool) {
            Some("pas assez de sépia")
        } else if self.walkers.iter().any(|w| w.1 == i) {
            Some("une colonne passe ici")
        } else {
            self.closes(i, TOOLS[self.tool].tile)
        }
    }

    /// Le Droit de Passage: why `tile` at `i` would close the map, now or around the world event.
    fn closes(&self, i: usize, tile: Tile) -> Option<&'static str> {
        let event = &self.world().event;
        if self.changes.iter().any(|c| c.cells(self.base.width).any(|n| n == i)) {
            Some("le relief va changer ici")
        } else if !can_place(&self.grid, &self.species, &self.present, i, tile, &self.walkers) {
            Some("carte close : l'encre ne prend pas")
        } else if self.event_next
            && !self.event_on
            && !can_place(&self.base.under(event), &self.species, &self.present, i, tile, &[])
        {
            Some(&event.refuse_next)
        } else if self.event_on && !can_place(&self.base, &self.species, &self.present, i, tile, &[]) {
            Some(&event.refuse_after)
        } else if !self.changes.is_empty()
            && !can_place(&self.after_changes(), &self.species, &self.present, i, tile, &[])
        {
            // ponytail: checks the map after the changes, not after the changes under the event too.
            Some("le relief à venir fermerait la carte")
        } else {
            None
        }
    }

    /// Ronces-mères: as a wave starts, one bramble spreads to a neighbouring cell, if le Droit de Passage allows.
    pub fn grow_ronces(&mut self) {
        let base = &self.base;
        let spread = (0..base.tiles.len())
            .filter(|&i| base.tiles[i] == Tile::Ronces)
            .flat_map(|i| base.neighbours(i))
            .find(|&n| base.tiles[n].buildable() && self.closes(n, Tile::Ronces).is_none());
        if let Some(n) = spread {
            self.base.tiles[n] = Tile::Ronces;
            self.refresh();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::save::Save;
    use crate::map::species::{self, index_of};

    #[test]
    fn la_crue_is_checked_before_inking() {
        // Dry road on top, the other way around is low ground.
        let mut level = Level::new(
            Grid::parse(
                "
                S...M
                ,^^^,
                ,,,,,
                ",
            ),
            species::load(),
        );
        let road = 2;
        assert_eq!(level.refusal(road), None, "the low way around is open");
        level.event_next = true;
        assert_eq!(level.refusal(road), Some("la crue fermerait la carte"));

        level.base.tiles[5] = Tile::Tour;
        level.base.tiles[11] = Tile::Boue;
        let flooded = level.base.under(&level.world().event);
        assert_eq!(flooded.tiles[5], Tile::Tour, "towers stand through the flood");
        assert_eq!(flooded.tiles[11], Tile::Eau, "mud inked on low ground still drowns");
        assert_eq!(flooded.tiles[10], Tile::Eau);
    }

    /// A scripted avalanche: le Droit de Passage counts it before it falls, then it falls on time.
    #[test]
    fn avalanche_is_counted_before_it_falls() {
        // Two roads; the avalanche will close the top one at (2, 0).
        let mut level = Level::new(
            Grid::parse(
                "
                S...M
                .^^^.
                .....
                ",
            ),
            species::load(),
        );
        level.waves = vec![vec![]];
        level.changes = vec![crate::level::data::Change { wave: 1, at: vec![(2, 0)], becomes: Tile::Roc }];
        assert_eq!(level.refusal(2), Some("le relief va changer ici"));
        assert_eq!(
            level.refusal(12),
            Some("le relief à venir fermerait la carte"),
            "the bottom road will be the only one"
        );
        level.start_wave();
        assert_eq!(level.base.tiles[2], Tile::Roc);
        assert!(level.changes.is_empty());
    }

    /// Le Gel (world 2): the lake freezes for two waves, then thaws.
    #[test]
    fn le_gel_freezes_the_lake() {
        let mut level = Level::new(
            Grid::parse(
                "
                S.~.M
                ",
            ),
            species::load(),
        );
        level.world = 1;
        level.events = vec![2];
        let ice = level.base.under(&level.world().event);
        assert_eq!(ice.tiles[2], Tile::Glace);
        assert_eq!((1..=4).map(|w| level.event_at(w)).collect::<Vec<_>>(), [false, true, true, false], "two waves");
    }

    #[test]
    fn skills_change_costs() {
        let save = Save {
            skills: vec![Skill::MineDePlomb, Skill::BoueProfonde, Skill::RoncesMeres, Skill::Fascines],
            ..Default::default()
        };
        let level = Level::load(0, &save);
        let cost = |name: &str| level.tool_cost(tool_index(name).unwrap());
        assert_eq!((cost("arbalète"), cost("barricade"), cost("boue")), (45, 7, 15));
        let chaineur = &level.species[index_of(&level.species, "chaineur").unwrap()];
        assert_eq!(chaineur.cost(Tile::Boue), 4.0, "Boue profonde: 3 + 1");

        let mut level = level;
        let ronces = |level: &Level| level.base.tiles.iter().filter(|&&t| t == Tile::Ronces).count();
        let before = ronces(&level);
        level.grow_ronces();
        assert_eq!(ronces(&level), before + 1, "Ronces-mères: one more bramble per wave");
    }
}
