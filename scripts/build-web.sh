#!/usr/bin/env bash
# Builds the browser version of Hachures into website/play/.
# Needs, once:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --version <the wasm-bindgen version in Cargo.lock>
# Then serve website/ over HTTP (e.g. `python3 -m http.server -d website`) and open /play/.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --profile wasm-release --target wasm32-unknown-unknown -p game
wasm-bindgen --target web --no-typescript --out-dir website/play --out-name hachures \
    target/wasm32-unknown-unknown/wasm-release/Hachures.wasm

# Only what the game loads at runtime: game data is compiled in.
out=website/play/assets
rm -rf "$out"
mkdir -p "$out"
# game/assets/ keeps only what the game loads (the rest of the packs sits in Hachures-asset-library/), so each
# folder goes whole: paths built at runtime (`format!("models/{model}.glb")`) are covered too.
cp -r game/assets/{fonts,models,audio,shaders,ui} game/assets/CREDITS.md "$out/"

du -sh website/play
