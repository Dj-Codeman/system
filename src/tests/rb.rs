#[cfg(test)]
mod tests {
    use crate::types::rb::RollingBuffer;

    #[test]
    fn test_capacity() {
        let buffer = RollingBuffer::new(5);
        assert_eq!(buffer.capacity(), 5);
    }

    #[test]
    fn test_push_and_get_latest() {
        let mut buffer = RollingBuffer::new(2);
        buffer.push("one".to_string());
        buffer.push("two".to_string());
        assert_eq!(buffer.get_latest(), vec!["one", "two"]);
    }

    #[test]
    fn test_overflow() {
        let mut buffer = RollingBuffer::new(2);
        buffer.push("one".to_string());
        buffer.push("two".to_string());
        buffer.push("three".to_string()); // Should drop "one"
        assert_eq!(buffer.get_latest(), vec!["two", "three"]);
    }

    #[test]
    fn test_is_empty_and_is_full() {
        let mut buffer = RollingBuffer::new(2);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());

        buffer.push("one".to_string());
        assert!(!buffer.is_empty());
        assert!(!buffer.is_full());

        buffer.push("two".to_string());
        assert!(!buffer.is_empty());
        assert!(buffer.is_full());
    }

    #[test]
    fn test_front_and_back() {
        let mut buffer = RollingBuffer::new(3);
        assert!(buffer.front().is_none());
        assert!(buffer.back().is_none());

        buffer.push("first".to_string());
        buffer.push("second".to_string());
        buffer.push("third".to_string());

        assert_eq!(buffer.front(), Some(&"first".to_string()));
        assert_eq!(buffer.back(), Some(&"third".to_string()));
    }

    #[test]
    fn test_pop_front_and_back() {
        let mut buffer = RollingBuffer::new(3);
        buffer.push("one".to_string());
        buffer.push("two".to_string());
        buffer.push("three".to_string());

        let front_line = buffer.pop_front();
        assert_eq!(front_line, Some("one".to_string()));
        assert_eq!(buffer.len(), 2);

        let back_line = buffer.pop_back();
        assert_eq!(back_line, Some("three".to_string()));
        assert_eq!(buffer.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut buffer = RollingBuffer::new(2);
        buffer.push("one".to_string());
        buffer.push("two".to_string());
        buffer.clear();
        assert!(buffer.is_empty());
    }
}
