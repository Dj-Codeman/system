use serde::{Serialize, Deserialize};
use crate::core::errors::Errors;
use crate::core::types::stringy::Stringy;

/// Represents a generic error.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct ErrorArrayItem {
    /// Type of the error.
    pub err_type: Errors,
    /// Message associated with the error.
    pub err_mesg: Stringy,
}

impl ErrorArrayItem {
    /// Creates a new `ErrorArrayItem` instance.
    pub fn new<M>(kind: Errors, message: M) -> Self
    where
        M: Into<String>,
    {
        ErrorArrayItem {
            err_type: kind,
            err_mesg: Stringy::from(message),
        }
    }
}