use crate::core::errors::structs::warning_item::WarningArrayItem;
use std::sync::RwLock;
use std::sync::Arc;
use crate::log;
use crate::core::logger::LogLevel;

/// Represents a collection of warnings.
#[derive(Debug, Clone)]
pub struct WarningArray(pub Arc<RwLock<Vec<WarningArrayItem>>>);

impl WarningArray {
    /// Creates a new `WarningArray` instance.
    pub fn new(mut data: Vec<WarningArrayItem>) -> Self {
        let warning_array: Vec<WarningArrayItem> = Vec::new();
        let warning = Self {
            0: Arc::new(RwLock::new(warning_array)),
        };

        let mut to_append = warning.0.write().unwrap();
        to_append.append(&mut data);
        drop(to_append);
        return warning;
    }

    /// Creates an empty `WarningArray` instance.
    pub fn new_container() -> Self {
        let warning_array: Vec<WarningArrayItem> = Vec::new();
        Self {
            0: Arc::new(RwLock::new(warning_array)),
        }
    }

    /// Displays the warnings.
    pub fn display(self) {
        let mut warning_array = self.0.write().unwrap();
        for warns in warning_array.as_slice() {
            log!(LogLevel::Warn, "{}", warns)
        }
        warning_array.clear()
    }

    /// Pushes a new warning to the collection.
    pub fn push(&mut self, item: WarningArrayItem) {
        let mut warning_array = self.0.write().unwrap();
        warning_array.push(item);
        drop(warning_array)
    }

    pub fn append(&mut self, arr: Self) {
        let mut warning_array = self.0.write().unwrap();
        let mut donor_array = arr.0.write().unwrap();
        warning_array.append(&mut donor_array);
        drop(donor_array);
    }

    pub fn len(&self) -> usize {
        let vec = self.0.read().unwrap(); // Lock the RwLock and get a read guard
        vec.len()
    }
}
