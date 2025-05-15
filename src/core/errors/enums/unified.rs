use crate::core::errors::{ErrorArray, ErrorArrayItem, WarningArray, structs::OkWarning};

pub enum UnifiedResult<T> {
    /// Result variant containing data and warnings.
    ResultWarning(Result<OkWarning<T>, ErrorArrayItem>),
    /// Result variant containing data only.
    ResultNoWarns(Result<T, ErrorArrayItem>),
}

impl<T> UnifiedResult<T> {
    /// Creates a new `UnifiedResult` instance with warnings.
    pub fn new_warn(result: Result<OkWarning<T>, ErrorArrayItem>) -> Self {
        UnifiedResult::ResultWarning(result)
    }

    /// Creates a new `UnifiedResult` instance without warnings.
    pub fn new(result: Result<T, ErrorArrayItem>) -> Self {
        UnifiedResult::ResultNoWarns(result)
    }

    /// Resolves the `UnifiedResult` and returns the data if successful.
    pub fn unwrap(self) -> T {
        match self {
            UnifiedResult::ResultWarning(r) => match r {
                Ok(d) => {
                    d.warning.display();
                    return d.data;
                }
                Err(e) => {
                    ErrorArray::new(vec![e]).display(true);
                    unreachable!()
                }
            },
            UnifiedResult::ResultNoWarns(r) => match r {
                Ok(d) => return d,
                Err(e) => {
                    ErrorArray::new(vec![e]).display(true);
                    unreachable!()
                }
            },
        }
    }

    /// Unwraps the `UnifiedResult` and returns the data or errors.
    /// This function will display any warnings and empty the warning array
    pub fn uf_unwrap(self) -> Result<T, ErrorArrayItem> {
        match self {
            UnifiedResult::ResultWarning(r) => match r {
                Ok(d) => {
                    let warnings: WarningArray = d.warning;
                    let value: T = d.data;
                    warnings.display();
                    return Ok(value);
                }
                Err(e) => return Err(e),
            },
            UnifiedResult::ResultNoWarns(r) => return r,
        }
    }

    /// Determines if the value in UnifiedResult is Ok()
    pub const fn is_ok(&self) -> bool {
        match &self {
            UnifiedResult::ResultWarning(d) => matches!(d, Ok(_)),
            UnifiedResult::ResultNoWarns(d) => matches!(d, Ok(_)),
        }
    }

    /// Determines if the value in UnifiedResult is Ok()
    pub const fn is_err(&self) -> bool {
        match &self {
            UnifiedResult::ResultWarning(d) => matches!(d, Err(_)),
            UnifiedResult::ResultNoWarns(d) => matches!(d, Err(_)),
        }
    }

    /// Gets the ok value if the operation is successful returns none otherwise
    /// This operation will not panic. It does consume the result and displays and clears warnings if any
    /// are present
    pub fn get_ok(self) -> Option<T> {
        match self.uf_unwrap() {
            Ok(d) => Some(d),
            Err(_) => None,
        }
    }

    /// Similar to `get_ok()` this function will get the error value if present and return None if the operation
    /// succeeded
    pub fn get_err(self) -> Option<ErrorArrayItem> {
        match self.uf_unwrap() {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }
}
