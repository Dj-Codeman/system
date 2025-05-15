pub mod enums;
pub mod structs;
pub mod collections;
mod implementations;
pub mod utils;


pub use enums::{Errors, Warnings, UnifiedResult};
pub use structs::{ErrorArrayItem, WarningArrayItem, OkWarning};
pub use collections::{ErrorArray, WarningArray};