use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::core::types::stringy::Stringy;

/// Struct representing the version information of both application and library.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Clone)]
pub struct SoftwareVersion {
    /// Version of the application.
    pub application: Version,
    /// Version of the library.
    pub library: Version,
}

impl SoftwareVersion {
    /// Creates a new `SoftwareVersion` instance from given version strings.
    pub fn new(application_version: &str, library_version: &str, channel: VersionCode) -> Self {
        Self {
            application: Version::new(application_version, channel.clone()),
            library: Version::new(library_version, channel),
        }
    }

    /// Creates a `SoftwareVersion` instance with dummy version data.
    pub fn dummy() -> Self {
        let dummy_version = "0.0.0";
        let dummy_channel = VersionCode::Alpha;
        Self {
            application: Version::new(dummy_version, dummy_channel.clone()),
            library: Version::new(dummy_version, dummy_channel),
        }
    }

    /// Compares the application and library versions with an incoming `SoftwareVersion`.
    //  This function is experimental and may change or be removed in the future.
    /// Use at your own risk.    
    pub fn compare_versions(&self, incoming: &SoftwareVersion) -> bool {
        let app_match = Version::compare_versions(&self.application, &incoming.application);
        let lib_match = Version::compare_versions(&self.library, &incoming.library);
        app_match && lib_match
    }
}

impl fmt::Display for SoftwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Application Version: {}, Library Version: {}",
            self.application, self.library
        )
    }
}

/// Struct representing version details.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Clone)]
pub struct Version {
    /// Version number as a string (e.g., "1.0.0").
    pub number: Stringy,
    /// Code representing the release channel (e.g., Beta, Production).
    pub code: VersionCode,
}

/// Enumeration representing different release channels or version codes.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Clone)]
pub enum VersionCode {
    /// Production release version.
    Production,
    /// Release candidate version.
    ReleaseCandidate,
    /// Beta version.
    Beta,
    /// Alpha version.
    Alpha,
    /// Patched version.
    /// using `Patched` will bypass compatibility checks. This is for hot fixes
    Patched, // If a quick patch is issued before the platform update, this code is used.
}

impl fmt::Display for VersionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code_str = match self {
            VersionCode::Production => "P",
            VersionCode::ReleaseCandidate => "RC",
            VersionCode::Beta => "b",
            VersionCode::Alpha => "a",
            VersionCode::Patched => "*",
        };
        write!(f, "{}", code_str.bold().red())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.number.bold().green(), self.code)
    }
}

impl Version {
    /// Creates a new `Version` instance with the provided version number and channel.
    pub fn new(version_number: &str, channel: VersionCode) -> Self {
        Version {
            number: version_number.into(),
            code: channel,
        }
    }

    /// Creates a binary code representation of the version given
    pub fn encode(&self) -> u16 {
        let version_numbers = Self::parse_version_parts(&self.number);

        if let Some(numbers) = version_numbers {
            let major = numbers.0;
            let minor = numbers.1;
            let patch = numbers.2;

            // Map VersionCode to its corresponding value.
            let code_value = match self.code {
                VersionCode::Production => 0,
                VersionCode::ReleaseCandidate => 1,
                VersionCode::Beta => 2,
                VersionCode::Alpha => 3,
                VersionCode::Patched => 4,
            };

            // Pack major, minor, patch, and code into a u16.
            (code_value & 0b111)               // 3 bits for code
            | ((major as u16 & 0b11111) << 3)   // 5 bits for major
            | ((minor as u16 & 0b1111) << 8)    // 4 bits for minor
            | ((patch as u16 & 0b1111) << 12) // 4 bits for patch
        } else {
            // parsing error
            0
        }
    }

    /// Decodes a u16 into a Version
    pub fn decode(encoded: u16) -> Self {
        let code_value = encoded & 0b111;
        let major = (encoded >> 3) & 0b11111;
        let minor = (encoded >> 8) & 0b1111;
        let patch = (encoded >> 12) & 0b1111;

        let code = match code_value {
            0 => VersionCode::Production,
            1 => VersionCode::ReleaseCandidate,
            2 => VersionCode::Beta,
            3 => VersionCode::Alpha,
            4 => VersionCode::Patched,
            _ => VersionCode::Patched,
        };

        // Construct the `number` string in the format "MAJOR.MINOR.PATCH".
        let number = format!("{}.{}.{}", major, minor, patch);

        Version {
            number: number.into(),
            code,
        }
    }

    /// Returns the version as a `Stringy`.
    pub fn get_as_string(&self) -> Stringy {
        Stringy::from(&self.to_string())
    }

    /// Checks if an incoming version is compatible with the current version.
    pub fn compare_versions(&self, incoming: &Version) -> bool {
        if self.code == VersionCode::Patched {
            return true;
        }
        if incoming.code == VersionCode::Patched {
            return true;
        }
        match (&incoming.code, &self.code) {
            (VersionCode::Alpha, VersionCode::Alpha) => true,
            (VersionCode::Beta, VersionCode::Beta)
            | (VersionCode::Beta, VersionCode::Alpha)
            | (VersionCode::Alpha, VersionCode::Beta) => true,
            (VersionCode::ReleaseCandidate, VersionCode::ReleaseCandidate)
            | (VersionCode::ReleaseCandidate, VersionCode::Beta)
            | (VersionCode::Beta, VersionCode::ReleaseCandidate) => {
                let (incoming_major, _, _) = Self::parse_version_parts(&incoming.number).unwrap();
                let (current_major, _, _) = Self::parse_version_parts(&self.number).unwrap();
                incoming_major == current_major
            }
            (VersionCode::Production, VersionCode::ReleaseCandidate)
            | (VersionCode::ReleaseCandidate, VersionCode::Production)
            | (VersionCode::Production, VersionCode::Production) => {
                let (incoming_major, incoming_minor, _) =
                    Self::parse_version_parts(&incoming.number).unwrap();
                let (current_major, current_minor, _) =
                    Self::parse_version_parts(&self.number).unwrap();
                incoming_major == current_major && incoming_minor == current_minor
            }
            _ => false,
        }
    }

    /// Converts the version into a string representation.
    pub fn to_string(&self) -> String {
        format!("{}{}", self.number, self.code)
    }

    /// Constructs a `Version` from a string representation.
    pub fn from_string(version_str: String) -> Option<Self> {
        let pos = version_str
            .chars()
            .position(|c| !c.is_digit(10) && c != '.');
        if let Some(pos) = pos {
            let number_part = &version_str[..pos];
            let code_part = &version_str[pos..];
            let code = match code_part {
                "P" => VersionCode::Production,
                "RC" => VersionCode::ReleaseCandidate,
                "b" => VersionCode::Beta,
                "a" => VersionCode::Alpha,
                "*" => VersionCode::Patched,
                _ => return None,
            };
            Some(Version {
                number: Stringy::from(number_part),
                code,
            })
        } else {
            None
        }
    }

    /// Constructs a `Version` from a `Stringy` representation.
    pub fn from_stringy(version_str: Stringy) -> Option<Self> {
        Self::from_string(version_str.to_string())
    }

    /// Parses a version string into major and minor components.
    fn parse_version_parts(version: &str) -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major: u32 = parts[0].parse().ok()?;
        let minor: u32 = parts[1].parse().ok()?;
        let patch: u32 = parts[2].parse().ok()?;
        Some((major, minor, patch))
    }
}
