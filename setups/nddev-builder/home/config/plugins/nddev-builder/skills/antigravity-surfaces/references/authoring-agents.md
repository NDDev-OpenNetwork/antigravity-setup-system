# Writing an agent for this harness

Generated from the vendor's own reference and the pinned binary. Do not edit: the next render overwrites it, and a correction belongs in the source this file is derived from.

**Where it goes**: `~/.gemini/config/agents/<name>.md`

**Decided by**: https://antigravity.google/docs/subagents/

**How it runs**: A parent calls `invoke_subagent`, which spawns a concurrent session with that role.

## Frontmatter

| field | required | what it does |
|---|---|---|
| `name` | **yes** | Unique identifier. |
| `description` | **yes** | Task delegation guidance. |
| `tools` | no | Permitted tools, e.g. `view_file`, `replace_file_content`. Empty by default: a subagent is given nothing until it is listed here. |
| `mainAgent` | no | Whether it may be picked as the primary. Default true. |
| `subagent` | no | Whether it may be invoked as one. Default true. |
| `model` | no | `inherit`, `flash` or `pro`. Default `inherit`. |
| `commandExecutionPolicy` | no | `off`, `auto`, `eager` or `sandbox`. Default `sandbox`. |
| `mcpServers` | no | MCP configurations for this agent. |
| `skills` | no | Skill dependencies; `plugins` takes the same shape. |

## What bites

- **The keys are camelCase**, like the harness whose subagents use `disallowedTools` and unlike every skill frontmatter in this estate. `commandExecutionPolicy` has no equivalent anywhere else here and defaults to `sandbox`, so an agent copied in from another product runs more contained than it did, not less.
- The file may be `<name>.md` or `<name>/agent.md`. The directory form is what lets an agent carry files beside it.
- This row's citation was dead for a while: the vendor's `docs/agents` page answers 404 and the content moved to `docs/subagents`. Nothing in this repository fetches a URL, so a stale citation is found by reading it and in no other way.

## The same file on the other harnesses

Generated from the same rows as the section above, for every harness in this estate that routes this kind. `—` means the product's own reference does not name the field, and **dropped** means it names it as one it accepts and does not act on.

| field | `claude` | `grok` | `opencode` | `antigravity` |
|---|---|---|---|---|
| `name` | **required** | — | yes | **required** |
| `description` | **required** | — | **required** | **required** |
| `tools` | yes | — | yes | yes |
| `disallowedTools` | yes | — | — | — |
| `model` | yes | — | yes | yes |
| `permissionMode` | yes | yes | — | — |
| `maxTurns` | yes | — | — | — |
| `skills` | yes | — | — | yes |
| `mcpServers` | yes | yes | — | yes |
| `hooks` | yes | yes | — | — |
| `memory` | yes | — | — | — |
| `background` | yes | — | — | — |
| `effort` | yes | — | — | — |
| `isolation` | yes | — | — | — |
| `color` | yes | — | yes | — |
| `initialPrompt` | yes | — | — | — |
| `mcpInheritance` | — | yes | — | — |
| `mode` | — | — | yes | — |
| `temperature` | — | — | yes | — |
| `top_p` | — | — | yes | — |
| `permission` | — | — | yes | — |
| `disable` | — | — | yes | — |
| `mainAgent` | — | — | — | yes |
| `subagent` | — | — | — | yes |
| `commandExecutionPolicy` | — | — | — | yes |

**The part that does not, and says nothing when it does not**: a field absent from a column is not rejected there -- it is read past. Nothing warns, no run fails, and the component behaves differently with the same bytes. Where the field was carrying a restriction, the restriction is simply gone. Check the column before relying on one.

## Before you ship one

- **The surface is declared, so the component is a promise.** Every kind   this provider declares is a promise of a rollback. A component written   to a path the declaration does not carry is installed by nobody and   removed by nobody.
- **Name it once.** Where the product derives identity from the directory   or the filename, the frontmatter `name` is either redundant or a second   place to be wrong. Keep them equal.
- **Read it back.** After an install, look at the file where the product   reads it, not at the step that put it there.
