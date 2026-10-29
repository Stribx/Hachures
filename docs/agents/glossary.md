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
| le Lexique | L: every tool and species met, with its real numbers; the tool tooltip and the enemy card show the same lines (docs, section 9) | `Save.seen`, `campaign/lexique.rs`, `menu/lexique/` |
| le Calque | F1: the overlay of every species' route | `Selection.calque` |
| le Droit de Passage | nothing may be inked that leaves a land species without a path, now or on a map to come | `ink/passage.rs` |
| la Crue, le Gel | world 1 and 2 events, announced a wave ahead | `World.event`, `Level.event_on/next` |
| les Effondrements | world 3's collapses: cells turn to Gouffre for good, announced a wave ahead in red; they ride the level's scripted changes, not an event | `Change`, `Level.changes`, `Tile::Gouffre` |
| la Tension de la feuille | hidden difficulty gauge, −2 to 2 | `Level.tension` |
| Fuyards | non-hostile walkers heading for the Refuge; they bring Names | `Species.hostile == false` |
| Chaîneur, Échassier, Jalonneur, Gens du Col… | the imperial species | `data/species.ron` |
| la Grande Chaîne | world 1's boss, in phases | `combat/boss.rs`, `World.boss` |
| le Théodolite de Givre | world 2's boss, in phases: la Visée freezes the water in its axis, la Triangulation plants two masts (bornes) that seal the col if held, le Dégel forcé thaws all the ice | `combat/sighting.rs`, `World.boss` |
| bornes, Borne-Mère | imperial markers the boss plants; the Jalonneurs' goal | `Grid.bornes`, `Goal::BorneMere` |
| Gratter | a power: scrapes a borne, an Éboulis or a Palissade off a cell, never Nève's own ink (docs, section 7) | `TOOLS` "gratter", `Act::Scrape`, `LevelData.grants` |
| Biffer / le repentir | a tool: takes back one of Nève's strokes; wet ink refunds all it cost, dry ink half, and the Name stays scratched out (docs, sections 7 and 9) | `TOOLS` "biffer", `Act::Biffer`, `Level.inked`, `ink/biffer.rs` |
| Renommer | a power: changes a cell's kind, land to water or water to land, at the price of a Name on a named place (docs, section 7) | `TOOLS` "renommer", `Act::Rename`, `Grid::renamed`, `ink/rename.rs` |
| la Sonde | world 3's tower: the only weapon that reaches a species walking underground | `Weapon::Sonde`, `Species.underground`, `combat/drill.rs` |
| Éboulis | fallen stone on a cell, slow to walk; Gratter scrapes it | `Tile::Eboulis`, map char `e` |
| Palissade | an imperial barricade the level or world data lays: impassable, but never closes the map; Gratter scrapes it | `Tile::Palissade`, map char `p` |
| the terrain | the sheet's ground as drawn, as inked and as walked under the world event, and the routes over it. Every lasting change goes through it, as *the land changing* (a scripted change, a scrape: la Crue reads the new ground) or as *ink on the land* (the tools, the Chaussée, Ronces-mères: drowned on low ground unless spared) | `map::terrain::Terrain`, `Edit`, `Level::alter` |
| Pente, Ombre | how a species reads the land: by tile costs, or by the towers' shadow | `Species.reading` |
| Sol, Sol bas, Boue, Ronces, Eau, Gué, Barricade, Roc, Glace, Chaussée, Gouffre | the tiles (and the ones above) | `Tile` |
