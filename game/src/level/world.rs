//! A world of the campaign (`assets/data/worlds/`): what Nève defends there, its event and its
//! boss. Its texts are keys of `assets/data/i18n/<code>.ron`.

use std::sync::LazyLock;

use serde::Deserialize;

use crate::combat::boss::BossFile;
use crate::map::grid::Tile;

/// Every world, in order; a level file names its world by number (1 is `WORLDS[0]`).
pub static WORLDS: LazyLock<Vec<World>> = LazyLock::new(|| {
    [include_str!("../../assets/data/worlds/1.ron"), include_str!("../../assets/data/worlds/2.ron")]
        .iter()
        .map(|ron| ron::from_str(ron).expect("invalid world file"))
        .collect()
});

#[derive(Deserialize)]
pub struct World {
    /// As the Grand Plan titles it: "Les Marches de Saule" (`world.1.name`).
    pub name: String,
    /// The Cible, as the HUD names it mid-sentence: "le Moulin" (`world.1.defends`).
    pub defends: String,
    pub event: Event,
    #[serde(default)]
    pub boss: Option<BossFile>,
}

/// The world event (La Crue, Le Gel…), announced one wave ahead. While it lasts, the cells drawn
/// as one of `cells` on the map become `becomes`, unless inked with one of `spares`.
#[derive(Deserialize)]
pub struct Event {
    pub cells: Vec<Tile>,
    pub becomes: Tile,
    #[serde(default)]
    pub spares: Vec<Tile>,
    /// Waves it lasts once it starts.
    pub lasts: u32,
    /// HUD line the wave before.
    pub announce: String,
    /// HUD line while it lasts.
    pub during: String,
    /// Refusal: inking here would close the map once the event comes.
    pub refuse_next: String,
    /// Refusal: inking here would close the map once the event is over.
    pub refuse_after: String,
    /// Colour of the hachures shivering on its cells when it is announced.
    pub color: (f32, f32, f32),
}
