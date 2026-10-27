# 01: Gratter asks the grid what can be scraped

**What to build:** A refactor done first, with no behaviour change. The "is there something to scrape here" rule moves into one query on the grid, which both the "nothing to scrape" refusal and the scrape itself use. The scrape also goes through one grid method, so later tickets only have to widen that query and that method. Today only an imperial borne answers yes. See the spec, `.scratch/gratter-returns-to-sol/spec.md`, "What can be scraped" and "What a scrape does".

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [x] One grid query says whether a cell can be scraped. Today it answers yes for a borne only.
- [x] The Gratter refusal and the scrape act both go through it. There is no second copy of the rule.
- [x] The existing Gratter tests (scrapes only a borne; preview takes the borne off) pass unchanged.
- [x] `cargo test -p game`, clippy `-D warnings` and the structure test are green.
- [x] The balance bot table is identical before and after.
