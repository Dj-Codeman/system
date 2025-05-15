use crate::core::errors::structs::error_item::ErrorArrayItem;
use std::error::Error;

impl Error for ErrorArrayItem {
    // If you ever wrap some other error inside your `ErrorArrayItem`,
    // you can return `Some(&source)` here.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// impl From<ErrorArrayItem> for Errors {
//     fn from(item: ErrorArrayItem) -> Self {
//         // wrap it in whatever variant you use
//         Errors::FirstClass(item)
//     }
// }
