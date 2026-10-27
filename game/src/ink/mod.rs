//! Nève's tools: what she may ink where, what it costs, and the click that inks it.

mod ghost;
pub mod overlay;
mod passage;
pub mod selection;

use bevy::prelude::*;

use crate::Step;
use crate::campaign::load_level;
use crate::campaign::skills::Skill;
use crate::combat::towers::Weapon;
use crate::i18n::n_;
use crate::level::Level;
use crate::level::story::MarginNote;
use crate::map::flow::FlowField;
use crate::map::grid::{Grid, Tile};
use crate::map::land::fibres;
pub use selection::Selection;

pub fn plugin(app: &mut App) {
    app.add_plugins((overlay::plugin, selection::plugin))
        .add_message::<TowerInked>()
        .add_message::<Scraped>()
        .add_systems(
            Update,
            inking.in_set(Step::Input).after(selection::hover).after(selection::reset).after(load_level),
        )
        .add_systems(Update, ghost::ghost.after(fibres).in_set(Step::Render));
}

/// A tower was inked on `cell`; combat stands it up.
#[derive(Message)]
pub struct TowerInked {
    pub cell: usize,
    pub weapon: Weapon,
}

/// A borne was scraped off `cell`; combat takes its piece down.
#[derive(Message)]
pub struct Scraped {
    pub cell: usize,
}

pub struct Tool {
    pub key: KeyCode,
    /// Its id, as the level files name it (`tools`, `Without`).
    pub name: &'static str,
    /// What the bar calls it: a key of `assets/data/i18n/`.
    pub label: &'static str,
    pub tile: Tile,
    /// Sépia (docs, section 8).
    pub cost: u32,
    pub weapon: Option<Weapon>,
    /// Scrapes decor off a cell instead of inking a tile on it (Gratter).
    pub scrapes: bool,
}

const fn tool(
    key: KeyCode,
    name: &'static str,
    label: &'static str,
    tile: Tile,
    cost: u32,
    weapon: Option<Weapon>,
) -> Tool {
    Tool { key, name, label, tile, cost, weapon, scrapes: false }
}

/// A power that scrapes decor off a cell, back to plain ground.
const fn scraper(key: KeyCode, name: &'static str, label: &'static str, cost: u32) -> Tool {
    Tool { key, name, label, tile: Tile::Sol, cost, weapon: None, scrapes: true }
}

pub const TOOLS: [Tool; 8] = [
    tool(KeyCode::Digit1, "barricade", n_("tool.barricade"), Tile::Barricade, 10, None),
    tool(KeyCode::Digit2, "boue", n_("tool.boue"), Tile::Boue, 15, None),
    tool(KeyCode::Digit3, "ronces", n_("tool.ronces"), Tile::Ronces, 20, None),
    tool(KeyCode::Digit4, "arbalète", n_("tool.arbalete"), Tile::Tour, 50, Some(Weapon::Arbalete)),
    tool(KeyCode::Digit5, "mortier", n_("tool.mortier"), Tile::Tour, 90, Some(Weapon::Mortier)),
    tool(KeyCode::Digit6, "harpon", n_("tool.harpon"), Tile::Tour, 70, Some(Weapon::Harpon)),
    // Not priced in the docs: tune with the balance bot.
    tool(KeyCode::Digit7, "lanterne", n_("tool.lanterne"), Tile::Tour, 60, Some(Weapon::Lanterne)),
    // Not priced in the docs: tune with the balance bot.
    scraper(KeyCode::Digit8, "gratter", n_("tool.gratter"), 25),
];

/// Index in `TOOLS` of the tool called `name`.
pub fn tool_index(name: &str) -> Option<usize> {
    TOOLS.iter().position(|t| t.name == name)
}

/// Number keys pick a tool; a click inks it on the hovered cell.
pub(crate) fn inking(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut level: ResMut<Level>,
    mut selection: ResMut<Selection>,
    mut inked: MessageWriter<TowerInked>,
    mut scraped: MessageWriter<Scraped>,
) {
    if level.finished {
        return;
    }
    if let Some(&tool) = level.tools.iter().find(|&&t| keys.just_pressed(TOOLS[t].key)) {
        selection.tool = tool;
    }
    let Some(i) = selection.hovered else { return };
    if !mouse.just_pressed(MouseButton::Left) || level.refusal(i, selection.tool).is_some() {
        return;
    }
    level.ink(i, selection.tool);
    if let Some(weapon) = TOOLS[selection.tool].weapon {
        inked.write(TowerInked { cell: i, weapon });
    }
    if TOOLS[selection.tool].scrapes {
        scraped.write(Scraped { cell: i });
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

    /// Inks `tool` at `i`, paid in sépia, and in a Name if a place's name is written there; a scraping tool
    /// takes the borne off `i` instead.
    /// The caller has checked `refusal`.
    pub fn ink(&mut self, i: usize, tool: usize) {
        let piece = &TOOLS[tool];
        self.sepia -= self.tool_cost(tool);
        if !self.used.contains(&tool) {
            self.used.push(tool);
        }
        // The cell's type doesn't change, so no Name is lost.
        if piece.scrapes {
            self.base.bornes.retain(|&b| b != i);
            self.refresh();
            return;
        }
        if let Some(p) = self.place_at(i) {
            self.erased.push(p);
            self.names -= 1;
            let line = n_("note.erased").to_string();
            self.margin.push(MarginNote { line, name: Some(self.places[p].name.clone()) });
        }
        self.base.tiles[i] = piece.tile;
        if let Some(weapon) = piece.weapon {
            self.towers.push((i, weapon));
        }
        self.refresh();
    }

    /// The map and the shown species' field if `tool` were inked at `i`.
    pub fn what_if(&self, i: usize, tool: usize, shown: usize) -> (Grid, FlowField) {
        let mut grid = self.grid.clone();
        if TOOLS[tool].scrapes {
            grid.bornes.retain(|&b| b != i);
        } else {
            grid.tiles[i] = TOOLS[tool].tile;
        }
        let field = FlowField::new(&grid, &self.species[shown]);
        (grid, field)
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
        assert_eq!(level.refusal(road, 0), None, "the low way around is open");
        level.waves = vec![vec![]; 3];
        level.events = vec![2];
        assert_eq!(level.refusal(road, 0), Some("world.1.refuse_next"));

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
        assert_eq!(level.refusal(2, 0), Some(n_("refusal.relief_changes")));
        assert_eq!(level.refusal(12, 0), Some("refusal.relief_closes"), "the bottom road will be the only one");
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

    /// Le prix du nom: inking over a named place costs one Name, once, however many of its cells are inked.
    #[test]
    fn inking_a_place_scratches_its_name() {
        let mut level = Level::new(Grid::parse("S.....M"), species::load());
        level.places = vec![crate::level::data::Place { name: "place.pre_commun".into(), at: vec![(2, 0), (3, 0)] }];
        assert_eq!(level.place_at(2), Some(0));
        let boue = tool_index("boue").unwrap();
        level.ink(2, boue);
        level.ink(3, boue);
        assert_eq!(level.names, -1);
        assert_eq!(level.place_at(3), None, "the name is gone");
        assert!(level.margin.last().unwrap().shown().contains("le Pré-Commun"));

        let mut save = Save::default();
        level.finish(&mut save);
        assert_eq!(save.levels[0].erased, ["place.pre_commun"], "the carnet and the Grand Plan remember it");
    }

    #[test]
    fn skills_change_costs() {
        let save = Save {
            skills: vec![Skill::MineDePlomb, Skill::BoueProfonde, Skill::RoncesMeres, Skill::Fascines],
            ..Default::default()
        };
        // 1-1, after the prologue: it has brambles to spread.
        let level = Level::load(1, &save);
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
