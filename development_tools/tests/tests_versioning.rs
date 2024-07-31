use development_tools::find_max_matching_version;
use semver::{BuildMetadata, Prerelease, Version, VersionReq};
use std::process::Command;

#[cfg(test)]
mod tests_versioning {

    use super::*;

    #[test]
    fn test_parse_and_increment_version() {
        let mut parsed_version = Version::parse("0.2.6").unwrap();

        assert_eq!(
            parsed_version,
            Version {
                major: 0,
                minor: 2,
                patch: 6,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            }
        );

        parsed_version.patch += 1;
        assert_eq!(parsed_version.to_string(), "0.2.7");
        println!("New patch release: v{}", parsed_version);

        parsed_version.minor += 1;
        parsed_version.patch = 0;
        assert_eq!(parsed_version.to_string(), "0.3.0");
        println!("New minor release: v{}", parsed_version);

        parsed_version.major += 1;
        parsed_version.minor = 0;
        assert_eq!(parsed_version.to_string(), "1.0.0");
        println!("New major release: v{}", parsed_version);
    }

    #[test]
    fn test_parse_complex_version() {
        let version_str = "1.0.49-125+g72ee7853";
        let parsed_version = Version::parse(version_str).unwrap();
        let pre = Prerelease::new("125").unwrap();
        let build = BuildMetadata::new("g72ee7853").unwrap();

        assert_eq!(
            parsed_version,
            Version {
                major: 1,
                minor: 0,
                patch: 49,
                pre,
                build
            }
        );

        assert_eq!(
            parsed_version.build,
            BuildMetadata::new("g72ee7853").unwrap()
        );

        let serialized_version = parsed_version.to_string();
        assert_eq!(&serialized_version, version_str);
    }

    #[test]
    fn test_version_is_prerelease() {
        let version_1 = Version::parse("1.0.0-alpha").unwrap();
        let version_2 = Version::parse("1.0.0").unwrap();

        assert!(!version_1.pre.is_empty());
        assert!(version_2.pre.is_empty());
    }

    #[test]
    fn test_find_max_matching_version() {
        assert_eq!(
            find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"]).unwrap(),
            Some(Version::parse("1.0.0").unwrap())
        );

        assert_eq!(
            find_max_matching_version(
                ">1.2.3-alpha.3",
                vec![
                    "1.2.3-alpha.3",
                    "1.2.3-alpha.4",
                    "1.2.3-alpha.10",
                    "1.2.3-beta.4",
                    "3.4.5-alpha.9",
                ]
            )
            .unwrap(),
            Some(Version::parse("1.2.3-beta.4").unwrap())
        );
    }

    #[test]
    fn test_check_external_command_version_for_compatibility() {
        let version_constraint = ">=1.50.0";
        let version_test = VersionReq::parse(version_constraint).unwrap();
        let output = Command::new("rustc").arg("--version").output().unwrap();

        assert_eq!(output.status.success(), true);

        let version_str = String::from_utf8(output.stdout).unwrap();
        let version = version_str.split_whitespace().nth(1).unwrap();
        let parsed_version = Version::parse(version).unwrap();

        assert!(version_test.matches(&parsed_version));
    }
}
