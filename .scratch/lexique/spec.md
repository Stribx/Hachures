# Le Lexique, and reading the board

**Status:** done (approved by the owner 2026-10-28).

## Problem Statement

The player can't read the game.
- A tool card shows a key, an icon, a name and a cost: no damage, range, rate or "strong against".
- Placing a tower shows no range (`ink/ghost.rs` claims one that doesn't exist).
- Placed towers and enemy figures react to nothing on hover; `weak_to` is shown nowhere.
- There is no reference page, and the save doesn't record what the player has met.

## Solution

- **Le Lexique** (key L, also from the title, pause and the tool bar): a page with tabs (Outils, Espèces; built so a later tab is one more row). Each entry unlocks **the first time it appears on the sheet**: a tool when it first shows in the bar, a species when its first figure enters the board. Before that it reads `???`. Old saves are backfilled from the levels already won. Bible §9, row "Lexique".
- **Numbers with words**, after skill bonuses: "Dégâts 30 · Portée 3 cases · un tir toutes les 0,8 s · Fort contre : Chaîneur".
- **Tool card tooltip**, **range circle** (placing a tower; hovering a placed one), **enemy hover card**, and a margin note "Noté au Lexique : X" on each new entry.
- One source for every line shown: pure functions in `campaign/lexique.rs`.

## Decisions

- Entry ids are the i18n label keys: `tool.arbalete`, `species.chaineur` (`TOOLS[t].label`, `Species.label`).
- `Save.seen: BTreeSet<String>`; `VERSION` 2 → 3; `upgrade` backfills from won levels.
- No rule changes: the balance table must not move.
- Out of scope: gamepad, scrolling, more tabs (Lieux, Pouvoirs), new art.

## Tickets

01 save-seen · 02 lexique-lines · 03 unlock-on-sight · 04 lexique-page · 05 tool-tooltip · 06 range-circle · 07 enemy-card · 08 words (done with 00) · 09 review-and-verify
