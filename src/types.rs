use std::{
    fmt, fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tempfile::{tempdir /*tempfile*/};

use crate::{
    errors::{ErrorArrayItem, Errors},
    log,
    log::LogLevel,
    stringy::Stringy,
};

/// Represents different types of paths.
///
/// This enum can hold various types of paths:
///
/// - `PathBuf`: Represents an owned path buffer.
/// - `Path`: Represents a borrowed path.
/// - `str`: Represents a borrowed string path.
/// - `Content`: Represents a path as a string content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathType {
    /// Represents an owned path buffer.
    PathBuf(PathBuf),
    /// Represents a borrowed path.
    Path(Box<Path>),
    /// Represents a borrowed string path.
    Str(Box<str>),
    /// Represents a path as a string content.
    Content(String),
    /// Represents a path as a stringy
    Stringy(Stringy),
}

/// A trait for types that can be converted into a `PathBuf`.
pub trait CopyPath {
    /// Returns a `PathBuf` representing the path.
    fn copy_path(&self) -> PathBuf;
}

/// A trait for types that can be cloned into a `PathType`.
pub trait ClonePath {
    /// Returns a cloned `PathType`.
    fn clone_path(&self) -> PathType;
}

impl ClonePath for PathType {
    /// Clones the `PathType` into a new instance.
    fn clone_path(&self) -> PathType {
        match self {
            PathType::PathBuf(d) => PathType::PathBuf(d.clone()),
            PathType::Path(d) => PathType::Path(d.clone()),
            PathType::Str(d) => PathType::Str(d.clone()),
            PathType::Content(d) => PathType::Content(d.clone()),
            PathType::Stringy(d) => PathType::Stringy(d.clone()),
        }
    }
}

impl CopyPath for PathType {
    /// Converts the `PathType` into a `PathBuf`.
    fn copy_path(&self) -> PathBuf {
        match self {
            PathType::PathBuf(path_buf) => path_buf.clone(),
            PathType::Path(path) => path.as_ref().to_path_buf(),
            PathType::Str(str_box) => PathBuf::from(&**str_box),
            PathType::Content(content) => PathBuf::from(content),
            PathType::Stringy(stringy) => PathBuf::from(stringy.to_string()),
        }
    }
}

impl PathType {
    /// Converts the `PathType` into a `PathBuf`.
    pub fn to_path_buf(&self) -> PathBuf {
        self.copy_path()
    }

    /// Converts the `PathType` into a `Path`.
    pub fn to_path(&self) -> Box<Path> {
        self.copy_path().as_path().into()
    }

    /// Attempts to delete the file or directory
    pub fn delete(&self) -> Result<(), ErrorArrayItem> {
        match self.exists() {
            true => {
                if self.is_dir() {
                    fs::remove_dir_all(&self).map_err(ErrorArrayItem::from)
                } else if self.is_file() || self.is_symlink() {
                    fs::remove_file(&self).map_err(ErrorArrayItem::from)
                } else {
                    Ok(())
                }
            }
            false => {
                log!(LogLevel::Warn, "{}, Doesn't exist", self.to_string());
                return Ok(());
            }
        }
    }

    pub fn temp_dir() -> Result<Self, ErrorArrayItem> {
        if let Ok(dir) = tempdir() {
            let path = dir.into_path();
            Ok(PathType::PathBuf(path))
        } else {
            Err(ErrorArrayItem::new(
                Errors::CreatingDirectory,
                "Failed to create a temp dir",
            ))
        }
    }

    // pub fn temp_file() -> Result<Self, ErrorArrayItem> {
    //     if let Ok(file) = tempfile() {
    //         let file_meta = file.metadata().map_err(ErrorArrayItem::from)?;
    //         let path = P

    //         Ok(PathType::PathBuf(path))
    //     } else {
    //         Err(ErrorArrayItem::new(Errors::CreatingDirectory, "Failed to create a temp dir"))
    //     }
    // }
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathType::PathBuf(path_buf) => write!(f, "{}", path_buf.display()),
            PathType::Path(path) => write!(f, "{}", path.display()),
            PathType::Str(str_box) => write!(f, "{}", str_box),
            PathType::Content(content) => write!(f, "{}", content),
            PathType::Stringy(stringy) => write!(f, "{}", stringy),
        }
    }
}

impl Deref for PathType {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        match self {
            PathType::PathBuf(path_buf) => path_buf.as_path(),
            PathType::Path(path) => path.as_ref(),
            PathType::Str(str_box) => Path::new(&**str_box),
            PathType::Content(content) => Path::new(content),
            PathType::Stringy(stringy) => Path::new(&*stringy),
        }
    }
}

impl<T> AsRef<T> for PathType
where
    T: ?Sized,
    <PathType as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl From<PathBuf> for PathType {
    fn from(path_buf: PathBuf) -> Self {
        PathType::PathBuf(path_buf)
    }
}

impl From<&PathBuf> for PathType {
    fn from(path_buf: &PathBuf) -> Self {
        PathType::PathBuf(path_buf.clone())
    }
}

impl From<Box<Path>> for PathType {
    fn from(path: Box<Path>) -> Self {
        PathType::Path(path)
    }
}

impl From<&str> for PathType {
    fn from(path: &str) -> Self {
        let new_path: String = String::from(path);
        PathType::Content(new_path)
    }
}
