# Code structure

The full rules; `AGENTS.md` has the short version.

Checked by the build: break a rule and CI fails. Don't work around a check; change the code.

- **One file, one job.** The file's `//!` line names its job in one sentence. If the sentence needs an "and" between unrelated things, split the file. Related files share a folder with a `mod.rs`.
- **A file has at most 200 lines of code.** Blank lines, comments and the `#[cfg(test)]` module don't count. `game/tests/structure.rs` lists the files over the limit.
- **A function does one thing, in at most 70 lines.** `clippy::too_many_lines` enforces this; the threshold is in `clippy.toml`. A system reads, decides, and hands off to named helpers. A UI tree is built one section at a time (`spawn_header`, `spawn_row`, …).
- **Don't silence structure lints** (`too_many_lines`, `too_many_arguments`, `type_complexity`). Group `Local`s in a struct (`map::cells::Drawn`), group queries and resources in a `#[derive(SystemParam)]` (`settings::apply::Due`, `map::land::Landing`), or split the system.
- **Data may be imported from anywhere; behaviour follows the layers.**
  - Data means plain types, tables and pure functions, with no systems and no ECS access (`flow::fields`, `tool_index`): `Tile`, `Grid`, `Species`, `Weapon`, `TowerStats`, `Skill`, `TOOLS`, `Trigger`, `Save`, `BossFile`. The toolkit counts as data too: `theme`, `look`, `storage`, `i18n`, `ui::widgets`.
  - Systems, spawn functions and plugins are layered, from low to high: `schedule` → `settings` → `level` → `campaign` → `map` → `ink` → `combat` → `overworld` → `ui` and `menu` (peers: their buttons open each other's pages), with `shot` (the dev tool) on top. A module may use the behaviour of the layers below it, never above.
  - App-wide state (`Step`, `Screen`, `over_world`) lives in `schedule.rs`.
- **A feature never calls another feature's systems or spawn functions.** It sends a message instead: `LoadLevel`, `Travel`, `WalkTo`, `TowerInked`. Every level, a replay included, starts through `LoadLevel`.
- **Ordering against another feature's system is declared by the higher module.** `combat` orders itself `.before(level::finish_level)`; `level` doesn't name combat's systems. A run condition belongs to the module whose state it reads: `level` adds `Step::Simulate.run_if(alive)`.
- **What the player points at and has picked is not level state.** The tool, the route shown, le Calque and the hovered cell live in `ink::Selection`; `Level` holds the level and the run.
- **Shared values have one home.** Colours, fonts and the type scale live in `theme.rs`: text is `TITLE_SIZE` 56, `HEADING_SIZE` 30, `SUBTITLE_SIZE` 26, `BODY_SIZE` 22 or `SMALL_SIZE` 18; only display type (the game's name, the wave stamp) has its own size. A literal used in two places (a number, an asset path) becomes a named const next to its owner.
- **Persisted data goes through `storage`.** `Save` (campaign progress) and `Settings` (player preferences) are separate resources and separate files. Change a field's meaning and you must migrate the files saved before.
- **Tests** go at the end of the file in `#[cfg(test)] mod tests`, or in a sibling `tests.rs` when large. Neither counts toward the limits.
