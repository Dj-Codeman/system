#[cfg(test)]
mod tests {
    use crate::version::{SoftwareVersion, Version, VersionCode};

    #[test]
    fn test_version_creation() {
        let version = Version::new("1.2.3", VersionCode::Beta);
        assert_eq!(version.number.to_string(), "1.2.3");
        assert_eq!(version.code, VersionCode::Beta);
    }

    #[test]
    fn test_version_comparison_equal_versions() {
        let version1 = Version::new("1.2.3", VersionCode::Production);
        let version2 = Version::new("1.2.3", VersionCode::Production);
        assert!(Version::compare_versions(&version1, &version2));
    }

    #[test]
    fn test_version_comparison_different_channels() {
        let version1 = Version::new("1.2.3", VersionCode::Beta);
        let version2 = Version::new("1.2.3", VersionCode::Alpha);
        assert!(Version::compare_versions(&version1, &version2));

        let version3 = Version::new("1.2.3", VersionCode::Production);
        assert!(!Version::compare_versions(&version1, &version3));
    }

    #[test]
    fn test_version_comparison_release_candidate() {
        let version1 = Version::new("1.2.3", VersionCode::ReleaseCandidate);
        let version2 = Version::new("1.2.4", VersionCode::ReleaseCandidate);
        assert!(Version::compare_versions(&version1, &version2));

        let version3 = Version::new("1.2.3", VersionCode::Beta);
        assert!(Version::compare_versions(&version1, &version3));
    }

    #[test]
    fn test_version_from_string() {
        let version_str = "1.2.3b";
        let version = Version::from_string(version_str.to_string()).unwrap();
        assert_eq!(version.number.to_string(), "1.2.3");
        assert_eq!(version.code, VersionCode::Beta);
    }

    #[test]
    fn test_version_from_string_invalid() {
        let version_str = "1.2.3x"; // Invalid code
        let version = Version::from_string(version_str.to_string());
        assert!(version.is_none());
    }

    #[test]
    fn test_software_version_comparison() {
        let app_version1 = Version::new("1.2.3", VersionCode::Production);
        let lib_version1 = Version::new("2.3.4", VersionCode::Beta);
        let software_version1 = SoftwareVersion {
            application: app_version1.clone(),
            library: lib_version1.clone(),
        };

        let app_version2 = Version::new("1.2.3", VersionCode::Production);
        let lib_version2 = Version::new("2.3.4", VersionCode::Beta);
        let software_version2 = SoftwareVersion {
            application: app_version2.clone(),
            library: lib_version2.clone(),
        };

        assert!(software_version1.compare_versions(&software_version2));
    }

    #[cfg(test)]
    #[test]
    fn test_encode_basic() {
        let version = Version {
            number: "1.2.3".into(),
            code: VersionCode::Beta,
        };
        let encoded = version.encode();
        assert_eq!(encoded, 0b0011_0010_00001_010); // 3 for Beta, 1 for major, 2 for minor, 3 for patch
    }

    #[test]
    fn test_encode_edge_cases() {
        // Maximum possible values within the limits
        let version = Version {
            number: "31.15.15".into(),
            code: VersionCode::Alpha,
        };
        let encoded = version.encode();
        assert_eq!(encoded, 0b1111_1111_11111_011); // 3 bits for Alpha, max values for major, minor, patch
    }

    #[test]
    fn test_encode_invalid_version_string() {
        let version = Version {
            number: "not.a.version".into(),
            code: VersionCode::Production,
        };
        let encoded = version.encode();
        assert_eq!(encoded, 0); // Invalid string should result in 0
    }

    #[test]
    fn test_decode_basic() {
        let encoded = 0b0011_0010_00001_010; // Beta, 1 for major, 2 for minor, 3 for patch
        let decoded = Version::decode(encoded);
        assert_eq!(decoded.number.as_str(), "1.2.3");
        assert_eq!(decoded.code, VersionCode::Beta);
    }

    #[test]
    fn test_decode_edge_cases() {
        let encoded = 0b1111_1111_11111_011; // Alpha, max values for major, minor, patch
        let decoded = Version::decode(encoded);
        assert_eq!(decoded.number.as_str(), "31.15.15");
        assert_eq!(decoded.code, VersionCode::Alpha);
    }

    #[test]
    fn test_round_trip_encode_decode() {
        let version = Version {
            number: "5.10.7".into(),
            code: VersionCode::ReleaseCandidate,
        };
        let encoded = version.encode();
        let decoded = Version::decode(encoded);
        assert_eq!(decoded.number, version.number);
        assert_eq!(decoded.code, version.code);
    }

    #[test]
    fn test_decode_invalid_encoded_value() {
        let encoded = 0b0000_0000_00000_111; // Invalid code (not mapped to any VersionCode)
        let decoded = Version::decode(encoded);
        assert_eq!(decoded.code, VersionCode::Patched); // Default fallback
        assert_eq!(decoded.number.as_str(), "0.0.0"); // Default values for major, minor, patch
    }
}
