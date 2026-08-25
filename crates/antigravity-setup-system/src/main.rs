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

use harness_runtime::Harness;
use provider_v3::{ComponentKind, ProjectionKind};

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
    config_home_env: "",
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
    ],
    // Gemini CLI's own, not this product's. Never read, never written, and
    // never copied into a backup slot: a backup of someone else's credentials
    // is a leak with a schedule.
    never_touch: &[
        "settings.json",
        "oauth_creds.json",
        "google_accounts.json",
        "tmp",
    ],
    permission_profiles: &["default"],
    // No instruction and no command: the product documents both only at
    // workspace scope, under a project's `.agents/`, and this provider
    // configures a home rather than a checkout. Declaring either would offer a
    // route that resolves nowhere.
    component_kinds: &[
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Hook,
        ComponentKind::Mcp,
        ComponentKind::Plugin,
        ComponentKind::Setting,
    ],
    projection_kinds: &[ProjectionKind::NativeFiles, ProjectionKind::Plugin],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    software: Some(software::SOFTWARE),
};

fn main() -> ExitCode {
    harness_runtime::run(&ANTIGRAVITY, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

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

    #[test]
    fn the_baseline_this_harness_cites_is_present_and_readable() {
        // The facts above are transcribed from it; a build whose baseline is
        // missing has no evidence for what it claims to own.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/antigravity-baseline.json");
        let value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert!(value.is_object());
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
}
