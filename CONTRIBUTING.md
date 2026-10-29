# Contributing to Hachures

This project is a cartographic tower defense written in Rust and Bevy. The game design is not inferred from
the implementation: `docs/bible.md` is authoritative. When code and the bible disagree, stop and resolve the
design question before changing behavior.

## Before changing code

1. Start from a GitHub issue or write a short proposal when the work changes a rule, save format, level,
   player-visible text, art direction, or architecture.
2. Read the relevant source of truth:
   - `docs/roadmap.md` for the current state and next work.
   - `docs/bible.md` for game rules and fiction.
   - `docs/agents/map.md` for where code, data, and assets belong.
   - `docs/agents/code-structure.md` and `docs/agents/rust-style.md` for implementation constraints.
   - `docs/agents/data.md` for levels, worlds, species, and towers.
   - `docs/agents/i18n.md` for player-visible text.
   - `docs/agents/dev-tools.md` for screenshots, performance, and balance checks.
3. Identify the change type before editing:
   - **Bug:** reproduce it, add a failing regression test, fix it, and keep the test.
   - **Feature:** define the rule and acceptance cases first, then implement and test it.
   - **Data/content:** update the data and its validation tests; update the roadmap when an item lands.
   - **Refactor:** preserve behavior and prove it with the existing suite.
   - **Performance:** record a before/after measurement and explain the workload used.
4. Keep the patch narrow. Do not mix unrelated cleanup, generated web output, screenshots, or `target/`.

## Local workflow

Clone or fork the repository, create a branch, and run the local gate:

```sh
git clone <url>
cd Hachures
git switch -c feat/short-description
./scripts/ci-local.sh
```

The script runs the same fast checks as CI: formatting, clippy, unit/integration tests, the ignored balance
regression test, and the WebAssembly check. It installs no dependencies and does not modify tracked files.

For focused work, use the smallest relevant command first:

```sh
cargo test -p game name_of_test
cargo test -p game balance -- --ignored --nocapture
cargo run --release -p game
./scripts/build-web.sh
python3 scripts/check-web-package.py website/play
```

For visual or performance changes, use the harness described in `docs/agents/dev-tools.md`. Do not judge a
harness run by its frame rate: use the logged GPU pass times and the in-game F3 performance overlay.

## Pull requests

Open a pull request against `main` only after the local gate passes. The PR description must state:

- the issue or design decision;
- the current project state and the intended change;
- what was tested locally, including commands and results;
- the regression test added or updated for behavior changes;
- visual, performance, save-migration, localization, and platform impact;
- any remaining uncertainty or manual verification needed.

CI checks formatting, clippy, tests, balance, WebAssembly compilation, and native release builds on Linux,
macOS ARM64, macOS Intel, and Windows. CI cancels obsolete runs and caches Cargo artifacts. Documentation-only
changes are intentionally filtered from CI.

Reviewers should reject a change that:

- contradicts `docs/bible.md` without an approved design update;
- changes behavior without a focused test or an explicit reason a test is impossible;
- changes a save field without migration;
- adds player-visible text without every language key;
- edits generated `website/play/` or `target/`;
- lacks evidence for a claimed performance improvement.

After merge, update `docs/roadmap.md` in the same change when a tracked item is completed. Include the commit
short hash beside the ticked item.

## Docker

Docker is optional and not the primary development path. It can provide a reproducible Linux toolchain, but it
cannot replace native macOS and Windows builds or graphics testing. The supported source of truth remains the
local script plus GitHub's OS matrix. Do not add a Docker dependency to every contributor's workflow unless
native setup becomes unreliable.
