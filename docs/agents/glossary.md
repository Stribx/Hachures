# Glossary

The fiction's words and where they live in code.

The game's world keeps its French names in code, data and files (they are names in the fiction); engineering words are English.

| Term | Meaning | In code |
|---|---|---|
| Nève | the cartographer the player plays | |
| les Marches | the land of the campaign | `overworld` |
| le Grand Plan | the Marches seen from above, Nève walking from sheet to sheet | `Screen::Plan` |
| a sheet (*feuille*) | one level's board | `Level`, `overworld::Sheet` |
| sépia | the ink every tool costs; enemies shot down pay it | `Level.sepia` |
| the Cible | the place a level defends (the Moulin…); the HUD shows what it still holds | `Goal::Cible`, `Level.lives` |
| le Rôle des Noms | the roll of names engraved at the Moulin; losing all `lives` loses the Rôle | `Level.names`, `Save::names` |
| a Name / *le prix du nom* | inking a named place *rayes* its name from the Rôle (*rayer*, never *gratter*) | `Place`, `Level.erased` |
| les sceaux | seals won per level (victory, challenge), spent in l'arbre des sceaux | `Save::sceaux`, `menu/sceaux.rs`, `campaign/skills.rs` |
| le carnet | Nève's notebook: the margin notes in play, the campaign read back at the end | `ui` margin, `menu/carnet.rs` |
| le Lexique | L: every tool and species met, with its filmed piece or figurine and its real numbers; an entry unlocks the first time it shows on the sheet (`???` and a faded silhouette before). The tool tooltip and the enemy card show the same lines | `Save.seen`, `campaign/lexique.rs`, `menu/lexique/` |
| le Calque | F1: the overlay of every species' route | `Selection.calque` |
| le Droit de Passage | nothing may be inked that leaves a land species without a path, now or on a map to come | `ink/passage.rs` |
| la Crue, le Gel | world 1 and 2 events, announced a wave ahead | `World.event`, `Level.event_on/next` |
| la Tension de la feuille | hidden difficulty gauge, −2 to 2 | `Level.tension` |
| Fuyards | non-hostile walkers heading for the Refuge; they bring Names | `Species.hostile == false` |
| Chaîneur, Échassier, Jalonneur, Gens du Col… | the imperial species | `data/species.ron` |
| la Grande Chaîne | world 1's boss, in phases | `combat/boss.rs`, `World.boss` |
| le Théodolite de Givre | world 2's boss, in phases: la Visée freezes the water in its axis, la Triangulation plants two masts (bornes) that seal the col if held, le Dégel forcé thaws all the ice | `combat/sighting.rs`, `World.boss` |
| bornes, Borne-Mère | imperial markers the boss plants; the Jalonneurs' goal | `Grid.bornes`, `Goal::BorneMere` |
| Gratter | a power: scrapes a borne off a cell (the ground under it stays), or returns an Éboulis or a Palissade to plain Sol, and nothing else (never Nève's own ink: that is Biffer's job). Never costs a Name and is never needed to win a level; a scrape is final for Nève. Earned by beating world 1's boss | `TOOLS` "gratter", `Act::Scrape`, `LevelData.grants` |
| Biffer / le repentir | a tool: takes back one of Nève's own strokes (tower or aménagement laid with `Level::ink`) and returns the cell to what it was. Wet ink (laid between waves, no wave sent since) refunds what it cost; dry ink half, rounded down. The Name it cost stays scratched out. Free; every level has it | `TOOLS` "biffer", `Act::Biffer`, `Level.inked`, `ink/biffer.rs` |
| Renommer | a power: changes a cell's kind, dry ground (Sol, Sol bas, Boue) to Eau, Eau or Gué to Sol, ice back to Eau until the world event ends. Costs sépia, and a Name on a named place; goes through le Droit de Passage when it makes water; a rename is final. Earned by beating world 2's boss | `TOOLS` "renommer", `Act::Rename`, `Grid::renamed`, `ink/rename.rs` |
| Éboulis | fallen stone on a cell, slow to walk; Gratter scrapes it | `Tile::Eboulis`, map char `e` |
| Palissade | an imperial barricade the level or world data lays: impassable, but never closes the map; Gratter scrapes it | `Tile::Palissade`, map char `p` |
| the terrain | the sheet's ground as drawn, as inked and as walked under the world event, and the routes over it. Every lasting change goes through it, as *the land changing* (a scripted change, a scrape: la Crue reads the new ground) or as *ink on the land* (the tools, the Chaussée, Ronces-mères: drowned on low ground unless spared) | `map::terrain::Terrain`, `Edit`, `Level::alter` |
| Pente, Ombre | how a species reads the land: by tile costs, or by the towers' shadow | `Species.reading` |
| Sol, Sol bas, Boue, Ronces, Eau, Gué, Roc, Glace, Chaussée | the tiles | `Tile` |
