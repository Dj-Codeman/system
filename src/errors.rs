use pretty::warn;
use std::{
    collections,
    convert::Infallible,
    fmt, io, net,
    ops::FromResidual,
    path,
    sync::{self, Arc, RwLock},
    thread, time,
};

/// Represents different types of generic errors.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Errors {
    OpeningFile,
    ReadingFile,
    CreatingFile,
    CreatingDirectory,
    DeletingDirectory,
    DeletingFile,
    SettingPermissionsDirectory,
    SettingPermissionsFile,
    UntaringFile,
    InputOutput,
    GeneralError,
    InitializationError,
    SecretArray,
    JsonCreation,
    JsonReading,
    InvalidType,
    InvalidChunkData,
    InvalidHMACData,
    InvalidHMACSize,
    InvalidKey,
    InvalidHexData,
    InvalidIvData,
    InvalidBlockData,
    InvalidAuthRequest,
    InvalidMapRequest,
    InvalidMapVersion,
    InvalidMapData,
    InvalidMapHash,
    InvalidBufferFit,
    InvalidUtf8Data,
    InvalidSignature,
    InvalidFile,
    PermissionDenied,
    NotFound,
    OutOfMemory,
    ConnectionError,
    Timeout,
    AuthenticationError,
    Unauthorized,
}

/// Represents a generic error.
#[derive(Debug)]
pub struct ErrorArrayItem {
    pub err_type: Errors,
    pub err_mesg: String,
}

impl ErrorArrayItem {
    /// Creates a new `ErrorArrayItem` instance.
    pub fn new(kind: Errors, message: String) -> Self {
        ErrorArrayItem {
            err_type: kind,
            err_mesg: message,
        }
    }
}

/// Represents a collection of warnings.
#[derive(Debug, Clone)]
pub struct WarningArray(pub Arc<RwLock<Vec<WarningArrayItem>>>);
/// Represents a collection of errors.

#[derive(Debug, Clone)]
pub struct ErrorArray(pub Arc<RwLock<Vec<ErrorArrayItem>>>);

/// Represents different types of generic warnings.
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub enum Warnings {
    Warning,
    OutdatedVersion,
    MisAlignedChunk,
    FileNotDeleted,
    ConnectionLost,
    ResourceExhaustion,
    UnexpectedBehavior,
    UnexpectedConfiguration,
}

/// Represents a generic warning.
#[derive(Debug)]
pub struct WarningArrayItem {
    pub warn_type: Warnings,
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

impl WarningArray {
    /// Creates a new `WarningArray` instance.
    pub fn new(mut data: Vec<WarningArrayItem>) -> Self {
        let warning_array: Vec<WarningArrayItem> = Vec::new();
        let warning = Self {
            0: Arc::new(RwLock::new(warning_array)),
        };

        let mut to_append = warning.0.write().unwrap();
        to_append.append(&mut data);
        drop(to_append);
        return warning;
    }

    /// Creates an empty `WarningArray` instance.
    pub fn new_container() -> Self {
        let warning_array: Vec<WarningArrayItem> = Vec::new();
        Self {
            0: Arc::new(RwLock::new(warning_array)),
        }
    }

    /// Displays the warnings.
    pub fn display(self) {
        let warning_array = self.0.read().unwrap();
        for warns in warning_array.as_slice() {
            warn(&format!("{}", warns))
        }
    }

    /// Pushes a new warning to the collection.
    pub fn push(&mut self, item: WarningArrayItem) {
        let mut warning_array = self.0.write().unwrap();
        warning_array.push(item);
        drop(warning_array)
    }
}

impl ErrorArray {
    /// Creates a new `Errors` instance.
    pub fn new(mut data: Vec<ErrorArrayItem>) -> Self {
        let error_array: Vec<ErrorArrayItem> = Vec::new();
        let error = Self {
            0: Arc::new(RwLock::new(error_array)),
        };

        let mut to_append = error.0.write().unwrap();
        to_append.append(&mut data);
        drop(to_append);
        return error;
    }

    /// Creates an empty `Errors` instance.
    pub fn new_container() -> Self {
        let error_array: Vec<ErrorArrayItem> = Vec::new();
        Self {
            0: Arc::new(RwLock::new(error_array)),
        }
    }

    /// Displays the errors.
    pub fn display(self) {
        let error_array = self.0.read().unwrap();
        for errors in error_array.as_slice() {
            warn(&format!("{}", errors))
        }
    }

    /// Pushes a new error to the collection.
    pub fn push(&mut self, item: ErrorArrayItem) {
        let mut error_array = self.0.write().unwrap();
        error_array.push(item);
    }
}

/// Represents a unified result that can contain data or errors.
#[derive(Debug)]
pub enum UnifiedResult<T> {
    ResultWarning(Result<OkWarning<T>, ErrorArray>),
    ResultNoWarns(Result<T, ErrorArray>),
}
// pub struct UnifiedResult<T>(Result<OkWarning<T>, ErrorArray>);

/// Represents a result that contains data and warnings.
#[derive(Debug)]
pub struct OkWarning<T> {
    pub data: T,
    pub warning: WarningArray,
}

impl<T> UnifiedResult<T> {
    /// Creates a new `UnifiedResult` instance.
    pub fn new_warn(result: Result<OkWarning<T>, ErrorArray>) -> Self {
        UnifiedResult::ResultWarning(result)
    }

    pub fn new(result: Result<T, ErrorArray>) -> Self {
        UnifiedResult::ResultNoWarns(result)
    }

    /// Resolves the `UnifiedResult` and returns the data if successful.
    pub fn unwrap(self) -> T {
        match self {
            UnifiedResult::ResultWarning(r) => match r {
                Ok(d) => {
                    d.warning.display();
                    return d.data;
                }
                Err(e) => {
                    e.display();
                    std::process::exit(1)
                }
            },
            UnifiedResult::ResultNoWarns(r) => match r {
                Ok(d) => return d,
                Err(e) => {
                    e.display();
                    std::process::exit(1)
                }
            },
        }
    }

    /// Unwraps the `UnifiedResult` and returns the data or errors.
    pub fn uf_unwrap(self) -> Result<T, ErrorArray> {
        match self {
            UnifiedResult::ResultWarning(r) => match r {
                Ok(d) => {
                    let warnings: WarningArray = d.warning;
                    let value: T = d.data;
                    warnings.display();
                    return Ok(value);
                }
                Err(e) => return Err(e),
            },
            UnifiedResult::ResultNoWarns(r) => return r,
        }
    }
}

// Implement FromResidual<Result<Infallible, UnifiedResult<_>>> for UnifiedResult
impl<T> FromResidual<Result<Infallible, UnifiedResult<T>>> for UnifiedResult<T> {
    fn from_residual(residual: Result<Infallible, UnifiedResult<T>>) -> Self {
        match residual.unwrap_err() {
            UnifiedResult::ResultWarning(_) => {
                // Since Infallible can never be constructed, this code path is unreachable
                unreachable!()
            }
            UnifiedResult::ResultNoWarns(_) => {
                // Since Infallible can never be constructed, this code path is unreachable
                unreachable!()
            }
        }
    }
}

// Pretty display for WarningArrayItem
impl fmt::Display for WarningArrayItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.warn_mesg {
            Some(d) => write!(f, "WarningArray: {:#?} - {}", self.warn_type, d),
            None => write!(f, "WarningArray"),
        }
    }
}

// Pretty display for ErrorArrayItem
impl fmt::Display for ErrorArrayItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "We encountered the following error: {:#?} - {}",
            self.err_type, self.err_mesg
        )
    }
}

// Conversion from std::io::Error to ErrorArrayItem
impl From<io::Error> for ErrorArrayItem {
    fn from(err: io::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::path::StripPrefixError to ErrorArrayItem
impl From<path::StripPrefixError> for ErrorArrayItem {
    fn from(err: path::StripPrefixError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::thread::AccessError to ErrorArrayItem
impl From<thread::AccessError> for ErrorArrayItem {
    fn from(err: thread::AccessError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::sync::mpsc::SendError<T> to ErrorArrayItem
impl<T> From<sync::mpsc::SendError<T>> for ErrorArrayItem {
    fn from(err: sync::mpsc::SendError<T>) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::net::AddrParseError to ErrorArrayItem
impl From<net::AddrParseError> for ErrorArrayItem {
    fn from(err: net::AddrParseError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::collections::TryReserveError to ErrorArrayItem
impl From<collections::TryReserveError> for ErrorArrayItem {
    fn from(err: collections::TryReserveError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::time::SystemTimeError to ErrorArrayItem
impl From<time::SystemTimeError> for ErrorArrayItem {
    fn from(err: time::SystemTimeError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from regex::Error to ErrorArrayItem
impl TryFrom<regex::Error> for ErrorArrayItem {
    type Error = ();

    fn try_from(err: regex::Error) -> Result<Self, Self::Error> {
        Ok(ErrorArrayItem::new(Errors::InputOutput, err.to_string()))
    }
}

// Conversion from serde_json::Error to ErrorArrayItem
impl From<serde_json::Error> for ErrorArrayItem {
    fn from(err: serde_json::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from serde_yaml::Error to ErrorArrayItem
impl From<serde_yaml::Error> for ErrorArrayItem {
    fn from(err: serde_yaml::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from reqwest::Error to ErrorArrayItem
impl From<reqwest::Error> for ErrorArrayItem {
    fn from(err: reqwest::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from rand::Error to ErrorArrayItem
impl From<rand::Error> for ErrorArrayItem {
    fn from(err: rand::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from walkdir::Error to ErrorArrayItem
impl From<walkdir::Error> for ErrorArrayItem {
    fn from(err: walkdir::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}
