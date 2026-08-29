---
name: antigravity-surfaces
description: Find the right file before writing to an Antigravity CLI home. Use when adding or changing skills, agents, plugins, hooks, MCP servers, settings or keybindings under ~/.gemini, and whenever a path there is about to be written without being certain which product owns it.
---

# Antigravity's surfaces, and the ones next to them

Antigravity CLI is a **guest** in `~/.gemini`. Gemini CLI was there first and
still owns files at the root of it. A write aimed one directory too high does
not fail — it succeeds, and corrupts another product's configuration.

That is the whole reason this skill exists. Everything below is read from
`antigravity-baseline.json`, which records what the product's own documentation
says.

## What belongs to Antigravity

| What | Global | Per project |
| --- | --- | --- |
| Settings | `~/.gemini/antigravity-cli/settings.json` | — |
| Keybindings | `~/.gemini/antigravity-cli/keybindings.json` | — |
| Plugins (CLI-managed) | `~/.gemini/antigravity-cli/plugins/` | — |
| Plugins (placed by hand) | `~/.gemini/config/plugins/` | `.agents/plugins/`, `_agents/plugins/` |
| Skills | `~/.gemini/config/skills/` | `.agents/skills/` |
| Agents | `~/.gemini/config/agents/` | `.agents/agents/` |
| Hooks | `~/.gemini/config/hooks.json` | `.agents/hooks.json` |
| MCP servers | `~/.gemini/config/mcp_config.json` | `.agents/mcp_config.json` |
| Workflows | `~/.gemini/config/global_workflows/` | `.agents/rules/` |

Two plugin roots, and they are not interchangeable. `antigravity-cli/plugins/`
is where the CLI puts what it installs; `config/plugins/` is where something
else puts a plugin by hand. Writing into the first pretends to be the CLI.

## What does not

Inside the same home, and **not** Antigravity's:

- `~/.gemini/settings.json` — Gemini CLI's own settings. The similarity of the
  name to `antigravity-cli/settings.json` is the trap: they are different files
  for different products, one directory apart.
- `~/.gemini/oauth_creds.json`, `~/.gemini/google_accounts.json` — credentials.
- `~/.gemini/tmp/` — runtime scratch.

None of these should be read, written, or copied into a backup. A backup holding
someone else's credentials is a leak with a schedule.

## The global instruction, and what this section used to say

- **`config/rules/` is the global instruction surface.** The product's own
  embedded reference lists the customization elements available *within any of
  the customization roots*, and `rules/` is the second of them: *"**Rules**
  (Markdown Files): Location: `rules/` (relative to the customization root) or
  standalone `GEMINI.md`/`AGENTS.md` files."* It recommends a consolidated
  `AGENTS.md` under `rules/` over separate rule files. The `instruction` kind is
  declared here and routes to it.

- **A plugin's `rules/` is the other one, and it is what these setups use.** A
  plugin's rules are ingested with its skills and hooks when it is enabled, on
  one switch, and unlike `config/rules` it is not emptied when a setup installs.

**Until 2026-08-29 this section was titled "One thing this product does not
have" and said the opposite** — that instructions exist only at workspace scope,
that there is no home-level `AGENTS.md` to write, and that writing one places a
file the product never reads. It ended: *"That is a fact about the product, not
a gap to fill."*

Every part of that was wrong, and the last sentence is the reason it survived:
it tells a reader the question is settled. The evidence was inside the artifact
this repository already pins and had already downloaded — a reference the
product ships with itself — and the claim had been taken from the vendor pages
that happened to have been read. Twice more in the same declaration, `command`
and `plugin` were corrected the same way and for the same reason.

The lesson is not about this product. **A negative taken from the pages you
happened to read is a claim, and writing "that is a fact, not a gap" is how a
claim stops being re-examined.**

**This section used to say the same about commands, and it was wrong.** The
declaration was corrected on 2026-08-28 against the pinned 1.1.22 binary, which
carries `.gemini/config/global_workflows/<name>.md` as a path literal; the
vendor documents a *+ Global* button that creates one *accessible across all
your workspaces*. A global workflow is invoked as `/workflow-name`, which is
what makes it a command rather than a fourth kind of instruction. The sentence
outlived the measurement that refuted it because nothing here reads this file
against the baseline -- see `references/authoring-commands.md` for the shape,
and note the 12,000-character limit per file, which no other harness has.

## Before writing

1. **Name the surface from the table**, not from memory of another harness.
   Claude, Codex, Cursor, Grok, OpenCode and Pi each put these somewhere else,
   and the shapes rhyme enough to be confused.
2. **Check the neighbour.** If the path you are about to write is one directory
   above a surface in that table, stop: it is probably Gemini CLI's.
3. **Look at what is there.** `antigravity-setup-system status --target <home>`
   reports what a home holds without creating anything in it.
4. **Let something reversible do the write.** Every mutation through
   `antigravity-setup-system` captures a backup first, so `restore` has
   somewhere to return to.

## A plugin's own shape

A native plugin here is a directory with a manifest in it, and the vendor names
what may sit beside it:

```
plugin.json        (required; its only documented field is optional)
mcp_config.json
hooks.json
skills/
rules/
```

`plugin.json` is what makes the directory a plugin. Without it the directory is
not one, and nothing says so.

**This block listed `agents/` and omitted `mcp_config.json` and `hooks.json`
until 2026-08-28** — it had been written from the shape of the harness next
door rather than from this product's own page. There is no `commands/` here:
that is Cursor's shape, and copying it produces a directory the product does
not read. There is no marketplace either — the product documents none, so a
plugin arrives by being placed. See `references/authoring-plugins.md`.

## Routing

Everything below is generated by `tools/build_nddev_builder.py` and cannot go
stale against the declaration. This file is hand-written and can, which is why
two of its paragraphs above now carry the date they were corrected.

- **What this harness owns, declines, and why** — `references/surfaces.md`
- **The configuration file itself -- its grammar, whether comments parse, and
  whether what you write is the effective value**: read
  `references/authoring-settings.md`.
- **The instruction file, and which products read a neighbour's**: read
  `references/authoring-instructions.md`.
- **The second target this harness owns, and why a setup cannot carry a
  component for it**: read `references/second-target.md`. Generated from the
  baseline's scoped block.
- **The commands, the invariants, and the software half** — `references/lifecycle.md`
- **The gate, the render check, and the one rule** — `references/validation.md`
- **Writing a skill** — `references/authoring-skills.md`
- **Writing an agent** — `references/authoring-agents.md`
- **Writing a workflow, which is this product's command** — `references/authoring-commands.md`
- **Writing a hook** — `references/authoring-hooks.md`
- **Writing a plugin** — `references/authoring-plugins.md`
- **Configuring an MCP server, and what not to put in the file** — `references/authoring-mcp.md`
