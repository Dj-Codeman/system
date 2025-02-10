use std::collections::VecDeque;

/// A rolling buffer that stores up to a fixed number of recent lines.
///
/// # Examples
///
/// ```
/// use dusa_collection_utils::types::rb::RollingBuffer;
///
/// let mut buffer = RollingBuffer::new(3);
///
/// buffer.push("line 1".to_string());
/// buffer.push("line 2".to_string());
/// buffer.push("line 3".to_string());
///
/// // The buffer is now full, so pushing a new line will drop the oldest.
/// buffer.push("line 4".to_string());
///
/// assert_eq!(buffer.len(), 3);
/// // The oldest line ("line 1") has been dropped.
/// assert_eq!(buffer.get_latest(), vec!["line 2", "line 3", "line 4"]);
/// ```
pub struct RollingBuffer {
    /// A double-ended queue that holds the lines.
    lines: VecDeque<String>,
    /// The maximum capacity of the buffer.
    capacity: usize,
}

impl RollingBuffer {
    /// Creates a new `RollingBuffer` with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The maximum number of lines to store in the buffer.
    ///
    /// # Panics
    ///
    /// This function doesn't panic by default. However, note that if `capacity` is 0,
    /// pushing lines will silently do nothing.
    pub fn new(capacity: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Returns the capacity of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let buffer = RollingBuffer::new(5);
    /// assert_eq!(buffer.capacity(), 5);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Pushes a new line onto the buffer. If the buffer is at capacity, the oldest line
    /// is dropped.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to be pushed onto the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(2);
    /// buffer.push("first".to_string());
    /// buffer.push("second".to_string());
    /// buffer.push("third".to_string()); // drops "first"
    ///
    /// assert_eq!(buffer.get_latest(), vec!["second", "third"]);
    /// ```
    pub fn push(&mut self, line: String) {
        if self.lines.len() == self.capacity {
            // Drop the oldest line.
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    /// Returns a copy of all lines in the buffer, from oldest to newest.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(3);
    /// buffer.push("one".to_string());
    /// buffer.push("two".to_string());
    /// buffer.push("three".to_string());
    ///
    /// let lines = buffer.get_latest();
    /// assert_eq!(lines, vec!["one", "two", "three"]);
    /// ```
    pub fn get_latest(&self) -> Vec<String> {
        self.lines.iter().cloned().collect()
    }

    /// Returns true if the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(2);
    /// assert!(buffer.is_empty());
    ///
    /// buffer.push("not empty".to_string());
    /// assert!(!buffer.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Returns true if the buffer is at maximum capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(2);
    /// assert!(!buffer.is_full());
    ///
    /// buffer.push("line 1".to_string());
    /// buffer.push("line 2".to_string());
    /// assert!(buffer.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.lines.len() == self.capacity
    }

    /// Returns the number of lines currently in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(2);
    /// buffer.push("one".to_string());
    /// assert_eq!(buffer.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Clears all lines from the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(2);
    /// buffer.push("one".to_string());
    /// buffer.push("two".to_string());
    ///
    /// buffer.clear();
    /// assert!(buffer.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Returns the oldest line in the buffer, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(3);
    /// buffer.push("first".to_string());
    /// buffer.push("second".to_string());
    ///
    /// assert_eq!(buffer.front(), Some(&"first".to_string()));
    /// ```
    pub fn front(&self) -> Option<&String> {
        self.lines.front()
    }

    /// Returns the most recent line in the buffer, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(3);
    /// buffer.push("first".to_string());
    /// buffer.push("second".to_string());
    ///
    /// assert_eq!(buffer.back(), Some(&"second".to_string()));
    /// ```
    pub fn back(&self) -> Option<&String> {
        self.lines.back()
    }

    /// Attempts to pop the oldest line from the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(3);
    /// buffer.push("first".to_string());
    /// buffer.push("second".to_string());
    ///
    /// let popped = buffer.pop_front();
    /// assert_eq!(popped, Some("first".to_string()));
    /// ```
    pub fn pop_front(&mut self) -> Option<String> {
        self.lines.pop_front()
    }

    /// Attempts to pop the newest line from the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use dusa_collection_utils::types::rb::RollingBuffer;
    /// let mut buffer = RollingBuffer::new(3);
    /// buffer.push("first".to_string());
    /// buffer.push("second".to_string());
    ///
    /// let popped = buffer.pop_back();
    /// assert_eq!(popped, Some("second".to_string()));
    /// ```
    pub fn pop_back(&mut self) -> Option<String> {
        self.lines.pop_back()
    }
}
