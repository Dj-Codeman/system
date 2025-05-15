use crate::core::errors::ErrorArrayItem;
use crate::core::errors::Errors;
use crate::core::logger::LogLevel;
use crate::log;
use std::sync::Arc;
use std::sync::RwLock;

/// Represents a collection of errors.
#[derive(Debug, Clone)]
pub struct ErrorArray(pub Arc<RwLock<Vec<ErrorArrayItem>>>);

impl ErrorArray {
    /// Creates a new `Errors` instance.
    pub fn new(mut data: Vec<ErrorArrayItem>) -> Self {
        let error_array: Vec<ErrorArrayItem> = Vec::with_capacity(2);
        let error: ErrorArray = Self {
            0: Arc::new(RwLock::new(error_array)),
        };

        let mut to_append = error.0.write().unwrap();
        to_append.append(&mut data);
        drop(to_append);
        return error;
    }

    /// Creats an [`ErrorArray`] from a single [`ErrorArrayItem`]
    pub fn from(ei: ErrorArrayItem) -> Self {
        let mut container: ErrorArray = Self::new_container();
        container.push(ei);
        container
    }

    /// Clears the [`ErrorArray`]
    #[allow(unused_assignments)]
    pub fn clear(&mut self) {
        if let Ok(mut internal_array) = self.0.write() {
            internal_array.clear();
        } else {
            log!(
                LogLevel::Trace,
                "ERROR CLEANING THE ERROR ARRAY kinda dumb honestly"
            );
        };
    }

    /// Creates an empty `Errors` instance.
    pub fn new_container() -> Self {
        let error_array: Vec<ErrorArrayItem> = Vec::new();
        Self {
            0: Arc::new(RwLock::new(error_array)),
        }
    }

    /// Displays the errors.
    pub fn display(self, die: bool) {
        let mut error_array = self.0.write().unwrap();
        for errors in error_array.as_slice() {
            log!(LogLevel::Error, "{}", errors);
        }
        if die {
            std::process::exit(1);
        } else {
            error_array.clear()
        }
    }

    /// Pushes a new error to the collection.
    pub fn push(&mut self, item: ErrorArrayItem) {
        let mut error_array = self.0.write().unwrap();
        error_array.push(item);
    }

    /// Pop the last error from the array
    pub fn pop(&mut self) -> ErrorArrayItem {
        let mut error_array = self.0.write().unwrap();
        error_array.pop().unwrap_or(ErrorArrayItem::new(
            Errors::GeneralError,
            String::from("No previous error"),
        ))
    }

    pub fn append(&mut self, arr: Self) {
        let mut error_array = self.0.write().unwrap();
        let mut donor_array = arr.0.write().unwrap();
        error_array.append(&mut donor_array);
        drop(donor_array);
    }

    pub fn len(&self) -> usize {
        let vec = self.0.read().unwrap(); // Lock the RwLock and get a read guard
        vec.len()
    }
}
