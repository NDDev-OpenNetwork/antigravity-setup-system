# The second target this harness owns

## `target_scope: project`, rooted at `.agents`

**`.agents` is not this product's configuration home.** It is a
different target, reached by a consumer naming the scope on the
request, and every path below is relative to that root rather than
to the home -- writing the root into the path again would nest it
twice, which is a mistake this estate has made and shipped.

| path | routes | decided by | exercised by |
|---|---|---|---|
| `.agents/skills` | skill | <https://antigravity.google/docs/skills> | read its bytes |
| `.agents/agents` | agent | <https://antigravity.google/blog/introducing-custom-agents> | read its bytes |
| `.agents/plugins` | plugin | <https://antigravity.google/docs/plugins> | read its bytes |
| `.agents/hooks.json` | hook | <https://antigravity.google/docs/hooks> | read its bytes |
| `.agents/mcp_config.json` | mcp | <https://antigravity.google/docs/mcp> | *nothing -- a page* |
### `.agents/skills`, as measured

"Create a directory named .agents/skills/ at your project root"; the product still reads the legacy .agent/skills, which is not owned

**Promoted from `page` to `bytes` on 2026-08-29**, found in the 1.1.22 artifact with the digest verified before reading: `{workspace}/.agents/skills/{skill_name}/SKILL.md` as a path template, beside its reference's *"customization root (e.g. `.agents/skills/`)"*.

**The first search was wrong and nearly promoted two rows it should not have.** It used `grep` without `-F`, so the `.` in `.agents/skills` matched any character and `learning/gemini/agents/skills` counted as a hit -- as did `customizations/builtin/agents/agents.init` for the row below it. The control was absent in both runs and said nothing about it, because an invented string has no wildcard to exploit. **A control proves the search finds what is there; it does not prove the search is asking the right question.** Fixed-string matching is what separated the four real paths from two coincidences.

### `.agents/agents`, as measured

"place it inside that workspace's .agents/agents/ directory"

**Promoted from `page` to `bytes` on 2026-08-29**, found in the 1.1.22 artifact with the digest verified before reading: `{workspace}/.agents/agents/{agent_name}/` as a path template.

**The first search was wrong and nearly promoted two rows it should not have.** It used `grep` without `-F`, so the `.` in `.agents/skills` matched any character and `learning/gemini/agents/skills` counted as a hit -- as did `customizations/builtin/agents/agents.init` for the row below it. The control was absent in both runs and said nothing about it, because an invented string has no wildcard to exploit. **A control proves the search finds what is there; it does not prove the search is asking the right question.** Fixed-string matching is what separated the four real paths from two coincidences.

### `.agents/plugins`, as measured

**Promoted from `page` to `bytes` on 2026-08-29**, found in the 1.1.22 artifact with the digest verified before reading: its own reference: *"customization root (e.g. `.agents/plugins/`)"*.

**The first search was wrong and nearly promoted two rows it should not have.** It used `grep` without `-F`, so the `.` in `.agents/skills` matched any character and `learning/gemini/agents/skills` counted as a hit -- as did `customizations/builtin/agents/agents.init` for the row below it. The control was absent in both runs and said nothing about it, because an invented string has no wildcard to exploit. **A control proves the search finds what is there; it does not prove the search is asking the right question.** Fixed-string matching is what separated the four real paths from two coincidences.

### `.agents/hooks.json`, as measured

"a hooks.json file located in your customization directory (e.g., .agents/ in your workspace or ~/.gemini/config/)"

**Promoted from `page` to `bytes` on 2026-08-29**, found in the 1.1.22 artifact with the digest verified before reading: its own reference, and a changelog entry -- *"Fixed workspace-local hooks defined in `<workspace>/.agents/hooks.json` not loading after trusting a folder"*.

**The first search was wrong and nearly promoted two rows it should not have.** It used `grep` without `-F`, so the `.` in `.agents/skills` matched any character and `learning/gemini/agents/skills` counted as a hit -- as did `customizations/builtin/agents/agents.init` for the row below it. The control was absent in both runs and said nothing about it, because an invented string has no wildcard to exploit. **A control proves the search finds what is there; it does not prove the search is asking the right question.** Fixed-string matching is what separated the four real paths from two coincidences.

### `.agents/mcp_config.json`, as measured

"globally at ~/.gemini/config/mcp_config.json (or locally in your workspace under .agents/mcp_config.json)"

**Searched in the product's own pinned bytes on 2026-08-29 and not found, which argues nothing either way.** Fixed-string, anchored to this product's configuration home -- the bare leaf name is in every one of these binaries and proves nothing, so only the anchored form counts. An invented path was searched in the same run and was also absent, so the search discriminates.

This row stays `page` because **a path built by joining a directory to a name at runtime never appears as a literal**, and that is the shape of every remaining one. Moving it off `page` needs the product run against a target and asked what it resolved, not a deeper grep.


**A setup cannot carry one of these.** A setup is installed into one
target and its payload is relative to that target, so a component
for this scope is installed by the consumer against that root -- not
by a setup aimed at the configuration home. If you are looking for
where to put one by hand, it is the path above joined to the root
above, and nowhere under the home.

**The root is shared, and that changes what removal means.** Several
products read it. Under this scope `remove`, the backup and a
restore act on the files this provider recorded writing rather than
on the directory whole, so a neighbour's files are never captured
into a slot here and never reverted out of one.

