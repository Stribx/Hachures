# Rust style

How this repo applies `docs/rustguide.md`.

Follow `docs/rustguide.md` (Google's Rust style guide) as **application code**: its API rules are strong advice, and simplicity wins over generality. Its Chromium/Android/Fuchsia-only rules (flat layout, small crates, gtest) don't apply. What bites here, and our exceptions:

- **Imports.** No glob imports but `use bevy::prelude::*` and `use super::*` in tests (`clippy::wildcard_imports`). One module per `use`: `use a::b::{C, D};`, never `use a::{b::C, d::E};`. Types are imported and used bare.
- **Free functions** from another module are called through it: `shards::shatter(..)`, `map::cell_pos(..)`. Exception: the toolkit's vocabulary (`t`, `tf`, `tp`, `n_`, `text`, `font`, the `ui::widgets` builders) stays bare. A name that means two things (`look`, `ink`, `ground`) is always qualified.
- **Panics.** No `.unwrap()` outside tests (`clippy::unwrap_used`): `expect("why it can't fail")` for a real invariant, `let … else` / `if let` where a check would come first. Document `# Panics` when a caller's precondition can panic (bad level data, a skipped `refusal`).
- **Types.** An enum, not a `bool`, for a parameter that reads `f(x, true)` at the call site. Named fields, not a tuple or a tuple struct, once the value crosses a module (`Query<(..)>` and bundles are Bevy's and exempt). Derive what can be derived.
- **Iterators.** A `for` loop, not `for_each` (`clippy::needless_for_each`). Bind a chain to a named `let` before the `for`, never in its header. Short combinators (`any`, `find`, `sum`) are fine.
- **Asserts** carry a message when the failure would not explain itself: `assert!(xs.iter().all(..), "{xs:?}")`.
- **Names.** No abbreviations an outsider wouldn't know; single letters only in short closures and math. Don't shadow a function you also call (`t`, `look`). A method named like a std trait (`next`, `from_str`) is that trait or renamed.
- **`#[must_use]`** on pure functions whose result is easy to drop by mistake (a builder returning `Self`, `take_redraw`, `refusal`).
- **`mod.rs`** holds the plugin, re-exports and small shared types; the implementation lives in leaf files.
- **Per-frame systems** don't deep-clone and don't write unchanged values: `set_if_neq`, `run_if(resource_changed::<R>)`.
- **No `unsafe`** (`#![deny(unsafe_code)]`), no `Rc`/`Arc` without a reason, no macros where a function does.
- **Format:** `rustfmt.toml` is the guide's `max_width = 100`. Stable rustfmt doesn't wrap comments: keep new ones within 100 columns. `imports_granularity` and `hex_literal_case` are nightly-only, so review checks those.

The lints live in `game/src/main.rs`; CI's `clippy -D warnings` fails on them.
