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
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.7-6731160148115456/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 58_081_202,
        sha256: "sha256:8ddbb669158de1d1bc4c1fe5c130dca8f51da80d62569a54a4133f06768a723b",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.7-6731160148115456/linux-x64/cli_linux_x64.tar.gz",
        bytes: 61_763_170,
        sha256: "sha256:e410dd56d8c213ef12643d3ff5eaaab57a17e05bbf72e9415322f23879fc4a18",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.7-6731160148115456/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 53_945_901,
        sha256: "sha256:ce9fe3f4d6f44a2b1c83b334fc5c8f2975079959e24dd805e10eb49ab8c76a7e",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.7-6731160148115456/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 59_079_923,
        sha256: "sha256:2f1a82f55201fc47987b448e34d90608bb0eec48e4c45f0a27ba49625d984cd5",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.7-6731160148115456/windows-arm/cli_windows_arm64.exe",
        bytes: 192_598_168,
        sha256: "sha256:8e6730c4011031a51eec47c354826051721ea132561d961937d943b0b8d7325a",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.7-6731160148115456/windows-x64/cli_windows_x64.exe",
        bytes: 203_523_736,
        sha256: "sha256:162607893eaacaf7b4a34bcd0bc3978342c6707b0340f96040f0139ac904dd22",
        shape: Shape::Raw,
        member: "",
    },
];

/// The artifacts 1.2.4 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.4-6085322963025920/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 54_547_529,
        sha256: "sha256:9dee8d8de3ebf420525902ead1b4ca8d12b93731bfae3673f6f8a24131ac5e5c",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.4-6085322963025920/linux-x64/cli_linux_x64.tar.gz",
        bytes: 58_151_273,
        sha256: "sha256:dcd3e4d8c8afb1902d59c1ae52812458d2ddab67a5d2db44810c512910d918fe",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.4-6085322963025920/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 50_449_780,
        sha256: "sha256:f59c12c289e74bbb48178f827702c6224bd0aa920914319f85b42760dfc72f4c",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.4-6085322963025920/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 55_445_606,
        sha256: "sha256:ed5d05f8175d05b1a3ba19e06552698beb59f0eb9c9e76a7eca092f5a8f7d07e",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.4-6085322963025920/windows-arm/cli_windows_arm64.exe",
        bytes: 185_867_416,
        sha256: "sha256:a0be9fa5ebc04dfaba30e8d5b733da478480674524bed1a4dc3f8500a25e608d",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.4-6085322963025920/windows-x64/cli_windows_x64.exe",
        bytes: 196_453_528,
        sha256: "sha256:05cdf2444b1bc9ee278386756b2cf2afbba94e822f21580278e07e206c3125b8",
        shape: Shape::Raw,
        member: "",
    },
];

/// Antigravity's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.2.7",
    command: "agy",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.2.4",
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
