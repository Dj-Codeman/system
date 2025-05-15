use crate::core::errors::WarningArray;

use super::WarningArrayItem;

/// Represents a result that contains data and warnings.
#[derive(Debug)]
pub struct OkWarning<T> {
    /// Data associated with the result.
    pub data: T,
    /// Warnings associated with the result.
    pub warning: WarningArray,
}

/// returns the data within the OkWarning<T>
/// This consumes the warning
impl<T> OkWarning<T> {
    /// returns the data within the OkWarning<T>
    /// This consumes the warning
    pub fn strip(self) -> T {
        let ok_warning: OkWarning<T> = self;
        ok_warning.warning.display();
        ok_warning.data
    }

    /// new_none wraps the associated T into a OkWarning<T> and the warning field is a empty warning array container.
    pub fn new_none(value: T) -> Self {
        OkWarning {
            data: value,
            warning: WarningArray::new_container(),
        }
    }

    /// Creates a new OkWarning from a WarningArrayItem and a value
    pub fn new_from_item(value: T, warning: WarningArrayItem) -> Self {
        let warning_array: WarningArray = WarningArray::new(vec![warning]);
        Self {
            data: value,
            warning: warning_array,
        }
    }
}