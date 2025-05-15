use serde::{Serialize, Deserialize};
use crate::core::errors::Warnings;

/// Represents a generic warning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarningArrayItem {
    /// Type of the warning.
    pub warn_type: Warnings,
    /// Optional message associated with the warning.
    pub warn_mesg: Option<String>,
}

impl WarningArrayItem {
    /// Creates a new `WarningArrayItem` instance.
    pub fn new(kind: Warnings) -> Self {
        WarningArrayItem {
            warn_type: kind,
            warn_mesg: None,
        }
    }

    /// Creates a new `WarningArrayItem` instance with details.
    pub fn new_details(kind: Warnings, message: String) -> Self {
        WarningArrayItem {
            warn_type: kind,
            warn_mesg: Some(message),
        }
    }
}