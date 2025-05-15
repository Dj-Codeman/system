pub mod collections;
pub mod enums;
mod implementations;
pub mod structs;
pub mod utils;

pub use collections::{ErrorArray, WarningArray};
pub use enums::{Errors, UnifiedResult, Warnings};
pub use structs::{ErrorArrayItem, OkWarning, WarningArrayItem};
