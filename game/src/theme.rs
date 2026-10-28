//! The shared palette and fonts: the UI's inks and papers, the world's tones, and the two
//! typefaces.

use bevy::prelude::*;

/// Ink: text, borders and the hachures on the relief and the hovered cell.
pub const INK: Color = Color::srgb(0.22, 0.14, 0.08);
/// Paper laid over the map: title, settings and pause.
pub const VEIL: Color = Color::srgba(0.93, 0.88, 0.76, 0.88);
/// Opaque paper: the splash.
pub const PAPER: Color = Color::srgb(0.93, 0.88, 0.76);
/// Secondary ink: what is said in passing.
pub const INK_SOFT: Color = Color::srgb(0.37, 0.32, 0.25);
/// The Chancellerie's wax: the one accent, on the action and on danger.
pub const SEAL: Color = Color::srgb(0.66, 0.2, 0.16);
/// The seal under the mouse or pressed, darker.
pub const SEAL_DEEP: Color = Color::srgb(0.56, 0.16, 0.13);
/// A button at rest: a faint wash of ink on the paper, a slip laid on the sheet rather than a ruled
/// frame.
pub const SLIP: Color = Color::srgba(0.55, 0.4, 0.22, 0.1);
/// A hovered button's wash.
pub const LIT: Color = Color::srgba(0.55, 0.4, 0.22, 0.25);
/// A pressed button's wash.
pub const PRESSED: Color = Color::srgba(0.55, 0.4, 0.22, 0.5);

/// The sheet under the world: the fog and the sky fade to it.
pub const SHEET: Color = Color::srgb(0.93, 0.89, 0.8);
/// Ink in the world: the engraving's lines.
pub const WORLD_INK: Color = Color::srgb(0.17, 0.12, 0.08);
/// Sépia, for le Calque and the goal.
pub const SEPIA: Color = Color::srgb(0.48, 0.29, 0.13);
/// The sun's colour: late-afternoon warm, against the sky's cool ambient (`AMBIENT`).
/// `hachure.wesl` reads both.
pub const SUNLIGHT: Color = Color::srgb(1.0, 0.94, 0.84);
/// The sky's cool ambient light.
pub const AMBIENT: Color = Color::srgb(0.9, 0.92, 1.0);
/// Water: lakes and the Eau tile, the resin's blue.
pub const WATER: Color = Color::srgb(0.45, 0.63, 0.76);

// The bocage's washes, as a hand-coloured map lays them under the ink
// (docs/direction-artistique.md, phase 6).
/// Dry pasture: the Sol of the board, the Marches' meadows.
pub const PASTURE: Color = Color::srgb(0.65, 0.74, 0.48);
/// Wet meadow, a bluer green: the Sol bas that la Crue drowns.
pub const MEADOW: Color = Color::srgb(0.53, 0.68, 0.53);
/// Ripe wheat and straw.
pub const WHEAT: Color = Color::srgb(0.84, 0.77, 0.52);
/// Hedgerows and the woods' floor, the deepest green.
pub const HEDGE: Color = Color::srgb(0.36, 0.5, 0.32);
/// The shadow a panel casts on the map, lifting the sheet off it.
pub const PANEL_SHADOW: Color = Color::srgba(0.22, 0.14, 0.08, 0.18);
/// Frames and borders drawn on the map.
pub const FRAME: Color = Color::srgb(0.36, 0.3, 0.24);
/// Wood: the barricade and the varnished props.
pub const WOOD: Color = Color::srgb(0.3, 0.2, 0.12);
/// Dark moss felt: the base a figurine stands on.
pub const FELT: Color = Color::srgb(0.30, 0.33, 0.24);
/// Cream linen: a figurine's collar and packs.
pub const LINEN: Color = Color::srgb(0.88, 0.83, 0.70);
/// Dark iron: chains and fittings on a figurine.
pub const IRON: Color = Color::srgb(0.25, 0.25, 0.27);
/// Ox hide: the Grande Chaîne's oxen.
pub const HIDE: Color = Color::srgb(0.42, 0.30, 0.20);
/// Slate cloth: the Chancellerie's coat, on the Chaîneur and on Gast.
pub const SLATE: Color = Color::srgb(0.30, 0.32, 0.40);
/// A lit lantern's glass.
pub const LAMP: Color = Color::srgb(0.93, 0.78, 0.38);
/// Cream limestone: the imperial bornes.
pub const LIMESTONE: Color = Color::srgb(0.86, 0.82, 0.72);

/// The game's two faces, regular and italic.
pub const FELL: [&str; 2] = ["fonts/IMFellEnglish-Regular.ttf", "fonts/IMFellEnglish-Italic.ttf"];
/// The readable face (Atkinson Hyperlegible), regular and italic.
pub const READABLE: [&str; 2] =
    ["fonts/AtkinsonHyperlegibleNext-Regular.ttf", "fonts/AtkinsonHyperlegibleNext-Italic.ttf"];

/// A page's title, over its sheet.
pub const TITLE_SIZE: f32 = 56.0;
/// A group's heading on a page (a column of settings, keys or a sceaux branch), and a panel's
/// title.
pub const HEADING_SIZE: f32 = 30.0;
/// The line under a title, a button's words, a panel's main line.
pub const SUBTITLE_SIZE: f32 = 26.0;
/// What a page says: its rows, notes and choices.
pub const BODY_SIZE: f32 = 22.0;
/// What is said in passing: keys, costs, notices, the credits' authors.
pub const SMALL_SIZE: f32 = 18.0;

/// The cut of the game's font; the order is `FELL`'s.
#[derive(Clone, Copy)]
pub enum Face {
    Roman,
    Italic,
}

/// Text in the game's font, in the cut `face`.
pub fn font(assets: &AssetServer, face: Face, size: f32) -> TextFont {
    TextFont {
        font: assets.load(FELL[face as usize]).into(),
        font_size: FontSize::Px(size),
        ..default()
    }
}
