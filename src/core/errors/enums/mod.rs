pub mod errors;
pub mod warnings;
pub mod unified;

// re-export so downstream code does:
//    use crate::errors::Errors;
//    use crate::errors::Warnings;
pub use errors::Errors;
pub use warnings::Warnings;
pub use unified::UnifiedResult;