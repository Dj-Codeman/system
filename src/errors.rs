use std::fmt;

#[derive(Debug, PartialEq)]
pub struct SystemError {
    pub kind: SystemErrorType,
    pub details: Option<String>
}

#[derive(Debug, PartialEq)]
pub enum SystemErrorType {
    ErrorOpeningFile,
    ErrorReadingFile,
    ErrorCreatingFile,
    ErrorCreatingDir,
    ErrorDeletingDir,
    ErrorDeletingFile,
    ErrorSettingPermDir,
    ErrorSettingPermFile,
    ErrorUntaringFile,
}

// pretty display
impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.details {
            Some(d) => write!(f, "Logger Error: {} - {}", self.kind_description(), d),
            None => write!(f, "Logger Error: {}", self.kind_description()),
        }
    }
}

impl SystemError {
    pub fn new(kind: SystemErrorType) -> Self {
        SystemError {
            kind, 
            details: None,
        }
    }

    pub fn new_details(kind: SystemErrorType, details: &str) -> Self {
        SystemError {
            kind,
            details: Some(details.to_string())
        }
    }

    fn kind_description(&self) -> String {
        match &self.kind {
            SystemErrorType::ErrorOpeningFile => String::from("An error occoured while opening a file"),
            SystemErrorType::ErrorReadingFile => String::from("An error occoured while trying to read file data"),
            SystemErrorType::ErrorCreatingFile => String::from("An error occoured while creating file"),
            SystemErrorType::ErrorCreatingDir => String::from("An error occoured while creating directories"),
            SystemErrorType::ErrorDeletingDir => String::from("An error occoured while deleting directory"),
            SystemErrorType::ErrorDeletingFile => String::from("An error occoured while deleting file"),
            SystemErrorType::ErrorSettingPermDir => String::from("An error occoured while setting folder permissions"),
            SystemErrorType::ErrorSettingPermFile => String::from("An error occoured while setting file permissions"),
            SystemErrorType::ErrorUntaringFile => String::from("An error was encountered while reading a tar file"),
        }
    }
}