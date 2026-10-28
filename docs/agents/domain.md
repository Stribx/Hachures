# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Before exploring, read these

- **`docs/agents/glossary.md`**: the game's terms. Names in the fiction stay French in code, data and files; engineering words are English.
- **`docs/bible.md`** (French): the game design, its rules and numbers. When the code and the bible disagree, the bible wins: the code has a bug.
- **`docs/adr/`**: read ADRs that touch the area you're about to work in.

If any of these don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront. The `/domain-modeling` skill (reached via `/grill-with-docs` and `/improve-codebase-architecture`) adds resolved terms to the table in `docs/agents/glossary.md` and creates `docs/adr/` lazily when a decision gets made.

## File structure

Single-context repo:

```
/
├── docs/agents/glossary.md
├── docs/
│   ├── bible.md       ← the design source of truth
│   └── adr/
│       └── 0001-<decision>.md
└── game/src/
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in the Glossary. Don't drift to synonyms (a *sheet* is a level's board, *sépia* is the ink, *the Cible* is what a level defends).

If the concept you need isn't in the glossary yet, that's a signal: either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/domain-modeling`).

## Flag ADR and bible conflicts

If your output contradicts an existing ADR or the bible, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 (…), but worth reopening because…_

A game rule changes in the bible first, and only with the owner's say.
