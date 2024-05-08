use std::{fmt, io};
use pretty::{output, warn};

/// Represents different types of generic errors.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GenErrors {
    ErrorOpeningFile,
    ErrorReadingFile,
    ErrorCreatingFile,
    ErrorCreatingDir,
    ErrorDeletingDir,
    ErrorDeletingFile,
    ErrorSettingPermDir,
    ErrorSettingPermFile,
    ErrorUntaringFile,
    ErrorInputOutput,
}

/// Represents a generic error.
#[derive(Debug, Clone)]
pub struct GenericError {
    pub err_type: GenErrors,
    pub err_mesg: Option<String>,
}

impl GenericError {
    /// Creates a new `GenericError` instance.
    pub fn new(kind: GenErrors) -> Self {
        GenericError {
            err_type: kind,
            err_mesg: None,
        }
    }

    /// Creates a new `GenericError` instance with details.
    pub fn new_details(kind: GenErrors, message: String) -> Self {
        GenericError {
            err_type: kind,
            err_mesg: Some(message),
        }
    }
}

/// Represents different types of generic warnings.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GenWarnings {
    Warning,
}

/// Represents a generic warning.
#[derive(Debug, Clone)]
pub struct GenericWarning {
    pub warn_type: GenWarnings,
    pub warn_mesg: Option<String>,
}

impl GenericWarning {
    /// Creates a new `GenericWarning` instance.
    pub fn new(kind: GenWarnings) -> Self {
        GenericWarning {
            warn_type: kind,
            warn_mesg: None,
        }
    }

    /// Creates a new `GenericWarning` instance with details.
    pub fn new_details(kind: GenWarnings, message: String) -> Self {
        GenericWarning {
            warn_type: kind,
            warn_mesg: Some(message),
        }
    }
}

/// Represents a collection of warnings.
#[derive(Debug, Clone)]
pub struct Warnings(pub Vec<GenericWarning>);

impl Warnings {
    /// Creates a new `Warnings` instance.
    pub fn new(data: Vec<GenericWarning>) -> Self {
        Self { 0: data }
    }

    /// Creates an empty `Warnings` instance.
    pub fn new_container() -> Self {
        Self { 0: Vec::new() }
    }

    /// Displays the warnings.
    pub fn display(self) {
        for warns in self.0 {
            warn(&format!("{}", warns))
        }
    }

    /// Pushes a new warning to the collection.
    pub fn push(&mut self, item: GenericWarning) {
        self.0.push(item)
    }
}

/// Represents a collection of errors.
#[derive(Debug, Clone)]
pub struct Errors(pub Vec<GenericError>);

impl Errors {
    /// Creates a new `Errors` instance.
    pub fn new(data: Vec<GenericError>) -> Self {
        Self { 0: data }
    }

    /// Creates an empty `Errors` instance.
    pub fn new_container() -> Self {
        Self { 0: Vec::new() }
    }

    /// Displays the errors.
    pub fn display(self) {
        for errs in self.0 {
            output("RED", &format!("{}", errs))
        }
    }

    /// Pushes a new error to the collection.
    pub fn push(&mut self, item: GenericError) {
        self.0.push(item)
    }
}

/// Represents a unified result that can contain data or errors.
#[derive(Debug, Clone)]
pub struct UnifiedResult<T>(Result<OkWarning<T>, Errors>);

/// Represents a result that contains data and warnings.
#[derive(Debug, Clone)]
pub struct OkWarning<T> {
    pub data: T,
    pub warning: Warnings,
}

impl<T> UnifiedResult<T> {
    /// Creates a new `UnifiedResult` instance.
    pub fn new(result: Result<OkWarning<T>, Errors>) -> Self {
        UnifiedResult(result)
    }

    /// Resolves the `UnifiedResult` and returns the data if successful.
    pub fn resolve(self) -> T {
        match self.0 {
            Ok(o) => {
                o.warning.display();
                o.data
            }
            Err(e) => {
                e.display();
                std::process::exit(1);
            }
        }
    }

    /// Unwraps the `UnifiedResult` and returns the data or errors.
    pub fn unwrap(self) -> Result<T, Errors> {
        match self.0 {
            Ok(d) => Ok(d.data),
            Err(e) => Err(e),
        }
    }
}

// Pretty display for GenericWarning
impl fmt::Display for GenericWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.warn_mesg {
            Some(d) => write!(f, "Warning: {:#?} - {}", self.warn_type, d),
            None => write!(f, "Warning: generic"),
        }
    }
}

// Pretty display for GenericError
impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.err_mesg {
            Some(d) => write!(f, "Error: {:#?} - {}", self.err_type, d),
            None => write!(f, "Error: generic"),
        }
    }
}

// Conversion from io::Error to GenericError
impl From<io::Error> for GenericError {
    fn from(err: io::Error) -> Self {
        GenericError::new_details(GenErrors::ErrorInputOutput, err.to_string())
    }
}

// Conversion from walkdir::Error to GenericError
impl From<walkdir::Error> for GenericError {
    fn from(err: walkdir::Error) -> Self {
        GenericError::new_details(GenErrors::ErrorInputOutput, err.to_string())
    }
}
