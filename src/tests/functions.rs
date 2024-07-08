#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{
        errors::{ErrorArray, UnifiedResult as uf, WarningArray},
        functions::{
            create_hash, del_dir, del_file, generate_random_string, is_string_in_file, make_dir,
            make_file, path_present, truncate,
        },
        types::PathType,
    };

    const TARGET_STRING: &str = "Line 2";

    fn get_errors() -> ErrorArray {
        ErrorArray::new_container()
    }

    fn get_warnings() -> WarningArray {
        WarningArray::new_container()
    }

    fn get_dir() -> PathType {
        PathType::Content(String::from("/tmp/test"))
    }

    fn get_file() -> PathType {
        let name = create_hash("dummy_test".to_string());
        let mut path = String::new();
        path.push_str("/tmp/");
        path.push_str(&name);
        path.push_str(".file");
        return PathType::Content(path);
    }

    #[test]
    fn random_string() {
        let test_string: String = generate_random_string(10, get_errors()).unwrap();
        assert_eq!(test_string.len(), 10);
    }

    #[test]
    fn trimming() {
        let result = truncate("Hello, World", 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn path_present_test() {
        let result: uf<bool> = path_present(&get_file(), get_errors());
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn hash() {
        let result = create_hash("hash".to_string());
        assert_eq!(
            result,
            "d04b98f48e8f8bcc15c6ae5ac050801cd6dcfd428fb5f9e65c4e16e7807340fa".to_string()
        );
    }

    #[test]
    fn create_dir() {
        let _ = del_dir(&&get_dir(), get_errors()).unwrap();
        let _ = make_dir(&get_dir(), get_errors()).unwrap();
        assert_eq!(path_present(&get_dir(), get_errors()).unwrap(), true);
    }

    #[test]
    fn delete_dir() {
        make_dir(&get_dir(), get_errors()).unwrap();
        del_dir(&get_dir(), get_errors()).unwrap();
        assert_eq!(path_present(&get_dir(), get_errors()).unwrap(), false);
    }

    #[test]
    fn create_file() {
        assert_eq!(make_file(get_file(), get_errors()).is_ok(), true);
    }

    #[test]
    fn delete_file() {
        let _ = make_file(get_file(), get_errors());
        let _ = del_file(get_file(), get_errors(), get_warnings());
        assert_eq!(path_present(&get_file(), get_errors()).unwrap(), false);
    }

    #[test]
    fn test_is_string_in_file() {
        use std::io::Write;
        // Create a temporary file for testing
        let tmp_file_path = "/tmp/test_file.txt";
        let mut file = File::create(tmp_file_path).unwrap();
        writeln!(file, "Line 1").unwrap();
        writeln!(file, "Line 2").unwrap();
        writeln!(file, "Line 3").unwrap();

        // Test with a string that exists in the file
        let path_buf = PathType::Str(tmp_file_path.into());
        assert_eq!(
            is_string_in_file(
                &PathType::PathBuf(path_buf.to_path_buf()),
                TARGET_STRING,
                get_errors()
            )
            .unwrap(),
            true
        );

        // Test with a string that does not exist in the file
        let non_existing_target = "Non-existing line";
        assert_eq!(
            is_string_in_file(
                &PathType::PathBuf(path_buf.to_path_buf()),
                non_existing_target,
                get_errors()
            )
            .unwrap(),
            false
        );

        // Clean up the temporary file
        del_file(
            PathType::Str(tmp_file_path.into()),
            get_errors(),
            get_warnings(),
        )
        .unwrap();
    }
}
