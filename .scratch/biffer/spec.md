# Biffer: taking back Nève's ink (le Repentir)


Status: ready-for-agent
## Context

Today the player can ink towers, barricades, mud and brambles but can never take them back. The code has no removal path, and the design forbids one: `docs/bible.md:421` and the Gratter glossary entry say "never Nève's own ink", and nothing records who placed what. One misclick costs a whole tower for the rest of the level, and the board can't be rearranged between waves.

The owner chose:
- **Rule, le Repentir.** Wet ink comes back at 100% of what it cost. Dry ink comes back at 50%, rounded down.
- **Scope.** Everything Nève inks: the 4 towers, barricade, boue, ronces.
- **Input.** A new tool in the bar, **Biffer**, key 9. It is free to use. Hovering your own ink shows what it would return.

**Calls I made** (the owner can change them):
- **When ink is wet.** Ink is wet only if it was laid *between waves* and no wave has been sent since. Anything laid while a wave runs is dry at once. This closes a loophole: with "wet since the last wave began", a player could pick a tower up for free mid-wave and put it down ahead of the column.
- **Names.** A scratched-out Name stays scratched out, even under wet ink (bible §2 rule 7). The existing hover warning already tells the player before the click.
- **Challenges.** `Level.used` keeps the tool even after it is biffed, so a "finir sans X" challenge is still failed.
- **Only `Level::ink` placements count.** Map-drawn `#`/`b`/`r` tiles, Ronces-mères growth, the Chaussée, Palissades and bornes can't be biffed. Palissades and bornes stay Gratter's.
- **The Biffer icon** reuses Gratter's scraper piece for now, marked with a `ponytail:` comment.

## Tasks (in order; TDD: each code step starts with failing tests)

### 0. Design first (bible wins)
- `docs/bible.md`, §2 rule 7: Nève may biffer her own ink; the name it cost never comes back.
- `docs/bible.md`, §7: a note under Tracer, or a row, for Biffer (le Repentir, wet 100% / dry 50%, free, earned at the Prologue).
- `docs/bible.md`: a line in the §8 economy and a row in the §9 mechanics table ("[T], recalcul" like a pose).
- `CLAUDE.md` glossary: a Biffer entry. Gratter keeps "never Nève's own ink" and gains "(that is Biffer's job)".
- `.scratch/biffer/` issue file per `docs/agents/issue-tracker.md`, `Status:` line.

### 1. Terrain can undo a placement (`map/terrain.rs`)
- **RED** (terrain tests): `Edit::Unink(tile)` on a tower cell sets `base.tiles[i] = tile`, drops it from `towers`, and lowers `cover` (Ombre). The same on a barricade restores the tile and reopens the flow field. Extend `every_preview_is_what_the_ink_does` to cover `Unink`.
- **GREEN:** add `Edit::Unink(Tile)`. In `apply()`: `tiles[i] = t; towers.retain(|w| w.0 != i)`. `settle()` already rebuilds cover and fields.

### 2. The record and the rule (`level/mod.rs` data, new `ink/biffer.rs`)
- **Data, in `level/mod.rs`:** `Level.inked: Vec<Inked { cell, tool, under: Tile, paid: u32, laid_at: Option<u32> }>`. `laid_at` is `Some(wave.number)` when laid with no wave running, `None` otherwise. Wet = `laid_at == Some(wave.number) && !wave.running`. Nothing is mutated at wave send.
- `Level::ink` (`ink/mod.rs:153`): record `under` before `alter`, and `paid = tool_cost(tool)` (Fascines/Mine de plomb prices are what comes back).
- **RED** (`ink/biffer.rs` tests, built with the `passage.rs:94` `level(map)` helper style). Each is one test:
  - wet barricade → +10, the cell is Sol again, the record is gone;
  - a barricade paid with Fascines (7) → +7;
  - dry (a wave has been sent) → half, rounded down: arbalète 50 → 25, barricade 7 → 3;
  - laid while a wave runs → dry at once;
  - refused on Sol, on a map-drawn `#`, on Ronces-mères growth, on a Palissade, on a borne → `refusal.nothing_to_biffer`;
  - biffing twice → the second is refused;
  - a stale record (the cell's tile changed under it) → refused;
  - a named place stays in `erased`, and `names` doesn't come back;
  - never refused by the Droit de Passage (skip `closes()` as Scrape does, `passage.rs:14`);
  - a barricade on Chaussée comes back as Chaussée.
- **GREEN:**
  - `Act::Biffer` with `lays()` → `None` and no `weapon()`. The `TOOLS` row is appended at the **end** (index 8, `Digit9`, cost 0, name "biffer", label `tool.biffer`), so the balance-bot indices stay put.
  - `Level::refund(i) -> Option<u32>` and `Level::biffer(i)`: refund the sépia, `alter([(i, Edit::Unink(under))])`, drop the record.
  - Branch in `refusal` (`passage.rs:13`).
  - The exhaustive `Act` matches: `From<Act> for Edit`, `ui/studio.rs:140`, `campaign/lexique.rs:88`.
- Every level gets Biffer: always add it to `level.tools` in `level/load.rs`. No `.ron` level edits.

### 3. The tower leaves the board (`combat/`)
- **RED** (`combat/tests.rs`, `app_with`): after a `TowerBiffed { cell }` message, the cell has no `Tower` entity and no tower base entity.
- **GREEN:**
  - `inking` (`ink/mod.rs:114`) sends `TowerBiffed { cell }` when the biffed tool was a tower.
  - The tower base gets a marker with its cell (it has none today, `towers.rs:89`).
  - A small system despawns both entities. It lives in a new file, since `towers.rs` is at 177/200 lines, and is ordered after `inking`, like `raise_towers`.
  - Check that nothing in flight (shots, a harpoon pin) still points at the despawned tower.

### 4. What the player reads
- `ink/verdict.rs` / `ui/places.rs:86` `tooltip`:
  - with Biffer over your own ink, show `ink.refund` ("Rend {} sépia" / "Returns {} sépia");
  - fix the existing bug: the "would scratch out «X»" warning should only show when `act.lays().is_some()`. It is wrong for Gratter today.
- `combat/range.rs` already circles a hovered tower. Nothing to do.
- Bar card: Biffer costs 0, so it never fades as "can't afford". Check `ui/bar.rs:170` `look()`.
- `menu/controls.rs`: if `controls.key.digits` says "1–8", make it "1–9" in fr and en.
- i18n keys in `fr.ron` and `en.ron` (`translator` agent):
  - `tool.biffer`: "biffer" / "strike out";
  - `tool.biffer.text`: "Reprend un trait de Nève. L'encre fraîche rend toute sa sépia, l'encre sèche la moitié. Un nom rayé ne revient pas.";
  - `refusal.nothing_to_biffer`: "Rien de Nève ici";
  - `ink.refund`.
- Lexique: `tool_lines` for Biffer shows the two rates. `every_tool_and_species_has_one_entry` covers the entry automatically.

### 5. Verify (no `build-web.sh`: the owner runs it)
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -p game --quiet` (structure, i18n, lexique, levels tests included).
- Balance bot: `cargo test -p game balance -- --ignored --nocapture`. It must not move, because the bot never biffs. If it moves, that is a bug: don't bless it.
- Screenshot harness (scratch `XDG_DATA_HOME`, see memory "Screenshot harness"):
  - `HACHURES_DEMO=1 HACHURES_TOOL=8 HACHURES_HOVER=tower` shows the refund tooltip and the range circle;
  - a second shot shows the bar with the Biffer card at 9.
- `reviewer` agent on the diff, then Opus reads it. Commit only when asked.

## Delegation
- Step 0: inline (Opus). These are design words.
- Steps 1–2: `implementer` (Sonnet), given this spec and the RED tests (Opus writes the tests first).
- Step 3: `implementer`.
- Step 4 i18n: `translator`. The rest: `implementer`.
- Step 5: `runner` (Haiku), then the `reviewer` and an Opus read.

## Critical files
`docs/bible.md`, `CLAUDE.md`, `game/src/map/terrain.rs`, `game/src/ink/{mod,passage,verdict}.rs`, new `game/src/ink/biffer.rs`, `game/src/level/{mod,load}.rs`, `game/src/combat/{towers,mod,tests}.rs` and a new despawn file, `game/src/ui/{places,studio,bar}.rs`, `game/src/campaign/lexique.rs`, `game/src/menu/controls.rs`, `game/assets/data/i18n/{fr,en}.ron`.
