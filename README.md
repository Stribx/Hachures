# Hachures

*Qui trace la frontière ?* — a cartographic tower defense.

Nève Achard, once the Empire's First Surveyor, has deserted before the Last Cadastre fixes the Marches for good. In
this world a map drawn in earth-ink, and seen by witnesses, becomes true. You don't stop the imperial columns with
walls of towers: you turn them aside by redrawing the relief, because every people reads the land its own way (by the
slope, by the shadow, by its goal). But every stroke has a price, a name scratched off the land. Under one rule: the
map may never be closed (*le Droit de Passage*).

A 3D low-poly tower defense built on path diversion, one Dijkstra flow field per species, written in Rust with
[Bevy](https://bevyengine.org) for the desktop and the browser.

## Play

- In the browser: build the web version (below) and serve `website/`.
- On the desktop: `cargo run --release -p game`.

Mouse only; keys are listed in the game's *Controls* page. The game is in French and English (settings).

## Build

```sh
cargo run -p game --features dev     # desktop, fast incremental builds
cargo build --release -p game        # desktop release binary
cargo test -p game                   # tests, including the level and Droit de Passage checks
./scripts/build-web.sh               # browser build into website/play/
python3 -m http.server -d website    # then open http://localhost:8000/play/
```

The web build needs `rustup target add wasm32-unknown-unknown` and the `wasm-bindgen-cli` version that matches
`Cargo.lock`.

## Project

- `game/src/` — the game, one folder per feature; `game/assets/` — data, models, sounds, fonts, shaders, art.
- `docs/` — the design: `bible.md` (the game design and story, in French), the art direction, the 3D asset rules
  and the prompts for the 2D art.
- `CLAUDE.md` — the full project map, the code rules (checked by CI) and the conventions.

## Licence

The code is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option. The assets keep
their own licences, listed in [`game/assets/CREDITS.md`](game/assets/CREDITS.md): Kenney models and sounds (CC0),
the music (CC0), the fonts (SIL OFL 1.1), and the AI-generated 2D art (CC0).
