#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::types::stringy::Stringy;

    #[test]
    fn test_creation_from_str() {
        let s = Stringy::from("Hello");
        if let Stringy::Immutable(arc_str) = &s {
            assert_eq!(arc_str.as_ref(), "Hello");
        } else {
            panic!("Expected Immutable variant.");
        }
    }

    #[test]
    fn test_creation_from_string() {
        let s = Stringy::from(String::from("World"));
        if let Stringy::Immutable(arc_str) = &s {
            assert_eq!(arc_str.as_ref(), "World");
        } else {
            panic!("Expected Immutable variant.");
        }
    }

    #[test]
    fn test_mutate_string() {
        let mut s = Stringy::from("Hello");

        // Mutate the string
        s.mutate(|str_val| {
            str_val.push_str(", World!");
        });

        if let Stringy::Mutable(mutated_str) = &s {
            assert_eq!(mutated_str, "Hello, World!");
        } else {
            panic!("Expected Mutable variant after mutation.");
        }
    }

    #[test]
    fn test_immutable_to_mutable() {
        let mut s = Stringy::from("Immutable");

        // Check the initial state
        if let Stringy::Immutable(arc_str) = &s {
            assert_eq!(arc_str.as_ref(), "Immutable");
        } else {
            panic!("Expected Immutable variant.");
        }

        // Mutate the string
        s.mutate(|str_val| {
            str_val.push_str(" and now mutable");
        });

        if let Stringy::Mutable(mutated_str) = &s {
            assert_eq!(mutated_str, "Immutable and now mutable");
        } else {
            panic!("Expected Mutable variant after mutation.");
        }
    }

    #[test]
    fn test_clone_immutable() {
        let s = Stringy::from("Clonable");
        let cloned = s.clone_immutable();

        assert_eq!(cloned.as_ref(), "Clonable");

        // Original should still be immutable
        if let Stringy::Immutable(arc_str) = &s {
            assert_eq!(arc_str.as_ref(), "Clonable");
        } else {
            panic!("Expected Immutable variant.");
        }
    }

    #[test]
    fn test_arc_reference_count() {
        let s = Stringy::from("Arc Test");

        if let Stringy::Immutable(arc_str) = &s {
            let arc_clone = Arc::clone(arc_str);
            assert_eq!(Arc::strong_count(arc_str), 2); // Two references: original and clone

            // Drop the clone and check reference count
            drop(arc_clone);
            assert_eq!(Arc::strong_count(arc_str), 1);
        } else {
            panic!("Expected Immutable variant.");
        }
    }

    #[test]
    fn test_mutation_does_not_affect_original_arc() {
        let mut s = Stringy::from("Shared");

        // Clone the immutable string
        let cloned = s.clone_immutable();

        // Mutate the original string
        s.mutate(|str_val| {
            str_val.push_str(" modified");
        });

        // Ensure the cloned immutable version remains unchanged
        assert_eq!(cloned.as_ref(), "Shared");

        // The original should now be mutated
        if let Stringy::Mutable(modified_str) = &s {
            assert_eq!(modified_str, "Shared modified");
        } else {
            panic!("Expected Mutable variant after mutation.");
        }
    }

    #[test]
    fn test_as_str_return_correct_type() {
        let original: &str = "Value";
        let stringy = Stringy::from(original);

        assert_eq!(stringy.as_str(), original)
    }
}
