use std::{collections, convert::Infallible, io, net, num::{ParseIntError, TryFromIntError}, path, str::Utf8Error, string::FromUtf8Error, sync, thread, time};

use block_modes::BlockModeError;
use hex::FromHexError;
use nix::errno::Errno;

use crate::core::{errors::{ErrorArrayItem, Errors}, errors_dep::SystemError};


// Conversion from &mut std::io::Error to ErrorArrayItem
impl From<&mut io::Error> for ErrorArrayItem {
    fn from(err: &mut io::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::path::StripPrefixError to ErrorArrayItem
impl From<path::StripPrefixError> for ErrorArrayItem {
    fn from(err: path::StripPrefixError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut std::path::StripPrefixError to ErrorArrayItem
impl From<&mut path::StripPrefixError> for ErrorArrayItem {
    fn from(err: &mut path::StripPrefixError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::thread::AccessError to ErrorArrayItem
impl From<thread::AccessError> for ErrorArrayItem {
    fn from(err: thread::AccessError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut std::thread::AccessError to ErrorArrayItem
impl From<&mut thread::AccessError> for ErrorArrayItem {
    fn from(err: &mut thread::AccessError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::sync::mpsc::SendError<T> to ErrorArrayItem
impl<T> From<sync::mpsc::SendError<T>> for ErrorArrayItem {
    fn from(err: sync::mpsc::SendError<T>) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut std::sync::mpsc::SendError<T> to ErrorArrayItem
impl<T> From<&mut sync::mpsc::SendError<T>> for ErrorArrayItem {
    fn from(err: &mut sync::mpsc::SendError<T>) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::net::AddrParseError to ErrorArrayItem
impl From<net::AddrParseError> for ErrorArrayItem {
    fn from(err: net::AddrParseError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut std::net::AddrParseError to ErrorArrayItem
impl From<&mut net::AddrParseError> for ErrorArrayItem {
    fn from(err: &mut net::AddrParseError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::collections::TryReserveError to ErrorArrayItem
impl From<collections::TryReserveError> for ErrorArrayItem {
    fn from(err: collections::TryReserveError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut std::collections::TryReserveError to ErrorArrayItem
impl From<&mut collections::TryReserveError> for ErrorArrayItem {
    fn from(err: &mut collections::TryReserveError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from std::time::SystemTimeError to ErrorArrayItem
impl From<time::SystemTimeError> for ErrorArrayItem {
    fn from(err: time::SystemTimeError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut std::time::SystemTimeError to ErrorArrayItem
impl From<&mut time::SystemTimeError> for ErrorArrayItem {
    fn from(err: &mut time::SystemTimeError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}
// // Conversion from regex::Error to ErrorArrayItem
// impl TryFrom<regex::Error> for ErrorArrayItem {
//     type Error = ();

//     fn try_from(err: regex::Error) -> Result<Self, Self::Error> {
//         Ok(ErrorArrayItem::new(Errors::InputOutput, err.to_string()))
//     }
// }

// // Conversion from &mut regex::Error to ErrorArrayItem
// impl TryFrom<&mut regex::Error> for ErrorArrayItem {
//     type Error = ();

//     fn try_from(err: &mut regex::Error) -> Result<Self, Self::Error> {
//         Ok(ErrorArrayItem::new(Errors::InputOutput, err.to_string()))
//     }
// }

// Conversion from serde_json::Error to ErrorArrayItem
impl From<serde_json::Error> for ErrorArrayItem {
    fn from(err: serde_json::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut serde_json::Error to ErrorArrayItem
impl From<&mut serde_json::Error> for ErrorArrayItem {
    fn from(err: &mut serde_json::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from serde_yaml::Error to ErrorArrayItem
impl From<serde_yaml::Error> for ErrorArrayItem {
    fn from(err: serde_yaml::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut serde_yaml::Error to ErrorArrayItem
impl From<&mut serde_yaml::Error> for ErrorArrayItem {
    fn from(err: &mut serde_yaml::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from reqwest::Error to ErrorArrayItem
impl From<reqwest::Error> for ErrorArrayItem {
    fn from(err: reqwest::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut reqwest::Error to ErrorArrayItem
impl From<&mut reqwest::Error> for ErrorArrayItem {
    fn from(err: &mut reqwest::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from rand::Error to ErrorArrayItem
impl From<rand::Error> for ErrorArrayItem {
    fn from(err: rand::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut rand::Error to ErrorArrayItem
impl From<&mut rand::Error> for ErrorArrayItem {
    fn from(err: &mut rand::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from walkdir::Error to ErrorArrayItem
impl From<walkdir::Error> for ErrorArrayItem {
    fn from(err: walkdir::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from &mut walkdir::Error to ErrorArrayItem
impl From<&mut walkdir::Error> for ErrorArrayItem {
    fn from(err: &mut walkdir::Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, err.to_string())
    }
}

// Conversion from FromUtf8Error::Error to ErrorArrayItem
impl From<FromUtf8Error> for ErrorArrayItem {
    fn from(value: FromUtf8Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from &mut FromUtf8Error::Error to ErrorArrayItem
impl From<&mut FromUtf8Error> for ErrorArrayItem {
    fn from(value: &mut FromUtf8Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from Utf8Error::Error to ErrorArrayItem
impl From<Utf8Error> for ErrorArrayItem {
    fn from(value: Utf8Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from &mut Utf8Error::Error to ErrorArrayItem
impl From<&mut Utf8Error> for ErrorArrayItem {
    fn from(value: &mut Utf8Error) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from FromHexError::Error to ErrorArrayItem
impl From<FromHexError> for ErrorArrayItem {
    fn from(value: FromHexError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from &mut FromHexError::Error to ErrorArrayItem
impl From<&mut FromHexError> for ErrorArrayItem {
    fn from(value: &mut FromHexError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from nix errors to ErrorArrayItem
#[cfg(unix)]
impl From<Errno> for ErrorArrayItem {
    fn from(value: Errno) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from &mut nix errors to ErrorArrayItem
#[cfg(unix)]
impl From<&mut Errno> for ErrorArrayItem {
    fn from(value: &mut Errno) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from ParseIntError errors to ErrorArrayItem
impl From<ParseIntError> for ErrorArrayItem {
    fn from(value: ParseIntError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

// Conversion from &mut ParseIntError errors to ErrorArrayItem
impl From<&mut ParseIntError> for ErrorArrayItem {
    fn from(value: &mut ParseIntError) -> Self {
        ErrorArrayItem::new(Errors::InputOutput, value.to_string())
    }
}

#[allow(deprecated)]
// Conversion from deprecated system Errors
impl From<SystemError> for ErrorArrayItem {
    fn from(value: SystemError) -> Self {
        ErrorArrayItem::new(
            Errors::DEPSYSTEM,
            value.details.unwrap_or(String::from("No message appended")),
        )
    }
}

impl From<Infallible> for ErrorArrayItem {
    fn from(value: std::convert::Infallible) -> Self {
        ErrorArrayItem::new(Errors::GeneralError, value.to_string())
    }
}

impl From<block_modes::InvalidKeyIvLength> for ErrorArrayItem {
    fn from(value: block_modes::InvalidKeyIvLength) -> Self {
        ErrorArrayItem::new(Errors::GeneralError, value.to_string())
    }
}

impl From<BlockModeError> for ErrorArrayItem {
    fn from(value: BlockModeError) -> Self {
        ErrorArrayItem::new(Errors::GeneralError, value.to_string())
    }
}

impl From<TryFromIntError> for ErrorArrayItem {
    fn from(value: TryFromIntError) -> Self {
        ErrorArrayItem::new(Errors::GeneralError, value.to_string())
    }
}

#[cfg(unix)]
impl From<nix::Error> for ErrorArrayItem {
    fn from(value: nix::Error) -> Self {
        ErrorArrayItem::new(Errors::GeneralError, value.to_string())
    }
}

impl From<tokio::sync::TryLockError> for ErrorArrayItem {
    fn from(value: tokio::sync::TryLockError) -> Self {
        ErrorArrayItem::new(Errors::GeneralError, value.to_string())
    }
}

// ! Convertions from std errors

impl From<std::io::Error> for ErrorArrayItem {
    fn from(io_err: std::io::Error) -> Self {
        ErrorArrayItem::new(
            Errors::InputOutput,                // or whatever variant makes sense
            io_err.to_string(),
        )
    }
}