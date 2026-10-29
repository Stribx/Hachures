//! The biomes the worlds lie in: the bocage, the Cols and the Combes.

/// The kind of biome a world lies in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Biome {
    /// World 1: hedged fields and meadows, wet with la Crue.
    Bocage,
    /// World 2: alpine meadows, cold with le Gel.
    Cols,
    /// World 3: karst plateaus and galleries.
    Karst,
    /// World 4: erg and salt pans.
    Desert,
}

impl Biome {
    /// The biome of world `world` (an index of `WORLDS`).
    ///
    /// # Panics
    ///
    /// For a world with no biome yet: a new world names its own biome.
    pub fn of(world: usize) -> Biome {
        match world {
            0 => Biome::Bocage,
            1 => Biome::Cols,
            2 => Biome::Karst,
            3 => Biome::Desert,
            _ => Biome::Desert,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::world::WORLDS;

    #[test]
    fn every_world_has_its_biome() {
        for world in 0..WORLDS.len() {
            Biome::of(world);
        }
    }
}
