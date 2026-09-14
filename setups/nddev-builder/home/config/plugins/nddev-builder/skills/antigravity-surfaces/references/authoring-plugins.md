# Writing a plugin for this harness

Generated from the vendor's own reference and the pinned binary. Do not edit: the next render overwrites it, and a correction belongs in the source this file is derived from.

**Where it goes**: `~/.gemini/config/plugins/<name>/plugin.json`

**Decided by**: https://antigravity.google/docs/plugins; the manifest filename confirmed in the 1.1.22 artifact, which names `plugin.json` and never `plugin-index.json`

**How it runs**: Loaded from the plugins directory; the manifest is what makes the directory a plugin.

## Manifest

| key | required | what it does |
|---|---|---|
| `name` | **yes** | Required identifier matching `^[a-zA-Z0-9-_]+$`. |
| `description` | no | Human-readable purpose shown in plugin listings. |

## What bites

- **A plugin here is a bundle and runs no code of its own**, like the manifest-shaped one elsewhere in this estate and unlike the two that are executable modules. Beside `plugin.json` it may carry `mcp_config.json`, `hooks.json`, `skills/`, `agents/` and `rules/` -- the same files, in the same shapes, as the loose ones.
- The current page's schema requires `name`, allows optional `description`, and says additional properties are rejected. The 1.2.2 product is looser: `agy plugin validate` rejects a missing `name` but accepts an invented extra key, and the schema URL shown on the page returned 404 when checked. Treat `name` as the native minimum, use only fields the product demonstrably consumes, and validate with the installed CLI rather than relying on editor schema completion. Without `plugin.json` the directory is not a plugin.
- **The filename was checked against the product, not taken from the page alone**, after the same claim for another harness here turned out to be a third-party invention. The pinned artifact names `plugin.json` and never `plugin-index.json`. Two harnesses in this estate use that filename and one of the two nests it under a dot-directory, so read the row rather than remembering it.

## The same file on the other harnesses

Generated from the same rows as the section above, for every harness in this estate that routes this kind. `—` means the product's own reference does not name the field, and **dropped** means it names it as one it accepts and does not act on.

| field | `claude` | `grok` | `cursor` | `antigravity` |
|---|---|---|---|---|
| `name` | **required** | yes | **required** | **required** |
| `displayName` | yes | — | — | — |
| `description` | yes | yes | yes | yes |
| `version` | yes | — | yes | — |
| `author` | yes | — | yes | — |
| `skills` | yes | yes | yes | — |
| `commands` | yes | yes | yes | — |
| `agents` | yes | yes | yes | — |
| `hooks` | yes | yes | yes | — |
| `mcpServers` | yes | yes | yes | — |
| `outputStyles` | yes | — | — | — |
| `lspServers` | yes | yes | — | — |
| `userConfig` | yes | — | — | — |
| `homepage` | yes | — | yes | — |
| `repository` | yes | — | yes | — |
| `license` | yes | — | yes | — |
| `keywords` | yes | — | yes | — |
| `metadata` | yes | — | — | — |
| `defaultEnabled` | yes | — | — | — |
| `dependencies` | yes | — | — | — |
| `logo` | — | — | yes | — |
| `rules` | — | — | yes | — |
| `variables` | — | — | yes | — |

**The part that travels**: `name`, `description`. Everything else is a bet on one product.

**The part that does not, and says nothing when it does not**: a field absent from a column is not rejected there -- it is read past. Nothing warns, no run fails, and the component behaves differently with the same bytes. Where the field was carrying a restriction, the restriction is simply gone. Check the column before relying on one.

## Before you ship one

- **The surface is declared, so the component is a promise.** Every kind   this provider declares is a promise of a rollback. A component written   to a path the declaration does not carry is installed by nobody and   removed by nobody.
- **Name it once.** Where the product derives identity from the directory   or the filename, the frontmatter `name` is either redundant or a second   place to be wrong. Keep them equal.
- **Read it back.** After an install, look at the file where the product   reads it, not at the step that put it there.
