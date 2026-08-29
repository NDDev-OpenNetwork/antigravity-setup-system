# NDDev builder policy for Antigravity CLI

- Write only under the surfaces `skills/antigravity-surfaces/SKILL.md` lists.
  Everything else in `~/.gemini` belongs to Gemini CLI.
- Never read, write or back up `settings.json`, `oauth_creds.json`,
  `google_accounts.json` or `tmp/` at the root of that home.
- A plugin here carries `plugin.json`, `skills/`, `agents/` and `rules/`. It
  does not carry `commands/`; that shape belongs to another harness.
- Both a global instruction and a global command exist here, and this rule used
  to deny both. The instruction is a consolidated `AGENTS.md` under a `rules/`
  directory -- the product's own reference recommends it over separate rule
  files -- and a global command is a Markdown file in `config/global_workflows/`,
  invoked as `/workflow-name` across every workspace.
- Prefer `status --target <absolute-home> --json` over reading the tree by hand,
  and prefer a manager mutation over an edit, because the manager captures a
  backup first.
- Keep every path absolute and explicit. Nothing here is inferred from a home
  directory.
