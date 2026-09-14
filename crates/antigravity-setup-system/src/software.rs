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
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.2-6061403484848128/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 54_016_481,
        sha256: "sha256:00fc5cb50cd714b81cdf1298fcc90e63a59e3e564a4f8076454ab8457d36e6eb",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.2-6061403484848128/linux-x64/cli_linux_x64.tar.gz",
        bytes: 57_596_853,
        sha256: "sha256:2cfa5c9a4a1edd96db6d4058f34970be60d3bcacda866e2bdce6aefb2451b48e",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.2-6061403484848128/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 49_929_659,
        sha256: "sha256:f90ff6094a196f1be3854ac45d999a542d47ec66a1513aa882a8505b947b9a0f",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.2-6061403484848128/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 54_877_358,
        sha256: "sha256:2b44ed726a73a0ef0a39956c80e07b5898c0b4fe21f8ecd383be74f1c0b65e5d",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.2-6061403484848128/windows-arm/cli_windows_arm64.exe",
        bytes: 182_498_456,
        sha256: "sha256:fe589dfba43ea957c3cb4db1b919b9436843ab413c08ac0439367eb03586161c",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.2-6061403484848128/windows-x64/cli_windows_x64.exe",
        bytes: 193_006_232,
        sha256: "sha256:80a029a8f22dccc123fd39453bd4d51c93d6384d4f365993a3d5accfa4af4b47",
        shape: Shape::Raw,
        member: "",
    },
];

/// The artifacts 1.1.27 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.27-5211191891591168/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 53_241_342,
        sha256: "sha256:97fc9fe5a6067406cd02cbe4ae6e362c9623a24d33bec486911246c17ceb6a94",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.27-5211191891591168/linux-x64/cli_linux_x64.tar.gz",
        bytes: 56_789_301,
        sha256: "sha256:f874d4f6b8a73c2df660f580f25fb656fcb6e64adbfd746e6692e837fd9a20be",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.27-5211191891591168/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 49_187_973,
        sha256: "sha256:e901e5c8fd20ab4c21c01df306030079286d08e6d372cdb535d5ccc7a3f565f4",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.27-5211191891591168/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 54_061_138,
        sha256: "sha256:fe5f102bf65be5d478c68fc2f9c3f5b3c5cb2cc788105310b35698fd1af10192",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.27-5211191891591168/windows-arm/cli_windows_arm64.exe",
        bytes: 179_063_448,
        sha256: "sha256:88fa8c65d814f0b8dee779f6b97318131a2dbd255b9fc902c712e30b6cbef44f",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.1.27-5211191891591168/windows-x64/cli_windows_x64.exe",
        bytes: 189_485_208,
        sha256: "sha256:d3bae6895069231c169f427c99527d1f556951e9c43488f45e8b040fbf3011ed",
        shape: Shape::Raw,
        member: "",
    },
];

/// Antigravity's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.2.2",
    command: "agy",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.1.27",
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
