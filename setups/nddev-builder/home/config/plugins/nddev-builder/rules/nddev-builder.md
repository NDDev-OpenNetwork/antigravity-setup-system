# NDDev builder policy for Antigravity CLI

- Build complete tool collections through the setup authoring workflow in
  `skills/antigravity-surfaces/SKILL.md`. Document the tasks, supplied
  capabilities, native invocation, dependencies and validation evidence.
- Resolve native paths and plugin contents from that skill's routed
  references, especially `references/authoring-plugins.md`; do not copy
  another harness's plugin shape.
- Antigravity shares `~/.gemini` with Gemini CLI. Preserve root-level
  settings and credentials. Bind the declared Antigravity scope and
  exact target instead of inventing a home-override variable.
- A complete setup may span declared scopes through ai-stp transactions.
  Each provider request still names exactly one target.
- Exercise authoring, installation, native loading and backup/restore
  in disposable homes, targets and prefixes. A live apply or publication
  must be part of the task and use its exact reviewed lifecycle.
