//! Antigravity's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/antigravity-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts agy is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.25-6680093607723008/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 53_226_078,
        sha256: "sha256:063063128d62f7fa8ff8a01f9629744cb48ec1edcb2c57112d89a79fe48479f7",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.25-6680093607723008/linux-x64/cli_linux_x64.tar.gz",
        bytes: 56_770_237,
        sha256: "sha256:45ab4a99884de17af76565a4ff8d9762d6e960067bd008fde9b050ec8fc9e421",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.25-6680093607723008/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 50_349_479,
        sha256: "sha256:8dab3e113726a3dfed69cbbf6757b4e491ae3b53aab0fdf17a56914d45281eff",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.25-6680093607723008/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 55_217_782,
        sha256: "sha256:e5c21bd67723c5a32e6c557e1406a219c769309e7c77746e2ed801c9bf496f66",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.25-6680093607723008/windows-arm/cli_windows_arm64.exe",
        bytes: 178_392_728,
        sha256: "sha256:b7e664057e80858e19c8f5fb8d64836cd2b1ccff7105e91ddedd30d908ebaf1a",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.25-6680093607723008/windows-x64/cli_windows_x64.exe",
        bytes: 188_758_168,
        sha256: "sha256:dbc665f942b59e56a0d3317aa01b93acc9521bdaa76277b922d82ef90eba2b3c",
        shape: Shape::Raw,
        member: "",
    },
];

/// The artifacts 1.1.24 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.24-6130423206641664/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 53_158_592,
        sha256: "sha256:e601bde6dcb9caebf8a83de235a0e10a21192c7f9ca9ed459a120714e7d42399",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.24-6130423206641664/linux-x64/cli_linux_x64.tar.gz",
        bytes: 56_692_103,
        sha256: "sha256:cff1fb7ed735da72c35658645a4f916cf74f020d4cd30ab95ebe8c2a49a4d569",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.24-6130423206641664/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 50_284_627,
        sha256: "sha256:189af288ed9527f567ab3a53b35a6da2fc0c3812c6245f266c75a2a3604bdec3",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.24-6130423206641664/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 55_138_078,
        sha256: "sha256:cd05d272ab2e4b97c2129a3e9c3fb6a76cebebb938a46930c7aa265df841cdb5",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.24-6130423206641664/windows-arm/cli_windows_arm64.exe",
        bytes: 177_259_672,
        sha256: "sha256:c715405df9c7186cb10104916f6587c5c50f330be1626a49f20c348162a7e41f",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.24-6130423206641664/windows-x64/cli_windows_x64.exe",
        bytes: 187_601_560,
        sha256: "sha256:7585871b1a34f9acc7f9c065f09e5bd1a7009519f0f219a4c43bb565c7880c95",
        shape: Shape::Raw,
        member: "",
    },
];

/// Antigravity's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.1.25",
    command: "agy",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.1.24",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/antigravity-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
