//! The nine tools Nève holds: their keys, names, prices and what each does to a cell.

use bevy::prelude::*;

use crate::combat::towers::Weapon;
use crate::i18n::n_;
use crate::map::grid::Tile;

/// What a tool does to the cell it is inked on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    /// Inks a tile: a barricade, mud, brambles.
    Lay(Tile),
    /// Stands a tower up.
    Raise(Weapon),
    /// Scrapes the cell (Gratter): an imperial borne comes off, an Éboulis or a Palissade returns to plain Sol.
    Scrape,
    /// Takes back one of Nève's own strokes (Biffer): the cell is what it was before.
    Biffer,
}

impl Act {
    /// The tile it leaves on the cell, if it changes it.
    pub const fn lays(self) -> Option<Tile> {
        match self {
            Act::Lay(tile) => Some(tile),
            Act::Raise(_) => Some(Tile::Tour),
            Act::Scrape | Act::Biffer => None,
        }
    }

    pub const fn weapon(self) -> Option<Weapon> {
        if let Act::Raise(weapon) = self { Some(weapon) } else { None }
    }
}

pub struct Tool {
    pub key: KeyCode,
    /// Its id, as the level files name it (`tools`, `Without`).
    pub name: &'static str,
    /// What the bar calls it: a key of `assets/data/i18n/`.
    pub label: &'static str,
    /// Sépia (docs, section 8).
    pub cost: u32,
    pub act: Act,
}

const fn tool(key: KeyCode, name: &'static str, label: &'static str, cost: u32, act: Act) -> Tool {
    Tool { key, name, label, cost, act }
}

pub const TOOLS: [Tool; 9] = [
    tool(KeyCode::Digit1, "barricade", n_("tool.barricade"), 10, Act::Lay(Tile::Barricade)),
    tool(KeyCode::Digit2, "boue", n_("tool.boue"), 15, Act::Lay(Tile::Boue)),
    tool(KeyCode::Digit3, "ronces", n_("tool.ronces"), 20, Act::Lay(Tile::Ronces)),
    tool(KeyCode::Digit4, "arbalète", n_("tool.arbalete"), 50, Act::Raise(Weapon::Arbalete)),
    tool(KeyCode::Digit5, "mortier", n_("tool.mortier"), 90, Act::Raise(Weapon::Mortier)),
    tool(KeyCode::Digit6, "harpon", n_("tool.harpon"), 70, Act::Raise(Weapon::Harpon)),
    // Not priced in the docs: tune with the balance bot.
    tool(KeyCode::Digit7, "lanterne", n_("tool.lanterne"), 60, Act::Raise(Weapon::Lanterne)),
    // Not priced in the docs: tune with the balance bot.
    tool(KeyCode::Digit8, "gratter", n_("tool.gratter"), 25, Act::Scrape),
    // Free: what it gives back is the price (docs, section 7). Last, so the balance bot's indices hold.
    tool(KeyCode::Digit9, "biffer", n_("tool.biffer"), 0, Act::Biffer),
];

/// Index in `TOOLS` of the tool called `name`.
pub fn tool_index(name: &str) -> Option<usize> {
    TOOLS.iter().position(|t| t.name == name)
}
