use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::time::{self, timeout};

use crate::errors::{ErrorArrayItem, Errors};

/// A struct that encapsulates an `Arc<RwLock<T>>` and provides methods
/// to acquire read and write locks with a timeout.
#[derive(Debug, Clone)]
pub struct LockWithTimeout<T> {
    state: Arc<RwLock<T>>,
}

impl<T> LockWithTimeout<T> {
    /// Creates a new `LockWithTimeout` with the given state.
    ///
    /// # Arguments
    ///
    /// * `state` - The initial state to be wrapped by the `RwLock`.
    ///
    /// # Returns
    ///
    /// A new instance of `LockWithTimeout`.
    pub fn new(state: T) -> Self {
        Self {
            state: Arc::new(RwLock::new(state)),
        }
    }

    /// Clones the `LockWithTimeout<T>`.
    ///
    /// # Returns
    ///
    /// A clone of the `LockWithTimeout<T>`.
    pub fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }

    /// Attempts to acquire a write lock on the shared state with a timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout_time` - An optional `Duration` specifying the timeout duration.
    ///
    /// # Returns
    ///
    /// A `Result` containing a write lock guard on success, or an error on timeout.
    pub async fn try_write_with_timeout<'a>(
        self: &'a Self,
        timeout_time: Option<Duration>,
    ) -> Result<RwLockWriteGuard<'a, T>, ErrorArrayItem> {
        let timeout_duration: Duration = timeout_time.unwrap_or(Duration::from_secs(1));

        match timeout(timeout_duration, async {
            loop {
                match self.state.try_write() {
                    Ok(guard) => return Ok(guard),
                    Err(_) => {
                        time::sleep(Duration::from_millis(10)).await;
                    }
                }
            }
        })
        .await
        {
            Ok(result) => result,
            Err(_) => Err(ErrorArrayItem::new(
                Errors::GeneralError,
                String::from("Timeout while trying to acquire write lock"),
            )),
        }
    }

    /// Attempts to acquire a read lock on the shared state with a timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout_time` - An optional `Duration` specifying the timeout duration.
    ///
    /// # Returns
    ///
    /// A `Result` containing a read lock guard on success, or an error on timeout.
    pub async fn try_read_with_timeout<'a>(
        self: &'a Self,
        timeout_time: Option<Duration>,
    ) -> Result<RwLockReadGuard<'a, T>, ErrorArrayItem> {
        let timeout_duration: Duration = timeout_time.unwrap_or(Duration::from_secs(1));

        match timeout(timeout_duration, async {
            loop {
                match self.state.try_read() {
                    Ok(guard) => return Ok(guard),
                    Err(_) => {
                        time::sleep(Duration::from_millis(10)).await;
                    }
                }
            }
        })
        .await
        {
            Ok(result) => result,
            Err(_) => Err(ErrorArrayItem::new(
                Errors::GeneralError,
                String::from("Timeout while trying to acquire read lock"),
            )),
        }
    }

    /// Attempts to acquire a read lock on the shared state.
    ///
    /// # Returns
    ///
    /// A `Result` containing a read lock guard on success, or an error on failure.
    pub async fn try_read<'a>(self: &'a Self) -> Result<RwLockReadGuard<'a, T>, ErrorArrayItem> {
        match self.try_read_with_timeout(None).await {
            Ok(d) => Ok(d),
            Err(e) => Err(ErrorArrayItem::from(e)),
        }
    }

    /// Attempts to acquire a write lock on the shared state.
    ///
    /// # Returns
    ///
    /// A `Result` containing a write lock guard on success, or an error on failure.
    pub async fn try_write<'a>(self: &'a Self) -> Result<RwLockWriteGuard<'a, T>, ErrorArrayItem> {
        match self.try_write_with_timeout(None).await {
            Ok(d) => Ok(d),
            Err(e) => Err(ErrorArrayItem::from(e)),
        }
    }
}
