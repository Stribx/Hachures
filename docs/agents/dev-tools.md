# Dev tools

The screenshot harness and the balance bot.

- **Screenshot harness** (`shot/`): F12 saves to `screenshots/`. `HACHURES_SHOT=out.png` jumps into play, saves a shot after `HACHURES_AT` seconds (a comma list takes several) and quits. Also: `HACHURES_SCREEN=title|plan|paused|settings|lexique|sceaux|controls|credits`, `HACHURES_LANG=en`, `HACHURES_LEVEL=n` (index in `LEVELS`), `HACHURES_DEMO=1` (one tower of each kind and a few barricades), `HACHURES_PARADE=1` (one still figure of each species in a row), `HACHURES_FASCINES=1`, `HACHURES_WAVE=1` (`n` sends wave n at once), `HACHURES_CALQUE=1` (le Calque on), `HACHURES_CAM=x,y,z,tx,ty,tz`, `HACHURES_WALK=n@t`, `HACHURES_TRAVEL=n@t`, `HACHURES_INK=t`, `HACHURES_HOVER=tower|enemy|<cell>` and `HACHURES_TOOL=n` (point without a mouse: range circle, enemy card), `HACHURES_CARD=n` (hover tool card `n`: its tooltip), and `HACHURES_FPS=1` (frame and per-pass GPU times, logged). The player's own view of the same numbers is the performance overlay (`ui/perf.rs`, F3 or the Display settings): FPS, frame time, the sum of the top-level passes' GPU times, the GPU's name. The harness changes settings in memory only; point `XDG_DATA_HOME` at a scratch folder to keep the real save untouched. Running the binary directly needs `BEVY_ASSET_ROOT=game`.
- **CPU profiler (Tracy)**: `cargo run --release -p game --features profile` sends a span per system, render node and frame to the Tracy GUI, which must be the version Bevy's `tracy-client-sys` bundles (0.14.1 for Bevy 0.20: AUR `tracy`). Open the GUI (`tracy-profiler`) and click Connect; `tracy-capture -o out.tracy -s 10` records without it, and `tracy-csvexport out.tracy` lists each zone's count and time. The harness's window has no focus, so Bevy caps it at 60 Hz: judge harness runs by the GPU pass times, not the frame rate.
- **Balance bot**: `cargo test -p game balance -- --ignored --nocapture` plays every level with a simple tower strategy and prints a table (won, Rôle, Names, towers, sépia, losses per wave). It is deterministic and checks the table against `game/tests/balance.txt` (CI runs it too): a rule or data change that moves it fails. Read the diff, then rerun with `HACHURES_BLESS=1` to keep the new table.

## Builds and formatting

- Desktop, release and web builds, and serving the site: `README.md`. The workspace's release and
  `wasm-release` profiles stay as they are unless the build requirements change.
- Now and then, wrap long comments and merge imports per module, which stable rustfmt can't:
  `cargo +nightly fmt -- --config wrap_comments=true,comment_width=100,imports_granularity=Module`.
