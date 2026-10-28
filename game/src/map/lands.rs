//! The two lands the worlds lie in: the bocage and the Cols.

/// The kind of land a world lies in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Land {
    /// World 1: hedged fields and meadows, wet with la Crue.
    Bocage,
    /// World 2: alpine meadows, cold with le Gel.
    Cols,
}

impl Land {
    /// The land of world `world` (an index of `WORLDS`).
    ///
    /// # Panics
    ///
    /// For a world with no land yet: a new world names its own.
    pub fn of(world: usize) -> Land {
        match world {
            0 => Land::Bocage,
            1 => Land::Cols,
            _ => panic!("world {world} has no land"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::world::WORLDS;

    #[test]
    fn every_world_has_its_land() {
        for world in 0..WORLDS.len() {
            Land::of(world);
        }
    }
}
