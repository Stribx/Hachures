# Hachures: project overview

Hachures is a 3D tower defense in Rust and Bevy. You rarely defend the map with walls of towers: you redraw
the relief with ink so that each people of enemies chooses another road. It is a student project, built to
learn game programming, real-time graphics and how to keep a codebase workable.

## The idea

Most tower defenses ask where to put a stronger tower. Here the question is what to do to the map. Each
species reads the land its own way (by the slope, by the shadow, by its goal), so one stroke can send the
marching column one way and the mule train another. A stroke costs ink and erases a name from the land. One
rule holds: the map may never be closed (*le Droit de Passage*), and a stroke that closes every route is
refused. The story and rules are in [`bible.md`](bible.md) (French).

## How it works

- **Flow fields.** Each species has its own Dijkstra flow field over the grid. When the player inks a cell,
  the fields are recomputed and the walkers follow the new costs. The Droit de Passage check runs the same
  search before a stroke is accepted.
- **Layers.** The code is one folder per feature, in a fixed dependency order:
  `schedule → settings → level → campaign → map → ink → combat → overworld → ui | menu → shot`. Features talk
  through messages (`LoadLevel`, `Travel`, `WalkTo`, `TowerInked`) rather than calling each other.
  [`agents/map.md`](agents/map.md) lists where everything lives.
- **Data.** Levels, worlds, species, towers, waves, story notes and languages are RON files, so content is
  added without touching the simulation. Player-visible text is in French and English files, never in code.
- **Look.** A low-poly world drawn as an engraved map: a custom material with a WESL hachure shader, wind
  in the vertex shader, paper and depth treatment on the camera. See
  [`direction-artistique.md`](direction-artistique.md).
- **Saves.** Campaign progress and settings go through one `storage` module (a file on desktop,
  `localStorage` in the browser). Changing a saved field means writing a migration.

## Checks

The repository enforces its own rules, so a change can be judged without reading all of it:

- `cargo fmt`, `cargo clippy -D warnings`, unit tests, and a structure test (at most 200 lines of code per
  file, 70 per function);
- tests for the rules: flow fields and the Droit de Passage, terrain and weather, tower strengths, wave
  timing, boss phases, save migration, and "every language has every key";
- a balance bot that plays the whole campaign and compares its result with a checked-in table
  (`game/tests/balance.txt`), so an unintended change to the economy shows up in CI;
- a browser package check and native builds for Linux, macOS (Apple Silicon and Intel) and Windows.

`./scripts/ci-local.sh` runs the fast part locally; the GitHub workflows run the same plus the platform builds
and, on a version tag, publish the release packages. [`../CONTRIBUTING.md`](../CONTRIBUTING.md) has the Git
workflow.

## How AI was used

AI coding assistants helped write and review code, and some 2D art was generated (the recipes are in
[`art/prompts.md`](art/prompts.md), the sources in [`../game/assets/CREDITS.md`](../game/assets/CREDITS.md)).
The design, the rules in the bible and the decision to accept a change stay with the author. To keep
assistants inside the architecture, [`../AGENTS.md`](../AGENTS.md) states the project rules and
`.claude/agents/` holds a few narrow helper roles (search, mechanical edits, test writing, review). Tests and
CI, not the assistant, decide whether a change is good.

## State at v0.1.0

`v0.1.0` is the first technical playable release: the campaign through Worlds 1–6, saves and progression,
French and English, native packages and the browser build. It is not the finished game. Bonus sheets are
data foundations, Signer is a preview, Heroic mode and the four ending consequences are incomplete, the
Treuil and defection branches are unfinished, and the audio and final art passes are still to do. The
checklist is in [`releases/v0.1.0.md`](releases/v0.1.0.md) and progress in [`roadmap.md`](roadmap.md).
