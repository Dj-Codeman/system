pub mod error_array;
pub mod warning_array;

// re-export so downstream code does:
//    use crate::errors::ErrorArray;
//    use crate::errors::WarningArray;
pub use error_array::ErrorArray;
pub use warning_array::WarningArray;