# Support

## Before opening anything

`--help` states what this build does and does not do. `status --target <dir>
--json` reports what it found in a target without changing it, and its output is
safe to share: it carries identities and digests, never secret values.

## Where to go

| You have | Go to |
| --- | --- |
| A defect | [Issues](../../issues) — use the defect template |
| A question about behaviour | [Issues](../../issues) — a blank issue is fine |
| A vulnerability | [Security advisories](../../security/advisories/new), privately |

Never open a public issue for a vulnerability, and never paste credentials,
tokens, or the contents of a backup slot anywhere in this repository. A backup
slot holds whatever the target held when it was captured.

## What this build does, and what it does not

The software lifecycle — installing, updating and removing the product
itself — is declared and does work. `plan` names the exact bytes offline,
whoever holds the network fetches them, and `apply` verifies and installs
with the network gone.

`launch` is not declared here.
This product documents no environment variable for its configuration home,
so a launch could not point it at the `--target` every command here takes.
It would start the product against whatever home the product picked for
itself, while reporting that it had honoured the target.

A provider that advertised an operation it cannot perform would let a caller ask
for something that cannot be honoured, which is worse than not offering it.

All five core operations do work: `backup`, `restore`, `remove`, `install` and
`replace`, both from the local setup catalog and from an `ai-stp-bundle/1`
arriving over the wire.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.gemini`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `antigravity-cli/settings.json` | `setting` | [source](https://antigravity.google/docs/settings) |
| `antigravity-cli/keybindings.json` | -- | [source](https://antigravity.google/docs/settings) |
| `antigravity-cli/plugins` | `plugin` | [source](https://antigravity.google/docs/plugins) |
| `config/plugins` | -- | [source](https://antigravity.google/docs/plugins) |
| `config/skills` | `skill` | [source](https://antigravity.google/docs/skills) |
| `config/agents` | `agent` | [source](https://antigravity.google/docs/agents) |
| `config/hooks.json` | `hook` | [source](https://antigravity.google/docs/hooks) |
| `config/mcp_config.json` | `mcp` | [source](https://antigravity.google/docs/mcp) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
