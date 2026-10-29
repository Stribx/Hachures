# Game data

The RON files under `game/assets/data/`. Read it before touching a level, world, species or tower file.

Each field is documented once, in the struct the file is read into; the RON file's header and comments
show it in use.

| File | Read as | Fields |
|---|---|---|
| `levels/<world>-<n>.ron` | `LevelData` | `level/data.rs` (with `Change`, `Place`); notes, challenges and choices in `level/story.rs`; `1-1.ron` shows them in use |
| `worlds/<n>.ron` | `World`, `Event` | `level/world.rs`; the boss: `BossFile`, `Phase` in `combat/boss.rs` |
| `species.ron` | `Species` | its header; `map/species.rs`. **A tile left out of `costs` is impassable** for that species |
| `towers.ron` | `TowerStats` | its header; `combat/towers.rs` |
| `i18n/<code>.ron` | | `docs/agents/i18n.md` |

- **Map characters** (`Grid::parse`): `.` Sol, `,` Sol bas, `b` Boue, `r` Ronces, `~` Eau, `g` Gué, `#` Barricade, `^` Roc, `e` Éboulis, `p` Palissade, `o` Gouffre. Markers stand on Sol: `S` spawn, `F` allies' start, `M` Cible, `B` Borne-Mère, `R` Refuge, `X` Sortie, `W` where the Refuge moves if `alt_refuge`'s choice went that way.
- **Adding a level**: write the file, add it to `LEVELS` in `level/data.rs` (the campaign order), give it an `at`, add its keys (`level.<id>.name`, its notes and places) to every i18n file, then run `cargo test -p game levels` (every level loads, and every species reaches its goal with and without the event) and the balance bot.
