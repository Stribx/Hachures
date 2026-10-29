# Hachures — project overview

> **A student project demonstrating game development, systems programming, real-time graphics, and software
> engineering practices.**

## One-minute overview

**Hachures** is a 3D cartographic tower-defense game made with Rust and Bevy. The player does not mainly defend
the map with fixed walls: they redraw terrain with ink so that different populations choose different paths.
Each stroke changes the map, the available routes, the economy, and sometimes the story.

The project combines:

- a data-driven campaign with levels, worlds, bosses, skills, saves, and localization;
- a Dijkstra-style flow field for each species, so enemies react to the terrain they can actually read;
- a custom visual identity inspired by engraved maps and paper documents;
- desktop builds for Linux, macOS, and Windows plus a WebAssembly build for the browser;
- automated tests, deterministic balance checks, CI/CD, release packaging, and contributor documentation.

The repository is deliberately more than a prototype. It shows how a student project can be structured so that
new features are explainable, testable, reviewable, and buildable by someone who did not create the original
code.

## Project goal and design reasoning

The central design question was:

> How can a tower-defense game make changing the map more interesting than simply placing stronger towers?

The answer was to make terrain the main strategic resource:

1. A species evaluates the map according to its own rules: slope, shadow, destination, barriers, or special
   movement abilities.
2. The player spends ink to change cells, create obstacles, redirect flow, or reveal information.
3. The game recalculates the consequences instead of following a fixed enemy path.
4. The player must preserve **le Droit de Passage**: a solution that closes every route is rejected.
5. The result is a system where geometry, pathfinding, economy, combat, and narrative choices reinforce each
   other.

This design created useful engineering constraints. A local change to terrain can affect pathfinding, combat,
visual previews, save data, the balance table, and the user interface. The architecture and tests were therefore
designed around explicit messages, deterministic data, and narrow feature modules.

## Development process

### 1. Start with a design source of truth

`docs/bible.md` defines the game rules and fiction. `docs/roadmap.md` records what is complete, what is next,
and which open questions still require a design decision. This avoids implementing a rule from memory or letting
the code silently redefine the game.

The workflow is:

1. identify the desired player behavior;
2. locate the relevant bible section and roadmap item;
3. decide which data, systems, messages, UI, and tests are involved;
4. implement the smallest coherent change;
5. run focused tests and the full local gate;
6. update the roadmap or documentation when the project state changes.

### 2. Split the game into understandable layers

The Rust code is organized by feature and dependency direction:

`schedule → settings → level → campaign → map → ink → combat → overworld → ui | menu → shot`

The important rule is that one feature does not call another feature's systems directly. Features communicate
through messages such as `LoadLevel`, `Travel`, `WalkTo`, and `TowerInked`. This makes system ordering explicit
and keeps Bevy's ECS schedule easier to reason about.

The repository also enforces:

- one clear responsibility per file;
- a limit on code per file and per function;
- named data types instead of unclear tuples or booleans;
- saved data behind a storage module;
- player-visible text in localization files;
- tests at the end of the relevant file or in a sibling test module.

These are not aesthetic rules only: they reduce the cost of reviewing a change and make regressions easier to
localize.

For orientation in a growing codebase, the repository map documents the module tree and a local Graphify code
graph can be regenerated when a visual dependency overview is useful. This is especially helpful when reviewing
Bevy systems, feature boundaries, and asset-loading paths.

### 3. Make behavior data-driven

Levels, worlds, species, towers, waves, story notes, and language strings are represented as structured data,
including RON files where appropriate. Static campaign data is compiled in for reliable browser builds, while
runtime assets such as models, audio, fonts, and shaders are loaded through Bevy's asset system.

This separation makes it possible to add content without rewriting the simulation. It also creates natural
validation points: every level must be playable, every language must contain the required keys, and every species
must have compatible movement data.

### 4. Test rules, not implementation details

The test suite protects design rules such as:

- pathfinding and the Droit de Passage;
- terrain and weather transformations;
- tower strengths, weaknesses, armor, and special abilities;
- wave timing and boss phases;
- save compatibility and migration behavior;
- localization completeness;
- playable levels and content consistency;
- UI calculations and developer tools.

The balance test runs a deterministic strategy through the campaign and compares the resulting table with the
checked-in expected table. If a gameplay or data change unexpectedly changes the economy, victories, losses, or
wave behavior, CI catches it.

## Technical skills demonstrated

### Rust

- Rust 2024 workspace and package management with Cargo.
- Ownership-aware data structures, enums, typed state, iterators, and error-aware boundaries.
- Unit tests, integration tests, ignored balance tests, compile-time included data, and platform-specific
  configuration.
- Clippy configured as a hard CI gate with `-D warnings`.
- Rust formatting and code-structure tests.
- Native and `wasm32-unknown-unknown` compilation.
- Save files and migration-aware persistence.

### Bevy and ECS architecture

- Plugins and systems organized by feature.
- Components, resources, events/messages, queries, run conditions, and explicit system ordering.
- 3D cameras, lights, fog, meshes, materials, scenes, asset loading, and render preparation.
- Runtime generation of terrain, roads, props, barricades, vegetation, figurines, towers, and effects.
- Warm-up and readiness systems so models and shaders are prepared before the player reaches screens that need
  them.
- A development harness for screenshots, deterministic scenes, level selection, waves, camera positions,
  hover states, and performance logging.

### Algorithms and game systems

- Dijkstra-style flow fields for species-specific movement.
- Grid and cell modeling for terrain and map edits.
- Rule-based path validity checks.
- Deterministic waves, combat resolution, bosses, status effects, and terrain events.
- Data-driven campaign progression, skills, choices, and unlocks.
- A save model that separates campaign progress from player settings.

### Graphics, shaders, and 3D

The project uses a deliberately coherent visual direction: a 3D low-poly world rendered as an illuminated,
hand-inked map.

Technical work includes:

- GLB/GLTF model loading for towers and world pieces;
- procedural Bevy meshes for terrain, roads, fields, vegetation, barricades, tools, figures, and effects;
- a custom extended material (`InkMaterial`) layered on Bevy's standard material;
- WESL shaders for the engraved hachure surface, paper/fullscreen treatment, and wind deformation;
- prepass and vertex-shader support for animated wind;
- material variants for ink, fine ink, varnish, water, weighted terrain, and engraved surfaces;
- camera framing, depth fog, depth of field, and a dedicated presentation/studio scene;
- mesh and material reuse to avoid rebuilding identical resources unnecessarily;
- asset credits and license tracking for models, sounds, fonts, and generated art.

The graphics goal was not photorealism. It was visual consistency: the map, paper, terrain, models, UI, and
effects should feel like parts of the same physical object.

### Web and platform delivery

- Browser build through `wasm32-unknown-unknown` and `wasm-bindgen`.
- Browser-specific storage through `localStorage`.
- Native release builds for Linux x86_64, macOS ARM64, macOS Intel, and Windows x86_64.
- Release archives containing the executable and the runtime asset tree.
- Tag-driven GitHub Releases with desktop and browser artifacts.
- Cargo caching, obsolete-run cancellation, path filtering, and parallel matrix jobs to reduce CI duration and
  GitHub Actions usage.

## AI-assisted engineering and agent rules

AI was used as an engineering aid, not as a replacement for design ownership or verification. The repository
contains explicit rules so that generated suggestions remain compatible with the project's architecture.

### Repository-level rules

`AGENTS.md` defines the non-negotiable project contract:

- the bible wins when code and design disagree;
- generated browser output and build directories are not edited;
- features communicate through messages;
- player text belongs in localization;
- saves require migration when their meaning changes;
- Rust style, file-size, function-size, and testing rules are checked.

This turns project knowledge into a reusable context for every contributor or coding agent.

### Specialized sub-agents

`.claude/agents/` contains narrowly scoped roles rather than one general prompt for every task:

| Agent | Responsibility |
|---|---|
| `finder` | Locate symbols, files, assets, and end-to-end flows before editing. |
| `implementer` | Apply an already-decided implementation or make a known compiler failure green. |
| `mechanic` | Perform mechanical multi-file edits such as consistent data changes. |
| `test-writer` | Add focused regression tests following repository conventions. |
| `reviewer` | Review diffs for correctness, Bevy pitfalls, save breaks, i18n gaps, and architecture violations. |
| `runner` | Run noisy builds, tests, and checks while returning a concise result. |
| `translator` | Update player-visible text across every language file. |
| `web-lookup` | Research external APIs and migration information when needed. |

The design principle is specialization with bounded responsibility. A finder should not redesign the game, a
mechanical editor should not choose rules, and a reviewer should not silently rewrite the patch. This reduces
hallucinated scope and makes AI output easier to inspect.

### Human responsibility and verification

The human developer remains responsible for:

- deciding whether a feature belongs in the bible and roadmap;
- checking that an AI proposal matches the existing architecture;
- reviewing every diff;
- running tests and visual checks;
- validating licenses and generated assets;
- deciding whether a behavior change is intended.

The anti-regression suite is the acceptance mechanism. An AI-generated patch is not considered successful
because it compiles; it must preserve the rules, pass tests, and explain its impact.

## CI/CD and quality engineering

The CI workflow is intentionally split:

1. **Quality job:** format, clippy, tests, balance regression, and WebAssembly compilation on Linux.
2. **Native matrix:** release compilation on Linux, macOS ARM64, macOS Intel, and Windows.
3. **Release workflow:** on a semantic version tag, package native binaries and the web build, then publish a
   GitHub Release.

The local developer command is:

```sh
./scripts/ci-local.sh
```

It runs the same fast checks as the Linux quality job. The current baseline is:

- 236 passing tests and 1 intentionally ignored balance test in the normal suite;
- the balance regression test passes independently;
- formatting, clippy, and WebAssembly checks pass;
- measured test coverage baseline: 63.57% line coverage, 67.56% function coverage, 59.73% region coverage.

Coverage is recorded as a baseline for visibility, not yet as a blocking CI gate. Instrumented coverage builds
are much slower than normal checks, so adding a mandatory coverage job would conflict with the project's goal of
keeping feedback fast. The useful next step is to increase coverage around UI flows, settings application,
runtime spawning, and the developer harness, where the baseline is lower than the pure simulation modules.

## Soft skills shown by the project

These are demonstrated through repository practices rather than claimed as abstract qualities:

- **Product thinking:** translates a creative premise into a repeatable core loop and explicit player rules.
- **Systems thinking:** traces how a terrain edit affects pathfinding, combat, economy, visuals, saves, and UI.
- **Technical communication:** records architecture, design decisions, contributor expectations, and known gaps
  in Markdown.
- **Prioritization:** uses a roadmap with weighted progress instead of presenting unfinished work as complete.
- **Quality ownership:** treats regression tests, balance checks, localization, and platform builds as part of
  feature delivery.
- **Collaboration:** provides issue templates, pull-request checklists, focused agent roles, and reproducible
  local commands for future contributors.
- **Responsible AI use:** constrains automation with repository rules, specialization, review, and measurable
  verification.
- **Trade-off reasoning:** keeps Docker optional because it improves Linux reproducibility but cannot validate
  native macOS/Windows graphics; keeps coverage visible without slowing every PR.
- **Open communication:** records open bible questions and known visual/runtime limitations rather than hiding
  uncertainty.

## Current state and honest limitations

The roadmap currently reports approximately **40%** overall completion. The prologue and worlds 1–2 are playable,
while worlds 3–6, several powers, endings, and bonus sheets remain planned. Known follow-up work includes a full
browser playtest, shader and audio verification in the browser, additional world content, and visual polish.

This is important portfolio context: the project demonstrates a working vertical slice and a growing production
foundation, not a finished commercial game. The engineering work is valuable precisely because it makes the next
features safer to add.

## Suggested recruiter summary

> Built **Hachures**, a 3D cartographic tower-defense game in Rust and Bevy. Designed a data-driven ECS
> architecture with species-specific flow-field pathfinding, terrain editing, procedural 3D meshes, custom WESL
> shaders, localization, save migration, and deterministic balance tests. Added GitHub Actions CI/CD for Linux,
> macOS ARM64/Intel, Windows, and WebAssembly releases, with caching and cancellation to reduce feedback time.
> Established contributor and AI-agent rules, focused sub-agents, regression testing, and reproducible local
> checks so the project could scale beyond its original author.
