use std::{
    fmt,
    ops::Deref,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

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
        }
    }
}

impl PathType {
    /// Converts the `PathType` into a `PathBuf`.
    pub fn to_path_buf(&self) -> PathBuf {
        self.copy_path()
    }
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathType::PathBuf(path_buf) => write!(f, "{}", path_buf.display()),
            PathType::Path(path) => write!(f, "{}", path.display()),
            PathType::Str(str_box) => write!(f, "{}", str_box),
            PathType::Content(content) => write!(f, "{}", content),
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

impl From<Box<Path>> for PathType {
    fn from(path: Box<Path>) -> Self {
        PathType::Path(path)
    }
}
