# Portfolio review: Hachures

A candid review of this repository as a portfolio piece for an internship or apprenticeship, written the way a
hiring manager and an HR partner would read it. It lists what the project shows, what is weak or unclear, and
the order in which to fix it. Claims are tied to a command or a file so a reader can check them.

## Summary

Hachures is a 3D tower defense in Rust and Bevy, solo, built over a few weeks part-time. The design (the
bible), the architecture rules, the art direction and the release process are the author's decisions; most of the
code was written with AI assistants and checked by the author, the tests and CI. The strongest signals are
engineering discipline (tests, rules enforced by the build, a reproducible release) and a coherent design. The
weakest are the lack of players or feedback, a public history that is short and AI-heavy, and a few unclear
points listed below. Verdict for an internship: a credible portfolio piece once the P0 items are done; the
interview has to show that the author understands the code.

## Evidence

| Claim | Check |
|---|---|
| About 19,500 lines of Rust in 137 files | `find game/src -name '*.rs' \| xargs cat \| wc -l` |
| 243 tests (unit, structure, balance) | `grep -rn "#\[test\]" game/src game/tests \| wc -l` |
| 25 levels over a prologue and 6 worlds | `ls game/assets/data/levels` |
| French and English with identical keys | `cargo test -p game i18n` |
| The whole campaign is played by a bot and compared with a table | `cargo test -p game balance -- --ignored` |
| No `unwrap()` outside tests (`combat/tests.rs` only) | `grep -rn "unwrap()" game/src` |
| Four native targets plus a browser build in CI | `.github/workflows/ci.yml`, `release.yml` |
| 96 commits, 93 of them between 2026-10-27 and 2026-10-30 | `git log --format=%ad --date=short \| sort \| uniq -c` |
| 84 commits carry an AI co-author trailer | `git log --format=%B \| grep -ci co-authored-by` |

## Skills and knowledge shown

"Stated" means the author says they can explain and change it live; an interview is where that gets tested.

| Area | What the repository shows | Evidence |
|---|---|---|
| Rust 2024 | Typed state, enums over booleans, no `unwrap()` in game code, `clippy -D warnings` | `AGENTS.md`, `clippy.toml` |
| Bevy and ECS | Feature plugins, a fixed layer order, features talking by messages, explicit system ordering | `game/src/schedule.rs`, `docs/agents/code-structure.md` |
| Algorithms | One Dijkstra flow field per species, recomputed on each stroke, plus the Droit de Passage check | `game/src/map/flow.rs`, `game/src/ink/passage.rs` |
| Graphics | A custom material, WESL hachure shader, wind in the vertex stage, depth and paper treatment | `game/assets/shaders/`, `game/src/look/` |
| Data-driven design | Levels, worlds, species and towers as RON files; text only through i18n keys | `game/assets/data/`, `docs/agents/data.md` |
| Persistence | A save file with versioned migrations; `localStorage` in the browser | `game/src/campaign/save.rs`, `storage.rs` |
| Quality | Rule tests, a structure test (file and function size), a balance regression, a web package smoke test | `game/tests/`, `scripts/` |
| Delivery | Four native packages, a browser package, tag-driven release, local CI script | `.github/workflows/`, `scripts/ci-local.sh` |
| Product and design | A design bible, a roadmap with a counted percentage, a scoped release with acceptance criteria | `docs/bible.md`, `docs/roadmap.md`, `docs/releases/v0.1.0.md` |
| Process | Git Flow with release branch, one branch per task, merge commits, a validation record | `CONTRIBUTING.md` |
| Working with AI | Written project rules, narrow helper roles, tests as the gate | `AGENTS.md`, `.claude/agents/` |

Stated by the author as explainable and changeable live: the flow fields, Bevy systems and ordering, the
shaders and material, the CI and web build. The author also states that the bible, the architecture rules, the
art direction and the Git and release flow are their own decisions.

## What is good

1. **The rules are enforced, not just written.** File and function size, layer order, i18n completeness and the
   balance table fail the build when broken. That is rarer than a long README.
2. **The design is real.** The bible gives the code something to be wrong against ("when the code and the bible
   disagree, the code has a bug"), and the roadmap counts progress item by item.
3. **A reproducible release.** One local script, one CI matrix, one tag workflow, and an acceptance checklist with
   a validation record that says what was run and by whom.
4. **Honest scope.** The known limitations are written down and repeated in the release notes.
5. **A verification story.** An AI-assisted change that added the version label to the title screen passed every
   test and the browser package check, and still crashed at startup (a duplicated `TextColor`). The author found
   it by running the release build, a regression test was added, and the checks were re-run. This is the
   argument for "the author verifies" and it is better told than asserted.

## What is weak or unclear

1. **AI share of the code.** 84 commits carry an AI co-author trailer and most work happened in four days. A
   reader will ask what the author actually did. The answer exists (design, rules, review, release) but is only
   implicit in the repository.
2. **A short, squashed history.** The first commit is "Initial import" on 2026-10-23; earlier work lives in a
   private repository. Reviewers cannot see the learning path.
3. **No players, no feedback.** Nobody but the author has played the game. There is no playtest note and no public
   demo link.
4. **CI has not run on GitHub yet.** The local gate passes, but there is no green badge or run to point at.
5. **Unfinished scope.** Bonus sheets, Signer, Heroic mode, the ending consequences and the Treuil branches are
   incomplete. They are documented, but the first screen of the README does not say "v0.1.0, vertical slice".
6. **Process heavier than product.** Many docs, agent roles and CI jobs for a solo game can read as
   over-engineering if the game is not easy to try.
7. **Third-party text without attribution.** `docs/rustguide.md` is a 1,801-line copy of Google's Rust style
   guide with no visible source or licence line.
8. **Internal tooling in the first look.** `.claude/` and `AGENTS.md` are useful to contributors and noise to a
   recruiter skimming the root. The local `.scratch/` issue tracker cannot be seen by anyone else.
9. **French-only design document.** `docs/bible.md` is the source of truth and is in French; an English-speaking
   reviewer cannot read the main design reasoning.
10. **Performance is not shown.** The repository has profiling hooks and a harness, but the README gives no
    frame-time or memory number.

## Fix plan

| Priority | Action | Done when |
|---|---|---|
| P0 | Push to GitHub; get CI green on all four platforms; add a CI badge | The Actions page shows a green run for the release candidate |
| P0 | Replace the Google guide copy with a link and a short "how we apply it" (already in `docs/agents/rust-style.md`), or add the source and licence line | No unattributed third-party text in `docs/` |
| P0 | Add a "How this was built" section to the README: what the author decided and checked, what AI assistants did, the title-panic story | A reader can answer "what did you do?" from the README |
| P1 | Publish a browser demo (GitHub Pages or itch.io) and put the link at the top of the README | The link opens the title screen |
| P1 | Record a 30–60 second GIF or video of one level | Embedded in the README |
| P1 | Run three playtests with people who have not seen the game; write one page of observations | `docs/playtests.md` exists and a fix came from it |
| P1 | Say "v0.1.0, vertical slice, Worlds 1–6" in the first lines of the README | The first screen states the scope |
| P2 | Add an English summary of the bible (rules, species, tools) | `docs/bible-summary.md` |
| P2 | Add a one-page architecture diagram: layers and messages | Linked from `project-overview.md` |
| P2 | Add a short frame-time table for one level on one machine | Numbers and the command that produced them |
| P2 | From now on, work through GitHub issues and pull requests (`Fixes #n`) so the history shows review | The next three changes are PRs |

## Interview preparation

Likely questions, and the evidence to bring:

1. **"The AI wrote it, what did you do?"** Show `docs/bible.md`, `AGENTS.md`, `docs/roadmap.md` and the validation
   record. Tell the title-panic story.
2. **"Walk me through one stroke."** Click, cost check, Droit de Passage search, ink applied, flow fields
   recomputed, walkers re-route. Open `ink/passage.rs` and `map/flow.rs`.
3. **"Why messages between features?"** Explicit ordering in the Bevy schedule, no hidden coupling; show
   `LoadLevel` and `TowerInked`.
4. **"What does the balance test protect?"** An unintended change to the economy or wave outcomes; show
   `game/tests/balance.txt`.
5. **"Why a 200-line limit?"** Reviewability; show `game/tests/structure.rs`. Be ready to say what it costs.
6. **"What would you change?"** Pick two items from this file's weak list and the fix.
7. **"What did you decide against?"** Examples: no Docker as the main path, no networked features, no post-game
   content in 0.1.0.
8. **"How did you test the browser build?"** Package and HTTP smoke test, and a manual run; explain what the smoke
   test cannot catch.
9. **"How do you handle a save format change?"** Versioned `Save`, migrations, tests in `campaign/save.rs`.
10. **"What did you learn?"** Have a specific answer about Bevy scheduling, shaders or release engineering.

## Questions for the author

These were not settled when this was written:

- Which two or three files can you rewrite from scratch without help, and which would you rather not be asked
  about?
- What was the hardest bug you found yourself, and how?
- Which design decision did you change after seeing it play?
- What do you want the next release (`0.2.0`) to prove: more content, more polish, or players?
- Who will play-test, and when?
- Is the Google style guide copy meant to stay, and under what licence line?
