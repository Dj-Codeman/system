use std::{ffi::OsStr, fmt, ops::Deref, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stringy {
    Immutable(Arc<str>),
    Mutable(String),
}

impl Stringy {
    /// Create a new Stringy from a &str
    pub fn new(s: &str) -> Self {
        Self::Immutable(Arc::from(s))
    }

    /// Create a new Stringy from a String
    pub fn from_string(s: String) -> Self {
        Self::Immutable(s.into())
    }

    /// Convert the Stringy to an Arc<str>
    pub fn as_arc_str(&self) -> Arc<str> {
        match self {
            Stringy::Immutable(arc_str) => Arc::clone(arc_str),
            Stringy::Mutable(s) => Arc::from(s.as_str()),
        }
    }

    /// Mutate the string if necessary. This avoids unnecessary conversion
    /// unless mutation is actually performed.
    pub fn mutate<F>(&mut self, f: F)
    where
        F: FnOnce(&mut String),
    {
        // Convert to mutable String if currently immutable
        if let Stringy::Immutable(arc_str) = self {
            // We have an immutable string, so convert it to a mutable String
            *self = Stringy::Mutable(arc_str.to_string());
        }

        // Apply the mutation on the mutable String
        if let Stringy::Mutable(s) = self {
            f(s);
        }
    }

    /// Avoid converting to String unless strictly necessary for operations.
    /// If only read access is needed, clone the Arc<str> to avoid converting to String.
    pub fn clone_immutable(&self) -> Arc<str> {
        match self {
            Stringy::Immutable(arc_str) => Arc::clone(arc_str),
            Stringy::Mutable(s) => Arc::from(s.as_str()),
        }
    }
}

impl Deref for Stringy {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Stringy::Immutable(arc_str) => arc_str.deref(),
            Stringy::Mutable(s) => s.deref(),
        }
    }
}

// Implement AsRef<OsStr> for `Stringy`
impl AsRef<OsStr> for Stringy {
    fn as_ref(&self) -> &OsStr {
        match self {
            Stringy::Immutable(arc_str) => OsStr::new(&**arc_str),
            Stringy::Mutable(s) => OsStr::new(s),
        }
    }
}

// Custom implementation for Serialize
impl Serialize for Stringy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Stringy::Immutable(arc_str) => {
                // Convert Arc<str> to a String before serialization
                serializer.serialize_str(&arc_str)
            }
            Stringy::Mutable(s) => {
                // Serialize the String directly
                serializer.serialize_str(s)
            }
        }
    }
}

// Custom implementation for Deserialize
impl<'de> Deserialize<'de> for Stringy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Stringy::Immutable(Arc::from(s)))
    }
}

impl fmt::Display for Stringy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stringy::Immutable(arc_str) => write!(f, "{}", arc_str),
            Stringy::Mutable(ref string) => write!(f, "{}", string),
        }
    }
}

impl From<String> for Stringy {
    fn from(s: String) -> Self {
        Self::Immutable(Arc::from(s.as_str()))
    }
}

impl From<&str> for Stringy {
    fn from(s: &str) -> Self {
        Self::Immutable(Arc::from(s))
    }
}

impl From<&String> for Stringy {
    fn from(s: &String) -> Self {
        Self::Immutable(Arc::from(s.as_str()))
    }
}
