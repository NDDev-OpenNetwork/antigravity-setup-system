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

## Two things this product does not have

- **No global instruction file.** Antigravity documents instructions only at
  workspace scope, under a project's `.agents/`. There is no home-level
  `AGENTS.md` to write, and writing one places a file the product never reads.
- **No global command surface.** Same reason.

If a task says "add an instruction" or "add a command" to a home here, the
answer is that the home has nowhere for it. That is a fact about the product,
not a gap to fill.

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

A native plugin here projects four things, and no others:

```
plugin.json
skills/
agents/
rules/
```

There is no `commands/` — that is Cursor's shape, and copying it produces a
directory the product does not read. There is no marketplace: the product
documents none, so a plugin arrives by being placed.
