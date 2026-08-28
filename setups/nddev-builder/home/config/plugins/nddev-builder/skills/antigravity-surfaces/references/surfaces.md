# What This Harness Owns

Generated from `references/antigravity-baseline.json` by
`tools/build_nddev_builder.py`. Do not edit: the next render overwrites
it, and the baseline is where a correction belongs.

Every row below was decided by a source, and the source is named. Where
this file and the binary disagree, the binary is right -- ask it with
`antigravity-setup-system provider-info`.

**Configuration home**: `~/.gemini`

## The configuration file

`antigravity-cli/settings.json` is **json**, and the parser does not accept comments.

Plain JSON per the vendor's settings page. No schema published, searched 2026-08-28.

## Owned surfaces

| path | kinds | shape | decided by | exercised by |
|---|---|---|---|---|
| `antigravity-cli/settings.json` | setting | file | <https://antigravity.google/docs/settings> | read its bytes |
| `antigravity-cli/keybindings.json` | *(routes no kind)* | file | <https://antigravity.google/docs/settings> | *nothing — a page* |
| `antigravity-cli/plugins` | *(routes no kind)* | directory | <https://antigravity.google/docs/cli/plugins/> | *nothing — a page* |
| `config/plugins` | plugin | directory | <https://antigravity.google/docs/plugins> | *nothing — a page* |
| `config/skills` | skill | directory | <https://antigravity.google/docs/skills> | read its bytes |
| `config/agents` | agent | directory | <https://antigravity.google/docs/subagents/> | *nothing — a page* |
| `config/hooks.json` | hook | file | <https://antigravity.google/docs/hooks> | read its bytes |
| `config/mcp_config.json` | mcp | file | <https://antigravity.google/docs/mcp> | read its bytes |
| `config/global_workflows` | command | directory | <https://antigravity.google/docs/rules-workflows/> | read its bytes |

**A citation is not a measurement.** `decided by` says where a row came from; `exercised by` says whether anybody made the product demonstrate it. Where a row records no method the answer is a page and nothing else, because absence of a record of measurement is not evidence of measurement.

Here that is **0 run**, **5 read from the product's own bytes**, and **4 resting on a page alone**. The last number is the one worth acting on: a row in it is not wrong, it is untested, and the two are indistinguishable from here.

A surface that routes no kind is owned deliberately: a backup captures
it and a restore returns it, and no component is routed there because
the kind it would carry already routes somewhere else. One kind on two
surfaces makes a consumer's route ambiguous, and the guard in
`harness_runtime::surfaces` refuses it by name.

## Considered and not owned

13 rows. Each records what was searched, so the next reader does not repeat the search:

- **`NDDEV-ANTIGRAVITY-PROVIDER.json`** — This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one.
- **`.antigravity-setup-system`** — This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is.
- **`config/import_manifest.json`** — Written by the product's own plugin installer. Measured against the pinned 1.1.22 linux/x86_64 bytes (sha256:1e1a219a...), verified before running and executed in a contained HOME so nothing of the owner's was touched: `antigravity plugin install` created this file beside config/plugins. It is the installer's bookkeeping about what it imported, not a projection surface, and owning it would let this provider's `remove` erase the product's record of plugins a person installed by hand.
- **`antigravity-cli/hooks.json`** — A second hooks file, beside the owned config/hooks.json. Both appear as path literals in the pinned binary and the string appears as a path literal in the pinned 1.1.22 binary. Which one the product prefers when both exist is not documented and has not been measured, so nothing here owns it: a hook file this provider wrote and the product ignored would be a silently inert setup, and one it removed would be somebody else's configuration.
- **`antigravity-cli/cache`** — The CLI's own cache, holding cache/projects.json. Runtime state with a lifetime the product owns -- the string appears as a path literal in the pinned 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood. Never captured into a backup slot: a backup of a cache is a stale answer with a signature on it.
- **`config/projects`** — Per-project state the product keeps under its own home. Not configuration this provider projects, and the string appears as a path literal in the pinned 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood.
- **`antigravity/transcript.jsonl`** — The conversation transcript, beside antigravity/artifacts. A person's session content, and the string appears as a path literal in the pinned 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood. Never owned and never backed up, for the reason the never_touch list gives about a neighbour's credentials: copying it into a slot would put private text somewhere with a retention policy nobody chose.
- **`config/config.json`** — The product's own user settings, created on first run: it held `{"userSettings": {"remoteControlHostname": "<this machine's hostname>"}}`. Distinct from the owned `antigravity-cli/settings.json`, which is where the documented posture keys live. Not owned: no vendor page names it, it carries machine identity rather than configuration a setup would choose, and owning it would put a hostname into every backup slot.
- **`config/.migrated`** — An empty marker the product writes beside its config. It records that a layout migration ran, which is the product's bookkeeping about its own history and never a projection surface.
- **`antigravity-cli/builtin`** — The product's own bundled skills and resources, including `builtin/skills` and a `.checksum`. Shipped with the program, replaced by an update, and never a person's to configure -- so never this provider's to own or restore.
- **`antigravity-cli/runtime-state`** — One row for a whole subtree, because listing eleven siblings would suggest each was weighed separately. A single run created `bin/`, `brain/`, `cache/`, `cli.log`, `conversations/`, `conversation_summaries.db` with its `-shm` and `-wal`, `crashes/`, `installation_id`, `jetski_state.pbtxt`, `knowledge/`, `last_check.timestamp`, `log/` and `updater/`. All are the product's own lifetime. None is configuration and none is ownable. The same run also wrote **outside the configuration home entirely** -- `~/.cache/ms-playwright-go`, under the user's cache directory. That has no row of its own because every recorded path here is relative to the target, and this one is relative to a root this provider never evaluates against; the guard that enforces it refused the row, correctly. It is recorded in this sentence instead, so a reader looking for everything the product writes does not stop at `~/.gemini`.
- **`config/workflows`** — The workspace tier of the workflow surface, named by the product's own embedded reference beside the global one this provider owns. Not owned: this provider configures a home, not a checkout. It was explained inside the `config/global_workflows` note until 2026-08-28, which is not where a reader looks before opening a file to find out what it is -- the declined block is.
- **`config/workflows.json`** — The workspace tier of the workflow surface, named by the product's own embedded reference beside the global one this provider owns. Not owned: this provider configures a home, not a checkout. It was explained inside the `config/global_workflows` note until 2026-08-28, which is not where a reader looks before opening a file to find out what it is -- the declined block is.
