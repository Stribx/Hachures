# Dev tools

The screenshot harness and the balance bot.

- **Screenshot harness** (`shot/`): F12 saves to `screenshots/`. `HACHURES_SHOT=out.png` jumps into play, saves a shot after `HACHURES_AT` seconds (a comma list takes several) and quits. Also: `HACHURES_SCREEN=title|plan|paused|settings|lexique|controls|credits`, `HACHURES_LANG=en`, `HACHURES_LEVEL=n` (index in `LEVELS`), `HACHURES_DEMO=1` (one tower of each kind and a few barricades), `HACHURES_PARADE=1` (one still figure of each species in a row), `HACHURES_FASCINES=1`, `HACHURES_WAVE=1` (`n` sends wave n at once), `HACHURES_CALQUE=1` (le Calque on), `HACHURES_CAM=x,y,z,tx,ty,tz`, `HACHURES_WALK=n@t`, `HACHURES_TRAVEL=n@t`, `HACHURES_INK=t`, `HACHURES_HOVER=tower|enemy|<cell>` and `HACHURES_TOOL=n` (point without a mouse: range circle, enemy card), `HACHURES_CARD=n` (hover tool card `n`: its tooltip), and `HACHURES_FPS=1` (frame and per-pass GPU times). The harness changes settings in memory only; point `XDG_DATA_HOME` at a scratch folder to keep the real save untouched. Running the binary directly needs `BEVY_ASSET_ROOT=game`.
- **Balance bot**: `cargo test -p game balance -- --ignored --nocapture` plays every level with a simple tower strategy and prints a table (won, Rôle, Names, towers, sépia, losses per wave). It is deterministic and checks the table against `game/tests/balance.txt` (CI runs it too): a rule or data change that moves it fails. Read the diff, then rerun with `HACHURES_BLESS=1` to keep the new table.

## Builds and formatting

- Desktop, release and web builds, and serving the site: `README.md`. The workspace's release and
  `wasm-release` profiles stay as they are unless the build requirements change.
- Now and then, wrap long comments and merge imports per module, which stable rustfmt can't:
  `cargo +nightly fmt -- --config wrap_comments=true,comment_width=100,imports_granularity=Module`.
