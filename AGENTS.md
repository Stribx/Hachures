# Hachures

A cartographic tower defense: Rust 2024, Bevy 0.20, one crate in `game/`, also built for the browser
(`wasm32-unknown-unknown`). Code in `game/src/` (one folder per feature), what it loads in `game/assets/`,
generated browser output in `website/play/` (never edit it, nor `target/`).

## The bible wins

`docs/bible.md` (French) is the game design. **When the code and the bible disagree, the code has a
bug.** A rule changes in the bible first, and only with the owner's say; comments cite it as "docs,
section N". Art: `docs/direction-artistique.md`, `docs/3d-assets.md`, `docs/art-prompts.md`.

## Commands

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings     # CI fails on any warning
cargo test -p game <filter> --quiet           # smallest filter first
cargo test -p game balance -- --ignored       # the balance table must not move unless meant to
cargo run -p game --features dev
```

- **Never run `./scripts/build-web.sh`: the owner does.** `cargo check --target wasm32-unknown-unknown -p game` is fine.
- Keep output bounded and failures visible: `set -o pipefail; … 2>&1 | tail -n 40`. No large logs in context.
- Commit only when asked, ending with the attribution line the tool gives.

## Rules the build checks

Break one and CI fails; change the code, never silence the check. Full text: `docs/agents/code-structure.md`.

- One file, one job, named by its `//!` line; at most 200 lines of code per file, 70 per function.
- Data (plain types, tables, pure functions) may be used anywhere. Behaviour follows the layers
  `schedule → settings → level → campaign → map → ink → combat → overworld → ui | menu → shot`.
- Features talk through messages (`LoadLevel`, `Travel`, `WalkTo`, `TowerInked`), never by calling each
  other's systems. The higher module declares ordering against a lower one.
- No player-visible text in code or data files: i18n keys, in every language file (`docs/agents/i18n.md`).
- Saves go through `storage`; change a field's meaning and migrate old files.
- Rust style is Google's guide as application code (`docs/agents/rust-style.md`): no `unwrap()` outside
  tests, enums over unclear `bool`s, named fields over shared tuples, `for` over `for_each`, width 100.
- Tests sit at the end of the file (`#[cfg(test)] mod tests`) or in a sibling `tests.rs`. Write the
  failing test first for a bug or a rule.

## Read when you need it

| Task | Read |
|---|---|
| Where a module, folder or asset belongs; adding an asset | `docs/agents/map.md` |
| A level, world, species or tower file; adding a level | `docs/agents/data.md` |
| A word of the fiction (sépia, Cible, Biffer, la Crue…) | `docs/agents/glossary.md` |
| Screenshots, the harness env vars, the balance bot | `docs/agents/dev-tools.md` |
| Text the player reads, a new language | `docs/agents/i18n.md` |
| Issues, specs, triage, ADRs | `docs/agents/issue-tracker.md`, `triage-labels.md`, `domain.md` |
