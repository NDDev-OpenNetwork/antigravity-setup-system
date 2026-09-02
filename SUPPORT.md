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
This product documents no environment variable for its configuration home, so launch cannot point it at the explicit target.

A provider that advertised an operation it cannot perform would let a caller ask
for something that cannot be honoured, which is worse than not offering it.

All five core operations do work: `backup`, `restore`, `remove`, `install` and
`replace`, both from the local setup catalog and from an `ai-stp-bundle/1`
arriving over the wire.

## Using this against a home you already have

**An owned namespace is removed whole.** The table below says what this build
owns; `remove` deletes each of those paths entirely, and a backup slot holds
what was there first. That includes content this build never wrote -- if the
product itself put a key in a configuration file this provider owns, `remove`
takes the file, not the keys this provider added to it.

Measured, with the real product: launching Codex through `launch` and running
`mcp add` writes `~/.codex/config.toml` with an `[mcp_servers.*]` entry; a
later `install` captures that file into a slot and replaces it; a later
`remove` deletes it. The entry is not lost -- `backups` lists the slot as
*before install, setup none*, and restoring it returns the file byte for byte
-- but it is not in the target either.

So: point `--target` at a home you are willing to have managed. `backups
--target <dir>` names every earlier state and which setup each preceded, and
`restore --backup <ref>` returns any of them exactly.

## When conformance says this provider is malformed

`ai-stp provider conformance --protocol-version 3` reports each case by name.
If the one that fails is `provider_info_v3_closed`, with a detail about fields
differing from the closed schema, **check the version of the checker before
suspecting this build**.

The v3 capability schema is compared as an exact field set, so a provider that
declares a field the checker predates is reported as malformed rather than as
newer. `scoped_projection_profiles` (`ADR-0125`) is the field this applies to,
and it is omitted entirely when empty -- so a build that declares no scope
satisfies an older checker by accident, and a build that declares one does not.

Two versions, two different answers, both measured:

| checker | result |
| --- | --- |
| `ai-stp-cli` 0.0.3 | five pass; Codex and Antigravity report `conforms=false`, detail *fields differ from the closed v3 schema* |
| `ai-stp-cli` 0.0.7 | six pass 23 of 23; Codex reports `conforms=false`, detail *a scoped projection profile names an unknown target scope* |
| `ai-stp-cli` 0.0.8 | **all seven pass**, 27 to 29 cases each |

The middle row was never a defect in this build, and the third row is how that
was settled: **it closed with no change on this side.** `0.0.7` carried the
field but its scope enum was `["project"]` alone, while the provider kit this
program vendors and verifies byte-for-byte gave `["project", "user_root"]`. The
kit is the artifact a provider is told to build against, so a build declaring
`user_root` was right by the document it was handed and wrong by the checker
shipped beside it. `0.0.8` shipped the enum, and a declaration that had been
correct for a month started being read as correct.

**Withdrawing a correct declaration to make a lagging instrument print green is
never the answer here.** The three rows above are the argument for that, and
they are also the argument for the rule this section exists for.

Which is the general rule this section exists for: **check the version of the
checker before suspecting this build**, and prefer the newest, because an older
one reports a wider failure than the one it found.

## What `status` reports, and what it does not

`state` answers **who manages this target**, and never *whether a setup is
installed*. Three values, and the distinction matters most for the fourth
situation, which is not a fourth value:

| | |
| --- | --- |
| `missing` | the directory is empty |
| `unmanaged` | it holds content, none of it this provider's |
| `managed` | this provider's state file is present and current |

`missing` used to be looser -- it asked whether this provider owned anything,
so a directory full of another product's files reported `missing`. A consumer
reads this to decide what it is looking at, and being told a populated
directory is empty invites it to treat the place as free. Emptiness is about
the directory, not about us.

**After a `remove`, `state` stays `managed`, and that is the honest answer.**
The setup is gone -- no file a product reads survives it -- but the control
directory and a backup slot remain, and that slot is what makes the removal
reversible: `restore` brings the setup back. A target reported as `missing`
while a restore is pending would be a lie in the direction that costs someone
their data.

Whether a setup is installed is carried by `setup_stable_id`, which is `null`
exactly when none is. That is the field to test, not this word.
`target_identity_digest` corroborates it -- after a remove it is the digest of
an empty tree -- but the field is the direct answer and the digest is not.

## The network, stated exactly

**This artifact does not link the network, and no local phase can spawn
anything that could.** Two lints hold it rather than a promise: `std::net` is
refused outright, and `std::process::Command` is refused everywhere but two
named places -- the `launch` command, which is declared in `provider-info` and
absent from builds that do not declare it, and a lifecycle probe that drives
this binary's own executable. Adding a `tar` shell-out to ordinary code fails
the build with *only `launch` may spawn, and it is declared*. Every crate that
may be linked is named in `deny.toml`, so a transitive dependency cannot arrive
unread.

Those are claims about the source, and a lint can be wrong, bypassed, or simply
disbelieved. So `ci` reads the shipped binary too: a `boundary` job asks the
import table of the artifact this build produces whether any network symbol is
present, and whether a build declaring no `launch` imports anything that could
spawn. You can run it yourself against a downloaded release --
`nm -D --undefined-only <binary>` on Linux, `nm -u` on macOS -- and it needs no
part of this repository to be trusted.

**What that does not buy, said plainly because the stronger claim is the
tempting one.** This is a dynamically linked program: it imports `syscall` from
libc like any other, so no property of the binary can prove a socket is
unreachable to code that is determined to open one. What is proven is narrower
and still worth having: no code path here reaches for the network, none can be
added without the build refusing, and no local phase can hand the job to a
child process. If your threat model needs the guarantee rather than the
absence, run `plan` and `apply` under whatever sandbox you already trust; both
phases are offline by design, and `apply` verifies the digests it was given
with the network gone.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.gemini`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `antigravity-cli/settings.json` | `setting` | [source](https://antigravity.google/docs/settings) -- confirmed against the product's own embedded reference, read from the 1.1.22 artifact whose bytes match this baseline's sha256 |
| `antigravity-cli/keybindings.json` | -- | [source](https://antigravity.google/docs/settings) |
| `antigravity-cli/plugins` | -- | [source](https://antigravity.google/docs/cli/plugins/) |
| `config/plugins` | `plugin` | [source](https://antigravity.google/docs/plugins) |
| `config/skills` | `skill` | [source](https://antigravity.google/docs/skills) -- confirmed against the product's own embedded reference, read from the 1.1.22 artifact whose bytes match this baseline's sha256 |
| `config/agents` | `agent` | [source](https://antigravity.google/docs/subagents/) |
| `config/hooks.json` | `hook` | [source](https://antigravity.google/docs/hooks) -- confirmed against the product's own embedded reference, read from the 1.1.22 artifact whose bytes match this baseline's sha256 |
| `config/mcp_config.json` | `mcp` | [source](https://antigravity.google/docs/mcp) -- confirmed against the product's own embedded reference, read from the 1.1.22 artifact whose bytes match this baseline's sha256 |
| `config/global_workflows` | `command` | [source](https://antigravity.google/docs/rules-workflows/) -- measured in the 1.1.22 binary; confirmed against the product's own embedded reference, read from the 1.1.22 artifact whose bytes match this baseline's sha256 |
| `config/rules` | `instruction` | measured from the 1.1.22 artifact's own embedded reference, digest verified before reading, 2026-08-29 |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### A second target: `target_scope: project`

Rooted at `.agents`, which is not the configuration home
above. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `.agents/skills` | `skill` | [source](https://antigravity.google/docs/skills) |
| `.agents/agents` | `agent` | [source](https://antigravity.google/blog/introducing-custom-agents) |
| `.agents/plugins` | `plugin` | [source](https://antigravity.google/docs/plugins) |
| `.agents/hooks.json` | `hook` | [source](https://antigravity.google/docs/hooks) |
| `.agents/mcp_config.json` | `mcp` | [source](https://antigravity.google/docs/mcp) |

This root is read by several products at once, so under this scope
`remove`, the backup and a restore act on the files this program
recorded writing rather than on the directory whole. A neighbour's
files are never captured into a backup slot here, and never reverted
by a restore.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`.antigravity-setup-system`** -- This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is. (this provider's own contract; no vendor page is involved)

**`NDDEV-ANTIGRAVITY-PROVIDER.json`** -- This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one. (this provider's own contract; no vendor page is involved)

**`antigravity-cli/builtin`** -- The product's own bundled skills and resources, including `builtin/skills` and a `.checksum`. Shipped with the program, replaced by an update, and never a person's to configure -- so never this provider's to own or restore. (measured from a run of the 1.1.22)

**`antigravity-cli/cache`** -- The CLI's own cache, holding cache/projects.json. Runtime state with a lifetime the product owns -- the string appears as a path literal in the 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood. Never captured into a backup slot: a backup of a cache is a stale answer with a signature on it. (measured from the 1.1.22 binary; no vendor page names it)

**`antigravity-cli/hooks.json`** -- A second hooks file, beside the owned `config/hooks.json`. Both appear as path literals in the 1.1.22 binary.

**Which one the product means is documented, in its own release notes, and the earlier text here said it was not.** From the changelog embedded in the pinned artifact: *"Fixed a bug where the `/hooks` command wrote configurations to `~/.gemini/antigravity-cli/hooks.json` instead of the shared `~/.gemini/config/hooks.json`, ensuring hooks remain synchronized between the TUI and the backend."* So `config/hooks.json` is the shared surface and this path is where a fixed defect used to write.

Still declined rather than owned, and now for a stated reason instead of an absence: it is the output of a bug the vendor repaired, so a file here on a user's machine is residue from an older build rather than configuration this provider should manage. Owning it would have a restore put back a file the product stopped writing.

Corrected 2026-08-29. The previous text read *"which one the product prefers when both exist is not documented and has not been measured"* — true when written, and false against a changelog that was inside the artifact this baseline already pins. Found because a user's issue about a different hooks path sent someone reading the same bytes again. (measured from the 1.1.22 binary; no vendor page names it)

**`antigravity-cli/runtime-state`** -- One row for a whole subtree, because listing eleven siblings would suggest each was weighed separately. A single run created `bin/`, `brain/`, `cache/`, `cli.log`, `conversations/`, `conversation_summaries.db` with its `-shm` and `-wal`, `crashes/`, `installation_id`, `jetski_state.pbtxt`, `knowledge/`, `last_check.timestamp`, `log/` and `updater/`. All are the product's own lifetime. None is configuration and none is ownable. The same run also wrote **outside the configuration home entirely** -- `~/.cache/ms-playwright-go`, under the user's cache directory. That has no row of its own because every recorded path here is relative to the target, and this one is relative to a root this provider never evaluates against; the guard that enforces it refused the row, correctly. It is recorded in this sentence instead, so a reader looking for everything the product writes does not stop at `~/.gemini`. (measured from a run of the 1.1.22)

**`antigravity/transcript.jsonl`** -- The conversation transcript, beside antigravity/artifacts. A person's session content, and the string appears as a path literal in the 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood. Never owned and never backed up, for the reason the never_touch list gives about a neighbour's credentials: copying it into a slot would put private text somewhere with a retention policy nobody chose. (measured from the 1.1.22 binary; no vendor page names it)

**`config/.migrated`** -- An empty marker the product writes beside its config. It records that a layout migration ran, which is the product's bookkeeping about its own history and never a projection surface. (measured: created by a single run of the 1.1.22 in a contained HOME)

**`config/config.json`** -- The product's own user settings, created on first run: it held `{"userSettings": {"remoteControlHostname": "<this machine's hostname>"}}`. Distinct from the owned `antigravity-cli/settings.json`, which is where the documented posture keys live. Not owned: no vendor page names it, it carries machine identity rather than configuration a setup would choose, and owning it would put a hostname into every backup slot. (measured: created by a single run of the 1.1.22 in a contained HOME)

**`config/import_manifest.json`** -- Written by the product's own plugin installer. Measured against the 1.1.22 linux/x86_64 bytes (sha256:1e1a219a...), verified before running and executed in a contained HOME so nothing of the owner's was touched: `antigravity plugin install` created this file beside config/plugins. It is the installer's bookkeeping about what it imported, not a projection surface, and owning it would let this provider's `remove` erase the product's record of plugins a person installed by hand. (measured from the product; https://antigravity.google/docs/cli/plugins/)

**`config/projects`** -- Per-project state the product keeps under its own home. Not configuration this provider projects, and the string appears as a path literal in the 1.1.22 binary; no vendor page names it and no run of this provider has made the product write it, so it is recorded as seen rather than as understood. (measured from the 1.1.22 binary; no vendor page names it)

**`config/workflows`** -- A second **global** workflow root, beside the owned `config/global_workflows`. Both are named as sources in the product's own embedded migration reference, which converts workflows into skills and tabulates them under scope `Global`: `~/.gemini/config/global_workflows/<name>.md` and `~/.gemini/config/workflows/<name>.md`, both targeting `~/.gemini/config/skills/<name>/SKILL.md`. The workspace tier is a different set of rows -- `<workspace>/.agents/workflows/<name>.md` and its siblings.

**This row said "the workspace tier" until 2026-08-31, and declined it on the ground that this provider configures a home rather than a checkout.** That argument is sound and is about a different path: this one is inside the home. The decline is right for another reason -- nothing here writes a workflow, the product is migrating the form away, and owning it would empty a person's un-migrated workflows on a posture switch, which is the shape `custody_namespaces` exists to stop. A true decline on a false reason survives every check, because nothing compares a reason with the thing it is about. (measured 2026-08-28 in the product's own embedded reference, read from the 1.1.22 artifact whose bytes match this baseline's sha256)

**`config/workflows.json`** -- The manifest beside that second global workflow root, beside the owned `config/global_workflows`. Both are named as sources in the product's own embedded migration reference, which converts workflows into skills and tabulates them under scope `Global`: `~/.gemini/config/global_workflows/<name>.md` and `~/.gemini/config/workflows/<name>.md`, both targeting `~/.gemini/config/skills/<name>/SKILL.md`. The workspace tier is a different set of rows -- `<workspace>/.agents/workflows/<name>.md` and its siblings.

**This row said "the workspace tier" until 2026-08-31, and declined it on the ground that this provider configures a home rather than a checkout.** That argument is sound and is about a different path: this one is inside the home. The decline is right for another reason -- nothing here writes a workflow, the product is migrating the form away, and owning it would empty a person's un-migrated workflows on a posture switch, which is the shape `custody_namespaces` exists to stop. A true decline on a false reason survives every check, because nothing compares a reason with the thing it is about. (measured 2026-08-28 in the product's own embedded reference, read from the 1.1.22 artifact whose bytes match this baseline's sha256)

**`managed-config`** -- Not a path in the target, and recorded because **there is no such path** -- the same measured absence as pi's, made against a very different artifact.

Measured 2026-08-29 against the 1.1.22 binary, whose bytes match this baseline's own sha256. Searched for `/etc/<product>`, `%ProgramData%\\<product>` and `/Library/Application Support/<product>` literals. The `/etc/` hits this binary does carry are `ssl`, `pki`, `ssh`, `passwd`, `shadow`, `hosts`, `resolv.conf` and `sudoers` -- the Go standard library and the embedded browser stack, not this product's configuration. **A grep that counted those as a managed policy would have invented one**, which is why the search is recorded with what it found and not only with its conclusion.

Its settings-key registry names `enterprisePreferences`, and that is the closest thing to a managed layer here. It is not a file: it sits beside `userStatus`, `oauthTokenInfo` and `useAICredits` -- account state this product receives, not an administrator's policy on disk. Nothing this provider writes can be overridden by a file it does not own, because no such file is named.

Absence of a literal is not proof a path cannot exist. This row says what was searched. (measured in the 1.1.22 artifact)

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
