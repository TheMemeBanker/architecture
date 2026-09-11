# registry-spec

the contract for `data/registry.json`, versioned with the data it governs.

## top level

| field | type | meaning |
|---|---|---|
| `title` | string | always `arc_registry` |
| `total_handshakes` | integer | must equal the count of entries with `type: handshake` — enforced by all three validators |
| `types` | object | human definitions of the three relationship types |
| `tags` | string[] | the closed tag taxonomy; entries may only use tags listed here |
| `entries` | entry[] | the registry body |

## entry

| field | required | type | notes |
|---|---|---|---|
| `id` | yes | string | kebab-case (`[a-z0-9-]+`), unique across the registry; doubles as the doc filename |
| `name` | yes | string | display name, may differ from id (`fabelis AI` vs `fabelis-ai`) |
| `type` | yes | enum | `project` \| `handshake` \| `ecosystem_partner` |
| `token` | no | enum | `token_live` \| `no_token`; **omitted for partners** — partners are relationships, not launches |
| `tags` | yes | string[] | non-empty, subset of the taxonomy |
| `link` | no | string | http(s) url |
| `summary` | yes | string | one paragraph, lowercase house style |

## semantics worth knowing

- **type vs tags:** `handshake` appears both as a type and as a tag. the TYPE records the vetting relationship; the TAG mirrors the card labels on the site. ryzome is type `handshake` (it counts toward `total_handshakes`) while its card carries only `ai_layer` — this is intentional and mirrors the source.
- **`microcontrollers`** exists in the taxonomy with zero entries today; it is reserved by the site's filter bar.
- **partner kinds** (`grant` vs `tech`) are expressed as tags in the data and rendered as badges in `index.html`.

## consumers

| surface | reads | enforced by |
|---|---|---|
| `index.html` | inline mirror of the data | eyeballs + PR review |
| `src/` (rust cli) | `data/registry.json` | `cargo test` (7 tests) |
| `sdk/js/registry.mjs` | same | `node --test` (6 tests) |
| `tools/validate_registry.py` | same + `registry.schema.json` | CI gate |
