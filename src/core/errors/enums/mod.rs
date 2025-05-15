pub mod errors;
pub mod unified;
pub mod warnings;

// re-export so downstream code does:
//    use crate::errors::Errors;
//    use crate::errors::Warnings;
pub use errors::Errors;
pub use unified::UnifiedResult;
pub use warnings::Warnings;
