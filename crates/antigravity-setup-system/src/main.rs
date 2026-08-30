//! The Antigravity CLI setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with the other setup systems, so a change to
//! behaviour lands once and a change to Antigravity CLI's surface lands here.
//!
//! # This target belongs to more than one product
//!
//! Every other harness here owns its home outright. Antigravity CLI does not:
//! it keeps its configuration *inside* `~/.gemini`, alongside Gemini CLI's own
//! files, split across two subdirectories -- `antigravity-cli/` for what belongs
//! to this CLI alone, and `config/` for the surfaces it shares.
//!
//! So the target named below is a directory this provider is a guest in. Two
//! things follow, and both are load-bearing rather than decorative:
//!
//! every namespace is prefixed, so nothing at the root of `~/.gemini` is ever
//! claimed -- most importantly `settings.json`, which is Gemini CLI's and not
//! this product's despite the name; and Gemini's credentials appear in
//! [`Harness::never_touch`] even though nothing would reach them anyway,
//! because a guard that only holds while the namespace list is correct is a
//! guard that fails the first time the list is edited.

use std::process::ExitCode;

mod software;

use harness_runtime::{Harness, LaunchBinding, Scoped};
use provider_v3::{ComponentKind, ProjectionKind, TargetScope};

/// Everything specific to Antigravity CLI, verified against
/// `antigravity-baseline.json`.
pub const ANTIGRAVITY: Harness = Harness {
    harness_id: "antigravity",
    provider_id: "antigravity-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Antigravity CLI",
    vendor: "Google",
    documented_config_home: "~/.gemini",
    // Empty on purpose: the product documents no override for its home. An
    // invented variable name would read as a fact and be one nothing honours.
    //
    // **Re-measured 2026-08-29, because this is a negative and a negative from
    // a page is the weakest thing this estate records.** Every environment
    // variable the pinned 1.1.22 binary names with a product prefix was
    // enumerated -- `GEMINI_API_KEY`, `GEMINI_TTS_VOICE_REPLICATION`,
    // `GEMINI_IOS_PRIMES`, `ANTIGRAVITY_SIDECAR_WEB_PORT`,
    // `ANTIGRAVITY_PROJECT_ID`, `ANTIGRAVITY_LS_ADDRESS`,
    // `ANTIGRAVITY_CSRF_TOKEN`, `ANTIGRAVITY_AGENTAPI_EXE`, `AGY_ADC_AUTH`,
    // `AGY_CLI_DISABLE_LATEX`. Not one names a home or a configuration
    // directory. Six plausible spellings were searched for by name and all
    // answered zero, beside an invented control that also answered zero -- so
    // the search discriminates by finding the ten that are there.
    //
    // `XDG_CONFIG_HOME` appears five times and is **not** evidence: its
    // neighbours are Go runtime, SSH, gzip, protobuf and TOML strings, which is
    // the vendored-dependency trap this estate already recorded for grok.
    //
    // This is why `launch` is the one command of the seven this harness does
    // not declare: a launch could not point the product at the `--target` it
    // was handed, and would start it against whatever home it picked for
    // itself while reporting that it had honoured the target.
    config_home_env: "",
    // No variable at all, which is why this build has never declared launch.
    launch_binding: LaunchBinding::Undocumented,
    // Not measured. The two artifacts this estate has read for this question are
    // claude's, which carries `DISABLE_UPDATES`, and codex's, which carries no
    // such literal. This product has been asked nothing, and an empty value here
    // says the launch environment is untouched rather than that the product
    // leaves the bytes alone.
    updates_off_env: "",
    // One home, one variable: nothing here is conditional.
    config_home_note: "",
    control_directory: ".antigravity-setup-system",
    state_file: "NDDEV-ANTIGRAVITY-PROVIDER.json",
    predecessor_state_file: "NDDEV-ANTIGRAVITY-CLI-SETUP.json",
    profile_id: "antigravity/native-files/1",
    // Everything outside this list is a sibling overlay preserved verbatim --
    // which here includes another product's entire configuration.
    native_namespaces: &[
        "antigravity-cli/settings.json",
        "antigravity-cli/keybindings.json",
        "antigravity-cli/plugins",
        "config/plugins",
        "config/skills",
        "config/agents",
        "config/hooks.json",
        "config/mcp_config.json",
        // Added 2026-08-28. Global workflows are Markdown files invoked as
        // `/workflow-name`, which makes them commands; the binary carries
        // `.gemini/config/global_workflows/<name>.md` and the vendor describes
        // the button that creates one across all workspaces. The comment below
        // said this product documents commands only at workspace scope, and
        // this is the evidence against it.
        "config/global_workflows",
        // Added 2026-08-29, and the comment below was wrong a second time.
        // The product's own embedded documentation lists five customization
        // *elements* available "within any of the customization roots" --
        // skills, rules, plugins, MCP servers and hooks -- and this
        // declaration carried four of them. `rules/` was the missing one, and
        // it is the instruction surface.
        "config/rules",
    ],
    // Gemini CLI's own, not this product's. Never read, never written, and
    // never copied into a backup slot: a backup of someone else's credentials
    // is a leak with a schedule.
    // Nothing measured. This product's alternate spellings, if it has
    // any, have not been asked for -- empty here says nobody looked,
    // not that the product reads one name.
    shadowing_names: &[],
    never_touch: &[
        "settings.json",
        "oauth_creds.json",
        "google_accounts.json",
        "tmp",
    ],
    // No near neighbour measured for this product. A marker listed here is a
    // refusal waiting to happen, so nothing is listed without evidence.
    foreign_homes: &[],
    permission_profiles: &["default"],
    // **This comment said "no instruction" and "no command", and both were
    // wrong.** It is kept as the shape rather than deleted: it read as a
    // measured absence for weeks, and each half was a negative taken from what
    // a page happened to discuss rather than from the product.
    //
    // `Command` went first, on 2026-08-28: `~/.gemini/config/global_workflows/`
    // holds Markdown workflows invoked as `/workflow-name`, across all
    // workspaces.
    //
    // `Instruction` followed. The evidence was in the same binary the whole
    // time: its embedded reference lists the customization *elements* --
    // *"Within any of the customization roots above, you can define: 1. Skills
    // … 2. **Rules** (Markdown Files): Location: `rules/` (relative to the
    // customization root) or standalone `GEMINI.md`/`AGENTS.md` files … 3.
    // Plugins … 4. MCP Servers and Hooks."* Four of those five were declared
    // here and `config/rules` was not. Both the namespace and the kind are
    // declared now.
    //
    // **The kind was reverted once on a misreading of our own tool, and the
    // misreading is worth keeping.** `declared_route_is_compilable:instruction`
    // fails while the consumer's `PROVIDER_RULES` has no `instruction` row for
    // this harness -- but that case carries `subject: consumer`, and the
    // checker's `conforms` is computed over provider-subject cases alone,
    // deliberately: a provider declaring a kind the compiler cannot route
    // *"has satisfied every obligation v3 places on it; the gap is ours, and
    // calling it non-conformance would name the wrong party in the one field
    // people read"*. `tools/conformance_report.py` was counting every failed
    // case regardless of subject, printed **REFUSED, 30 cases**, and that
    // number was written into a comment here and into the baseline as though
    // it were a fact about the protocol. It was a fact about our reader.
    //
    // Declaring the kind is safe while the route is missing: composition
    // refuses an antigravity instruction early with `native_surface_lost`,
    // exactly as before. What it buys is the ordering -- the provider is off
    // the critical path, so the route works the moment the consumer lands it,
    // with no window where an instruction composes and is refused late.
    //
    // The workspace tier -- `config/workflows/` and `config/workflows.json` --
    // stays unowned; that half of the old sentence is still true.
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Hook,
        ComponentKind::Mcp,
        ComponentKind::Plugin,
        ComponentKind::Setting,
        ComponentKind::Command,
    ],
    projection_kinds: &[ProjectionKind::NativeFiles, ProjectionKind::Plugin],
    // Antigravity is the one product of the seven that keeps a workspace copy
    // of its surfaces under a root this provider can be pointed at, and the
    // consumer asked for exactly that route in `ai_stp#424` and `#425`.
    //
    // Every path here was read from the vendor rather than from a routing
    // table, and the two the issues asked for are *not* here: no page names a
    // project-scoped command or instruction directory for this product, and a
    // declared kind is a promise of a rollback.
    scoped_projections: &[Scoped {
        target_scope: TargetScope::Project,
        // Distinct from the global identity, and it has to be: the digest binds
        // the declaration *together with* its scope, so two profiles differing
        // only in which target they own cannot share one.
        profile_id: "antigravity/native-files/project/1",
        component_kinds: &[
            ComponentKind::Skill,
            ComponentKind::Agent,
            ComponentKind::Hook,
            ComponentKind::Mcp,
            ComponentKind::Plugin,
        ],
        projection_kinds: &[ProjectionKind::NativeFiles, ProjectionKind::Plugin],
        // Relative to a workspace, not to `~/.gemini`. `config/skills` means
        // nothing here and `.agents/skills` means nothing there, which is why
        // one declaration could not have described both.
        native_namespaces: &[
            ".agents/skills",
            ".agents/agents",
            ".agents/plugins",
            ".agents/hooks.json",
            ".agents/mcp_config.json",
        ],
    }],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    // Generated by `build.rs` from this harness's `setups/` directory, so the
    // binary carries the catalog it is named after instead of hoping to find
    // one on a disk it was never shipped to.
    embedded_setups: include!(concat!(env!("OUT_DIR"), "/embedded_setups.rs")),
    software: Some(software::SOFTWARE),
};

fn main() -> ExitCode {
    harness_runtime::run(&ANTIGRAVITY, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    /// The directory name this harness's setups live under in the workspace.
    const TOOL: &str = "antigravity";
    /// The declaration under test, named once so the shared test below reads
    /// the same in all seven crates.
    const HARNESS: Harness = ANTIGRAVITY;

    /// `build.rs` put the whole catalog in, under the paths it will be read by.
    ///
    /// This does **not** test for staleness, and an earlier version of this
    /// comment claimed it did. It cannot: `build.rs` declares
    /// `rerun-if-changed` on the catalog directory, so editing a setup rebuilds
    /// the table before this runs, and the test would be comparing the tree
    /// with itself. Observed — a deliberately edited setup left it green.
    ///
    /// What it does test is the build script, against a walk written
    /// independently of it: every file present, none invented, bytes exact, and
    /// paths relative and slash-separated. That last one is the one that would
    /// really break — `join("/")` is the only reason these keys are usable on
    /// Windows, and a path built with the platform separator would still look
    /// perfectly correct in the generated source.
    /// The bytes this harness ships, pinned so they cannot change unseen.
    ///
    /// A setup's `definition_digest` is what makes two setups the same setup,
    /// and it appears in `list`, in a plan and in provider state -- and until
    /// this, nothing compared it to anything. A stray character in a setup file
    /// changed what the estate installs and every test stayed green.
    ///
    /// One aggregate rather than one per setup, because the claim is about the
    /// catalogue: sorted definition digests, joined by a newline, hashed. A
    /// deliberate change to a setup updates the line in the baseline, which is
    /// the point -- the peer calls this a golden and it earns itself the first
    /// time a row moves without anyone meaning it to.
    ///
    /// **And it is the three-OS check nothing else makes.** The setups are
    /// embedded with `include_bytes!`, so whatever the checkout holds is what
    /// ships; `.gitattributes` pins `eol=lf` to keep a Windows checkout from
    /// rewriting them, and this is the assertion that would notice if it ever
    /// stopped working. The matrix runs it on all three systems, so a digest
    /// that differed by platform could not stay hidden.
    #[test]
    fn the_catalogue_this_harness_ships_is_the_one_the_baseline_records() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let mut digests: Vec<String> = catalog
            .list()
            .unwrap()
            .iter()
            // **Both digests, because one of them holds nothing a person
            // reads.** `definition_digest` is the payload tree; the manifest --
            // `id`, `sources`, `description` -- was covered by no digest in this
            // estate, and those three are what a consumer renders on the surface
            // that precedes an install. A description was rewritten and the
            // whole gate stayed clean, which is how this was found.
            .map(|setup| format!("{}\n{}", setup.definition_digest, setup.manifest_digest))
            .collect();
        digests.sort();
        let joined = digests.join("\n");
        let aggregate = harness_runtime::digest_of_bytes(&joined);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let recorded = baseline["setup_catalogue_digest"].as_str().unwrap_or("");
        assert_eq!(
            aggregate, recorded,
            "the setups this binary ships are not the ones {TOOL}-baseline.json \
             records; if the change was meant, put this digest there"
        );
    }

    #[test]
    fn the_catalog_this_binary_carries_is_the_one_in_the_tree() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // The workspace holds one directory per harness; a rendered public tree
        // ships one harness and holds it flat. Same two candidates `build.rs`
        // chooses between, asked the same way.
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };

        // Only the setup directories, which is what the reader lists and what
        // `build.rs` embeds. A rendered public tree also carries a
        // `setups/README.md` at the catalog root, which belongs to no setup.
        let mut on_disk = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.join("setup.json").is_file())
            .collect();
        while let Some(directory) = stack.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    on_disk.push(path);
                }
            }
        }

        assert_eq!(
            HARNESS.embedded_setups.len(),
            on_disk.len(),
            "the binary carries {} files and the tree holds {}",
            HARNESS.embedded_setups.len(),
            on_disk.len()
        );

        for (relative, bytes) in HARNESS.embedded_setups {
            assert!(
                !relative.contains('\\') && !relative.starts_with('/'),
                "{relative:?} is not a relative slash path; a key built with the \
                 platform separator reads correctly on Unix and finds nothing on Windows"
            );
            let path = root.join(relative);
            let found = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{relative} is compiled in but not in the tree: {e}"));
            assert_eq!(
                &found, bytes,
                "{relative} differs between the binary and the tree"
            );
        }
    }

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = ANTIGRAVITY.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "antigravity");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in ANTIGRAVITY.never_touch {
            assert!(
                !ANTIGRAVITY.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    /// Everything this harness claims to own, against the vendor page that
    /// decided it.
    ///
    /// What this replaced only checked that the baseline parsed. The block it
    /// reads now is hand-authored beside the rest of the baseline, and this is
    /// what keeps that block from being decoration: a namespace no vendor
    /// document names, or a declared kind no owned surface routes, is red here.
    ///
    /// Both directions, because the defect it was written for ran both ways --
    /// `~/.cursor/rules` was owned and does not exist, `~/.pi/agent/prompts`
    /// exists and was not owned. Conformance caught neither: its
    /// `declared_native_route_is_compilable` case asks for **one** route, not
    /// every one.
    #[test]
    fn every_surface_this_harness_owns_is_one_the_vendor_documents() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let problems = harness_runtime::surfaces::disagreements(&HARNESS, &baseline);
        assert!(
            problems.is_empty(),
            "the declaration and {TOOL}-baseline.json disagree:
  {}",
            problems.join(
                "
  "
            )
        );
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(ANTIGRAVITY.control_directory.contains("setup-system"));
        assert!(ANTIGRAVITY.state_file.starts_with("NDDEV-"));
        assert!(
            !ANTIGRAVITY
                .native_namespaces
                .contains(&ANTIGRAVITY.state_file)
        );
    }

    #[test]
    fn nothing_at_the_root_of_the_shared_home_is_claimed() {
        // The whole reason this harness is safe to point at `~/.gemini` is that
        // every namespace names a subdirectory. A root-level entry would claim a
        // file belonging to a product this provider knows nothing about, and
        // `remove` would then delete it.
        for namespace in ANTIGRAVITY.native_namespaces {
            assert!(
                namespace.contains('/'),
                "{namespace} sits at the root of a home shared with Gemini CLI"
            );
        }
    }

    #[test]
    fn the_two_settings_documents_are_told_apart_by_path_not_by_name() {
        // `settings.json` is Gemini CLI's; `antigravity-cli/settings.json` is
        // this product's. They share a basename, so only the prefix separates
        // them -- and this asserts the separation still exists.
        assert!(ANTIGRAVITY.never_touch.contains(&"settings.json"));
        assert!(
            ANTIGRAVITY
                .native_namespaces
                .contains(&"antigravity-cli/settings.json")
        );
    }
    /// A setup that writes a configuration file says where its format came from.
    ///
    /// The release before this one made the *surfaces* sourced: a path this
    /// provider owns cites the page that documents it. This is the same rule
    /// one level down, and it was written because two of the seven failed it.
    ///
    /// opencode's baseline set `"permission": "ask"` where the product
    /// documents an object of tool names, and antigravity's set
    /// `toolPermissions` where the product reads `toolPermission` with four
    /// values, none of them the one written. Both were valid JSON in the right
    /// file at the right path. Both installed, verified and restored cleanly.
    /// Neither changed anything about the product — a target that looks
    /// configured and is not, which is the failure this estate refuses one
    /// level up and had been shipping one level down.
    /// Two files in one setup that a case-insensitive filesystem would merge.
    ///
    /// macOS and Windows fold case, so such a pair is one file there and two on
    /// Linux -- the setup would install different content depending on the
    /// machine, and its catalogue digest would differ per platform. The bundle
    /// reader has refused this for an arriving bundle since 0.0.11; this is the
    /// same rule applied to what this repository authors.
    /// Every component entry point describes itself.
    ///
    /// A `SKILL.md` or an agent whose frontmatter lost its `description` still
    /// installs, verifies and restores cleanly -- and the product names it after
    /// its directory and gives the model nothing to choose on. Documents under
    /// `references/` and files under `commands/` are exempt, because the
    /// products measured do not read frontmatter from either.
    /// Supporting documents are reachable from an entry point.
    ///
    /// A `references/` folder whose skill has no `SKILL.md` is prose nothing
    /// routes to. A generator in this repository produced exactly that, and
    /// every other guard passed it: the files are documents, so `unsourced`
    /// exempts them, and there is no `SKILL.md`, so `undescribed` has nothing
    /// to check.
    /// Nothing shipped sends a reader to a file this setup does not carry.
    ///
    /// A routing table naming `references/surfaces.md` in a setup that ships no
    /// such file sends the reader nowhere -- and the reader is a model, which
    /// will not say so. The generator here did exactly that: it pointed every
    /// harness's agent at that path, and codex ships no skill at all.
    #[test]
    fn nothing_shipped_names_a_document_it_does_not_carry() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::dangling_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn every_reference_folder_has_an_entry_point() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unreachable_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// Nothing inside a skill is a file no reader is sent to.
    ///
    /// Two findings in one hour were of exactly this shape and every guard in
    /// this estate was silent on both: an executable validator shipped into
    /// people's homes that nothing named, and eleven authoring pages written
    /// into four harnesses and routed to from none. The estate asked whether a
    /// *named* file exists and never whether an *existing* file is named.
    #[test]
    fn nothing_inside_a_skill_is_stranded() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let found = harness_runtime::catalog::stranded(
            &harness_runtime::Catalog::at(&root).list().unwrap(),
        );
        assert!(found.problems.is_empty(), "{}", found.problems.join("\n  "));
        // antigravity carries 12 file(s) inside its skill. Stated so that a layout change emptying the skill fails here rather than passing a guard with nothing left to walk.
        assert_eq!(
            found.entry_points, 12,
            "the stranded-file guard walked {} files inside skills, not 12",
            found.entry_points
        );
    }

    #[test]
    fn every_component_entry_point_describes_itself() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let examined = harness_runtime::catalog::undescribed(&catalog.list().unwrap());
        assert!(
            examined.problems.is_empty(),
            "{}",
            examined.problems.join("\n  ")
        );
        // antigravity ships 1 entry point(s) across its four postures. Stated so that a layout change removing them fails here rather than passing a guard with nothing left to check.
        assert_eq!(
            examined.entry_points, 1,
            "the description guard examined {} entry points, not 1",
            examined.entry_points
        );
    }

    #[test]
    fn no_two_files_in_a_setup_differ_only_in_case() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::colliding(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn a_setup_that_writes_configuration_says_where_its_format_came_from() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unsourced(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Three postures, on every one of the seven.
    ///
    /// `baseline` is a working floor, `minimal` is the product's own defaults,
    /// and `full-auto` asks nothing and sandboxes nothing. A caller who learns
    /// them on one product knows them on all seven, which is the whole reason
    /// the names are the estate's rather than each harness's.
    ///
    /// The second half of the check is the one worth having: two setups with
    /// the same bytes mean one of them is a posture in name only, and it would
    /// still read as offered in `list`.
    #[test]
    fn the_three_postures_are_offered_and_are_actually_different() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::asymmetric(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Nothing this setup ships tells a reader to run something that is not here.
    ///
    /// A setup carries documents an agent reads and acts on -- a skill, a rule,
    /// a command file -- and nothing was checking them. One shipped
    /// `software-status --target <dir> --json` and `list --json` for six
    /// releases; the binary refuses both, and says so in those words.
    ///
    /// Two refusals: a name belonging to the frozen estate, and any line naming
    /// this provider followed by a verb `into_command` does not accept. English
    /// is not judged -- `install` in a sentence is a word, and only
    /// `<provider> install` is an instruction.
    #[test]
    fn nothing_this_harness_ships_names_a_command_it_refuses() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems =
            harness_runtime::catalog::misdirecting(HARNESS.provider_id, &catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
}
