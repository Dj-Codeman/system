use hex::FromHexError;
use nix::errno::Errno;
use pretty::warn;
use std::{
    collections, fmt, io, net, num::ParseIntError, path, string::FromUtf8Error, sync::{self, Arc, RwLock}, thread, time
};

#[cfg(rust_comp_feature = "try_trait_v2")]
use std::{convert::Infallible, ops::FromResidual};

/// Represents different types of generic errors.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Errors {
    /// Error encountered while opening a file.
    OpeningFile,
    /// Error encountered while reading from a file.
    ReadingFile,
    /// Error encountered while creating a file.
    CreatingFile,
    /// Error encountered while creating a directory.
    CreatingDirectory,
    /// Error encountered while deleting a directory.
    DeletingDirectory,
    /// Error encountered while deleting a file.
    DeletingFile,
    /// Error encountered while setting permissions on a directory.
    SettingPermissionsDirectory,
    /// Error encountered while setting permissions on a file.
    SettingPermissionsFile,
    /// Error encountered while untaring a file.
    UntaringFile,
    /// General input/output error.
    InputOutput,
    /// General error.
    GeneralError,
    /// Initialization error.
    InitializationError,
    /// Error related to a secret array.
    SecretArray,
    /// Error encountered while creating JSON data.
    JsonCreation,
    /// Error encountered while reading JSON data.
    JsonReading,
    /// Invalid data type.
    InvalidType,
    /// Invalid chunk data.
    InvalidChunkData,
    /// Invalid HMAC data.
    InvalidHMACData,
    /// Invalid HMAC size.
    InvalidHMACSize,
    /// Invalid encryption key.
    InvalidKey,
    /// Invalid hexadecimal data.
    InvalidHexData,
    /// Invalid initialization vector (IV) data.
    InvalidIvData,
    /// Invalid block data.
    InvalidBlockData,
    /// Invalid authentication request.
    InvalidAuthRequest,
    /// Invalid map request.
    InvalidMapRequest,
    /// Invalid map version.
    InvalidMapVersion,
    /// Invalid map data.
    InvalidMapData,
    /// Invalid map hash.
    InvalidMapHash,
    /// Invalid buffer fit.
    InvalidBufferFit,
    /// Invalid UTF-8 data.
    InvalidUtf8Data,
    /// Invalid signature.
    InvalidSignature,
    /// Invalid file.
    InvalidFile,
    /// Permission denied.
    PermissionDenied,
    /// Resource not found.
    NotFound,
    /// Out of memory.
    OutOfMemory,
    /// Connection error.
    ConnectionError,
    /// Timeout error.
    Timeout,
    /// Authentication error.
    AuthenticationError,
    /// Unauthorized access.
    Unauthorized,
}

/// Represents a generic error.
#[derive(Debug)]
pub struct ErrorArrayItem {
    /// Type of the error.
    pub err_type: Errors,
    /// Message associated with the error.
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
    /// Generic warning.
    Warning,
    /// Warning indicating an outdated version.
    OutdatedVersion,
    /// Warning indicating a misaligned chunk.
    MisAlignedChunk,
    /// Warning indicating failure to delete a file.
    FileNotDeleted,
    /// Warning indicating a lost connection.
    ConnectionLost,
    /// Warning indicating resource exhaustion.
    ResourceExhaustion,
    /// Warning indicating unexpected behavior.
    UnexpectedBehavior,
    /// Warning indicating unexpected configuration.
    UnexpectedConfiguration,
}

/// Represents a generic warning.
#[derive(Debug)]
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

    pub fn append(&mut self, arr: Self) {
        let mut warning_array = self.0.write().unwrap();
        let mut donor_array = arr.0.write().unwrap();
        warning_array.append(&mut donor_array);
        drop(donor_array);
    }

    pub fn len(&self) -> usize {
        let vec = self.0.read().unwrap(); // Lock the RwLock and get a read guard
        vec.len()
    }
}

impl ErrorArray {
    /// Creates a new `Errors` instance.
    pub fn new(mut data: Vec<ErrorArrayItem>) -> Self {
        let error_array: Vec<ErrorArrayItem> = Vec::with_capacity(2);
        let error: ErrorArray = Self {
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

    pub fn append(&mut self, arr: Self) {
        let mut error_array = self.0.write().unwrap();
        let mut donor_array = arr.0.write().unwrap();
        error_array.append(&mut donor_array);
        drop(donor_array);
    }

    pub fn len(&self) -> usize {
        let vec = self.0.read().unwrap(); // Lock the RwLock and get a read guard
        vec.len()
    }
}

/// Represents a unified result that can contain data or errors.
#[derive(Debug)]
pub enum UnifiedResult<T> {
    /// Result variant containing data and warnings.
    ResultWarning(Result<OkWarning<T>, ErrorArray>),
    /// Result variant containing data only.
    ResultNoWarns(Result<T, ErrorArray>),
}

/// Represents a result that contains data and warnings.
#[derive(Debug)]
pub struct OkWarning<T> {
    /// Data associated with the result.
    pub data: T,
    /// Warnings associated with the result.
    pub warning: WarningArray,
}

impl<T> UnifiedResult<T> {
    /// Creates a new `UnifiedResult` instance with warnings.
    pub fn new_warn(result: Result<OkWarning<T>, ErrorArray>) -> Self {
        UnifiedResult::ResultWarning(result)
    }

    /// Creates a new `UnifiedResult` instance without warnings.
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

    /// Determines if the value in UnifiedResult is Ok()
    pub const fn is_ok(&self) -> bool {
        match &self {
            UnifiedResult::ResultWarning(d) => matches!(d, Ok(_)),
            UnifiedResult::ResultNoWarns(d) => matches!(d, Ok(_)),
        }
    }
}

#[cfg(rust_comp_feature = "try_trait_v2")]
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
            Some(d) => write!(f, "Warning: {:#?} - {}", self.warn_type, d),
            None => write!(f, "Warning"),
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

// Conversion from Utf8Error::Error to ErrorArrayItem
impl From<FromUtf8Error> for ErrorArrayItem {
    fn from(value: FromUtf8Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from FromHexError::Error to ErrorArrayItem
impl From<FromHexError> for ErrorArrayItem {
    fn from(value: FromHexError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from nix errors to ErrorArrayItem
impl From<Errno> for ErrorArrayItem {
    fn from(value: Errno) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from ParseIntError errors to ErrorArrayItem
impl From<ParseIntError> for ErrorArrayItem {
    fn from(value: ParseIntError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}