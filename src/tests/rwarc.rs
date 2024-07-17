#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::time::Duration;

    use crate::rwarc::LockWithTimeout;
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct AppName(String);

    #[derive(Debug, Clone)]
    struct Status;

    #[tokio::test]
    async fn test_try_write_with_timeout_success() {
        let state: HashMap<AppName, Status> = HashMap::new();
        let lock_with_timeout = Arc::new(LockWithTimeout::new(state));

        let timeout_duration = Duration::from_secs(1);
        let result = lock_with_timeout.try_write_with_timeout(Some(timeout_duration)).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_try_read_with_timeout_success() {
        let state: HashMap<AppName, Status> = HashMap::new();
        let lock_with_timeout = Arc::new(LockWithTimeout::new(state));

        let timeout_duration = Duration::from_secs(1);
        let result = lock_with_timeout.try_read_with_timeout(Some(timeout_duration)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_try_read_success() {
        let state: HashMap<AppName, Status> = HashMap::new();
        let lock_with_timeout = Arc::new(LockWithTimeout::new(state));

        let result = lock_with_timeout.try_read().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_try_write_success() {
        let state: HashMap<AppName, Status> = HashMap::new();
        let lock_with_timeout = Arc::new(LockWithTimeout::new(state));

        let result = lock_with_timeout.try_write().await;

        assert!(result.is_ok());
    }
}
