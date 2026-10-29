@AGENTS.md

## Claude Code only

**Work inline by default.** A subagent starts at about 10k tokens before it works, and re-reads what you
already know. Use one only when its output would flood this context or the job is independent:

| Job | Agent |
|---|---|
| A noisy command whose log you don't need (cargo, git) | `runner` |
| A sweep over many files when only the conclusion matters | `Explore` (skips CLAUDE.md) |
| Code from a spec you decided, or a build to make green when the errors say why | `implementer` |
| Text in every language | `translator` |
| A first-pass review of a large diff | `reviewer` |

Never for one Read, one Grep or a short edit. Fork only when the whole chat is needed. Give the agent a
self-contained prompt (paths, values, the acceptance check), start independent ones in one message, and
read the diff it leaves. `.claude/agents/` also has `finder` (where, what a file says, how a flow
runs), `mechanic`, `test-writer` and `web-lookup` for the same rule.
