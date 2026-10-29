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

## Git workflow and agile milestones

Hachures uses a lightweight Git Flow adapted to a student project. A Git tag is created only when the
milestone is finished and reproducible; it is not used to hide unfinished work.

| Branch | Purpose | Allowed source | Merge target |
|---|---|---|---|
| `main` | Stable, releasable history only | `release/*`, `hotfix/*` | — |
| `develop` | Integration branch for the current milestone | `feature/*`, `fix/*`, `docs/*`, `chore/*` | — |
| `feature/<issue>-<name>` | One feature or vertical slice | `develop` | `develop` |
| `fix/<issue>-<name>` | One bug or regression fix | `develop` | `develop` |
| `docs/<issue>-<name>` | Documentation-only work | `develop` | `develop` |
| `chore/<issue>-<name>` | Tooling, CI, or maintenance | `develop` | `develop` |
| `release/v<major>.<minor>.<patch>` | Stabilization and release preparation | `develop` | `main` and back to `develop` |
| `hotfix/v<major>.<minor>.<patch>` | Urgent production correction | `main` | `main` and back to `develop` |

### Milestone `0.1.0`

`0.1.0` is the first portfolio milestone, not merely the current `Cargo.toml` value. Its scope and
acceptance checklist are maintained in [`docs/releases/v0.1.0.md`](docs/releases/v0.1.0.md). The milestone
is ready only when that checklist is complete, the local CI gate passes, the browser package and native
packages are smoke-tested, and the known limitations are documented.

The chosen scope is **option 1: a technical playable release**. It includes the current playable campaign
through Worlds 1–6, the existing save/progression systems, localization, CI/CD, native packages for Linux,
macOS, and Windows, and the browser package. It does not claim that every design-bible post-game feature
is complete.

The following are explicitly known limitations of `0.1.0`: bonus sheets are data foundations rather than
fully integrated post-game content; Signer is only a safe preview foundation; Heroic mode and the complete
four-ending consequences are not finished; Treuil and defection branches remain incomplete; and the final
audio and art pass remains future work. These limitations must appear in the GitHub release notes and
portfolio documentation.

### `0.1.0` acceptance checklist

- The campaign included in the release starts, can be played through World 6, and persists progress.
- French and English localization validation passes.
- Native release packages build for Linux x86_64, macOS Apple Silicon, macOS Intel, and Windows x86_64.
- The browser package builds and passes its HTTP/package smoke test.
- Formatting, Clippy, unit/integration tests, balance regression, and WASM compilation pass.
- The README, roadmap, release notes, and known limitations agree with the tagged build.
- No generated output, secret, or unreviewed asset is included in the release.

The milestone is rejected if a listed platform package cannot start, if the core campaign is blocked, if a
save migration is missing, or if CI is red. A missing post-game feature is not a release blocker when it is
one of the documented limitations above.

The normal sequence is:

1. Create or update the milestone issue, for example `Milestone 0.1.0`, with acceptance criteria and links
   to the relevant roadmap items.
2. Create `develop` from the current `main` once, then do all milestone work through short branches:
   `git switch -c feature/123-world-3 develop`.
3. Open pull requests into `develop`. Each PR must represent one coherent change and pass CI plus review.
4. When the milestone scope is code-complete, freeze feature work and create:
   `git switch -c release/v0.1.0 develop`.
5. On `release/v0.1.0`, allow only release blockers, documentation, version metadata, packaging, and
   manual validation fixes. Do not add new features.
6. Merge the release branch into `main` with a merge commit, tag the exact merge commit, and push both:

   ```sh
   git switch main
   git merge --no-ff release/v0.1.0 -m "Release v0.1.0"
   git tag -a v0.1.0 -m "Hachures v0.1.0"
   git push origin main v0.1.0
   ```

7. Merge the same release branch back into `develop`, then delete the release branch after the GitHub
   release is published. The tag must never move; a correction gets a new patch version.

### Pull-request rules

- Never commit directly to `main` or `develop`; protect both branches on GitHub.
- A PR targets `develop` by default. Only `release/*` and `hotfix/*` target `main`.
- Keep one issue, one branch, and one PR whenever practical. Link the issue with `Fixes #123` only when
  the PR truly completes it.
- The PR title follows `type(scope): short description`; examples:
  `feat(world-3): add karst boss slice` and `fix(save): migrate version 4`.
- Squash feature/fix PRs when the branch contains exploratory commits. Preserve release and hotfix merge
  commits so the release history remains visible.
- A failed CI check, unresolved review, missing test, missing localization, or undocumented limitation
  blocks merge.

### Versioning rules

Use Semantic Versioning:

- `0.1.0`: first coherent playable milestone;
- `0.1.x`: fixes, packaging corrections, and safe documentation corrections;
- `0.2.0`: new gameplay scope or a deliberately incompatible save/data change;
- `1.0.0`: the project is feature-complete for its declared public scope.

Update `game/Cargo.toml`, release notes, and the roadmap in the release PR. Save-format changes require a
migration and a test before the version tag is created.

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

Open a pull request against `develop` (only `release/*` and `hotfix/*` target `main`) once the local gate passes. The PR description must state:

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
