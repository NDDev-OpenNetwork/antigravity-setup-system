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
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.15-5434575321694208/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 57_506_222,
        sha256: "sha256:851bdabda3b2eb679d0d46b469629b91752d1f1d7497185107408b60c58c2e32",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.15-5434575321694208/linux-x64/cli_linux_x64.tar.gz",
        bytes: 61_200_461,
        sha256: "sha256:bbd4a4b29f0e9fe1fc2e1345b5d44fa08540e43014da46bc2c4bf70cf05745d8",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.15-5434575321694208/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 53_696_937,
        sha256: "sha256:66f7e9e8750a506e8a2caaedaadf479f023820f712015c9c55cfb91a2891521b",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.15-5434575321694208/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 58_831_003,
        sha256: "sha256:5602d71a3afc16ee07fcd0bd806842342464f78ed5fc9fea69adffd1a9e2a64d",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.15-5434575321694208/windows-arm/cli_windows_arm64.exe",
        bytes: 177_315_992,
        sha256: "sha256:662c747d6580344ab18722c2700a9bfd669260acc89286ae3d33022a53d87ba3",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.15-5434575321694208/windows-x64/cli_windows_x64.exe",
        bytes: 188_111_512,
        sha256: "sha256:f2ca122c2b33d35d9a70332d820ec8d24e5481baebd80fe228f1ffc305646bd3",
        shape: Shape::Raw,
        member: "",
    },
];

/// The artifacts 1.2.14 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.14-4571742832820224/linux-arm/cli_linux_arm64.tar.gz",
        bytes: 57_229_161,
        sha256: "sha256:3b40c3baab245b43a41007c1db64df51f5f162b6059fcc4a4504731f2689d301",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.14-4571742832820224/linux-x64/cli_linux_x64.tar.gz",
        bytes: 60_882_033,
        sha256: "sha256:68cf4d221cb62e0289245439d3d37f599bdc8e0c4e1e3dae03f326463a0c26dc",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.14-4571742832820224/darwin-arm/cli_mac_arm64.tar.gz",
        bytes: 53_444_121,
        sha256: "sha256:468edcc454b6bb1c321d8d42591a16ace4d1a1d628a4f1ce95ad236c9ee4cc19",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.14-4571742832820224/darwin-x64/cli_mac_x64.tar.gz",
        bytes: 58_558_502,
        sha256: "sha256:39364cc24e7b2b05a4a0138c60d5d5da39df9fb76dedc9e3f4b74a168512fcd6",
        shape: Shape::GzipTar,
        member: "antigravity",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.14-4571742832820224/windows-arm/cli_windows_arm64.exe",
        bytes: 189_929_112,
        sha256: "sha256:bd52ec70f66306b4e5d5f56ea554087d1d3706cbd51f2cfa4654931b456f0889",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://storage.googleapis.com/antigravity-public/antigravity-cli/1.2.14-4571742832820224/windows-x64/cli_windows_x64.exe",
        bytes: 200_623_256,
        sha256: "sha256:07f7ed55654b7066886c7390d3f5e245dbe4ede425c005909342cfc90333f2f8",
        shape: Shape::Raw,
        member: "",
    },
];

/// Antigravity's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.2.15",
    command: "agy",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.2.14",
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
