# contributing to arc_registry

the registry is data-first: **`data/registry.json` is the single source of truth.** the site, the CLI, the SDK, and the docs all read from it. never hand-edit a rendered surface without updating the data.

## adding an entry

1. add the entry to `data/registry.json`:
   - `id` — kebab-case, unique
   - `type` — `project`, `handshake`, or `ecosystem_partner`
   - `token` — `token_live` or `no_token` (omit for partners)
   - `tags` — only tags already in the top-level taxonomy; propose new tags in the same PR
   - `summary` — one tight paragraph, lowercase house style
2. add a doc under `projects/` or `partners/` named `<id>.md`
3. add a row to the table in `README.md`
4. run the full check locally:

```sh
python3 tools/validate_registry.py   # schema + structural rules
node --test sdk/js/registry.test.mjs # sdk contract
cargo test                           # typed model + invariants
```

CI runs the same three gates on every push — a PR that fails any of them doesn't merge.

## invariants the tooling enforces

- entry ids unique and kebab-case
- every tag on an entry exists in the taxonomy
- `total_handshakes` equals the number of `handshake`-typed entries
- partners never declare a token status
- links are http(s) urls

## house style

lowercase prose, tight summaries, no hype adjectives. the registry describes; it does not sell.
