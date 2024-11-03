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
}
