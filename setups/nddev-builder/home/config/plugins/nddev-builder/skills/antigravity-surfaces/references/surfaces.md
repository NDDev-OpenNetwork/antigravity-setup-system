# What This Harness Owns

Generated from `references/antigravity-baseline.json`. Do not edit:
the next render overwrites it, and the baseline is where a correction
belongs.

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
| `config/rules` | instruction | directory | measured from the 1.1.22 artifact's own embedded reference, digest verified before reading, 2026-08-29 | read its bytes |

**A citation is not a measurement.** `decided by` says where a row came from; `exercised by` says whether anybody made the product demonstrate it. Where a row records no method the answer is a page and nothing else, because absence of a record of measurement is not evidence of measurement.

Here that is **0 run**, **6 read from the product's own bytes**, and **4 resting on a page alone**. The last number is the one worth acting on: a row in it is not wrong, it is untested, and the two are indistinguishable from here.

A surface that routes no kind is owned deliberately: a backup captures
it and a restore returns it, and no component is routed there because
the kind it would carry already routes somewhere else. One kind on two
surfaces makes a consumer's route ambiguous, and the guard in
`harness_runtime::surfaces` refuses it by name.

## A second target: `target_scope: project`

Rooted at `.agents`, which is **not** this product's configuration
home. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root rather than to the home
above -- writing the root into the path again would nest it twice.

| path | routes | shape | decided by | exercised by |
| --- | --- | --- | --- | --- |
| `.agents/skills` | skill | directory | <https://antigravity.google/docs/skills> | read its bytes |
| `.agents/agents` | agent | directory | <https://antigravity.google/blog/introducing-custom-agents> | read its bytes |
| `.agents/plugins` | plugin | directory | <https://antigravity.google/docs/plugins> | read its bytes |
| `.agents/hooks.json` | hook | file | <https://antigravity.google/docs/hooks> | read its bytes |
| `.agents/mcp_config.json` | mcp | file | <https://antigravity.google/docs/mcp> | *nothing — a page* |
Considered under this scope and not owned:

- **`.agents/commands`** — ai_stp#424 asks for a project-scoped command route. No Antigravity page names a commands directory at any scope: the slash commands the CLI documents are its own built-ins (/skills, /mcp, /agents), not user-authored files. Declared the day a vendor page names the path and not before -- a declared kind is a promise of a rollback.
- **`.agents/AGENTS.md`** — ai_stp#425 asks for a project-scoped instruction route. No page names an instruction file under .agents/, and guessing one would have this provider claim to own a path the product never reads.
- **`.agent/skills`** — The legacy spelling the product still reads for backward compatibility. Owning both would let one workspace hold two skill trees with the product reading one and this provider reporting the other; owning the documented default keeps the answer single.


**Under a scope the namespace is the permission and the recorded
files are the inventory.** A root like this one is read by several
products at once, so `remove`, the capture and a restore all act on
the files this provider recorded writing -- never on the namespace
whole, which would take or revert a neighbour's work.

## Considered and not owned

14 rows. Each records what was searched, so the next reader does not repeat the search:

- **`.antigravity-setup-system`** — This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is.
- **`NDDEV-ANTIGRAVITY-PROVIDER.json`** — This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one.
- **`antigravity-cli/builtin`** — The product's own bundled skills and resources, including `builtin/skills` and a `.checksum`. Shipped with the program, replaced by an update, and never a person's to configure -- so never this provider's to own or restore.
- **`antigravity-cli/cache`** — The CLI's own cache, holding cache/projects.json. Runtime state with a lifetime the product owns -- the string appears as a path literal in the 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood. Never captured into a backup slot: a backup of a cache is a stale answer with a signature on it.
- **`antigravity-cli/hooks.json`** — A second hooks file, beside the owned `config/hooks.json`. Both appear as path literals in the 1.1.22 binary.
- **`antigravity-cli/runtime-state`** — One row for a whole subtree, because listing eleven siblings would suggest each was weighed separately. A single run created `bin/`, `brain/`, `cache/`, `cli.log`, `conversations/`, `conversation_summaries.db` with its `-shm` and `-wal`, `crashes/`, `installation_id`, `jetski_state.pbtxt`, `knowledge/`, `last_check.timestamp`, `log/` and `updater/`. All are the product's own lifetime. None is configuration and none is ownable. The same run also wrote **outside the configuration home entirely** -- `~/.cache/ms-playwright-go`, under the user's cache directory. That has no row of its own because every recorded path here is relative to the target, and this one is relative to a root this provider never evaluates against; the guard that enforces it refused the row, correctly. It is recorded in this sentence instead, so a reader looking for everything the product writes does not stop at `~/.gemini`.
- **`antigravity/transcript.jsonl`** — The conversation transcript, beside antigravity/artifacts. A person's session content, and the string appears as a path literal in the 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood. Never owned and never backed up, for the reason the never_touch list gives about a neighbour's credentials: copying it into a slot would put private text somewhere with a retention policy nobody chose.
- **`config/.migrated`** — An empty marker the product writes beside its config. It records that a layout migration ran, which is the product's bookkeeping about its own history and never a projection surface.
- **`config/config.json`** — The product's own user settings, created on first run: it held `{"userSettings": {"remoteControlHostname": "<this machine's hostname>"}}`. Distinct from the owned `antigravity-cli/settings.json`, which is where the documented posture keys live. Not owned: no vendor page names it, it carries machine identity rather than configuration a setup would choose, and owning it would put a hostname into every backup slot.
- **`config/import_manifest.json`** — Written by the product's own plugin installer. Measured against the 1.1.22 linux/x86_64 bytes (sha256:1e1a219a...), verified before running and executed in a contained HOME so nothing of the owner's was touched: `antigravity plugin install` created this file beside config/plugins. It is the installer's bookkeeping about what it imported, not a projection surface, and owning it would let this provider's `remove` erase the product's record of plugins a person installed by hand.
- **`config/projects`** — Per-project state the product keeps under its own home. Not configuration this provider projects, and the string appears as a path literal in the 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood.
- **`config/workflows`** — A second **global** workflow root, beside the owned `config/global_workflows`. Both are named as sources in the product's own embedded migration reference, which converts workflows into skills and tabulates them under scope `Global`: `~/.gemini/config/global_workflows/<name>.md` and `~/.gemini/config/workflows/<name>.md`, both targeting `~/.gemini/config/skills/<name>/SKILL.md`. The workspace tier is a different set of rows -- `<workspace>/.agents/workflows/<name>.md` and its siblings.
- **`config/workflows.json`** — The manifest beside that second global workflow root, beside the owned `config/global_workflows`. Both are named as sources in the product's own embedded migration reference, which converts workflows into skills and tabulates them under scope `Global`: `~/.gemini/config/global_workflows/<name>.md` and `~/.gemini/config/workflows/<name>.md`, both targeting `~/.gemini/config/skills/<name>/SKILL.md`. The workspace tier is a different set of rows -- `<workspace>/.agents/workflows/<name>.md` and its siblings.
- **`managed-config`** — Not a path in the target, and recorded because **there is no such path** -- the same measured absence as pi's, made against a very different artifact.
