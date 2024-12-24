#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::{
        fs::{self, File},
        os::unix::fs::{MetadataExt, PermissionsExt},
        path::PathBuf,
    };

    use nix::unistd::{Gid, Uid};

    use crate::{
        errors::{UnifiedResult as uf, WarningArray},
        functions::{
            create_hash, del_dir, del_file, generate_random_string, is_string_in_file, make_dir,
            make_file, path_present, set_file_ownership, set_file_permission, tar, truncate, untar,
        },
        types::PathType,
    };

    const TARGET_STRING: &str = "Line 2";

    fn _get_warnings() -> WarningArray {
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

    fn create_test_file(path: &PathBuf) -> Result<(), std::io::Error> {
        if path.exists() {
            fs::remove_file(path)?;
        }
        File::create(path)?;
        Ok(())
    }

    #[test]
    fn random_string() {
        let test_string: String = generate_random_string(10).unwrap();
        assert_eq!(test_string.len(), 10);
    }

    #[test]
    fn trimming() {
        let result = truncate("Hello, World", 5);
        assert_eq!(result, "Hello".into());
    }

    #[test]
    fn path_present_test() {
        let result: uf<bool> = path_present(&get_file());
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn hash() {
        let result = create_hash("hash");
        assert_eq!(
            result,
            "d04b98f48e8f8bcc15c6ae5ac050801cd6dcfd428fb5f9e65c4e16e7807340fa".into()
        );
    }

    #[test]
    fn create_dir() {
        let _ = del_dir(&&get_dir()).unwrap();
        let _ = make_dir(&get_dir()).unwrap();
        assert_eq!(path_present(&get_dir()).unwrap(), true);
    }

    #[test]
    fn delete_dir() {
        make_dir(&get_dir()).unwrap();
        del_dir(&get_dir()).unwrap();
        assert_eq!(path_present(&get_dir()).unwrap(), false);
    }

    #[test]
    fn create_file() {
        assert_eq!(make_file(get_file()).is_ok(), true);
    }

    #[test]
    fn delete_file() {
        let _ = make_file(get_file());
        let _ = del_file(&get_file());
        assert_eq!(path_present(&get_file()).unwrap(), false);
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
            is_string_in_file(&PathType::PathBuf(path_buf.to_path_buf()), TARGET_STRING,).unwrap(),
            true
        );

        // Test with a string that does not exist in the file
        let non_existing_target = "Non-existing line";
        assert_eq!(
            is_string_in_file(
                &PathType::PathBuf(path_buf.to_path_buf()),
                non_existing_target,
            )
            .unwrap(),
            false
        );

        // Clean up the temporary file
        del_file(&PathType::Str(tmp_file_path.into())).unwrap();
    }

    #[test]
    fn test_set_file_ownership() {
        let path = PathBuf::from("/tmp/test_set_file_ownership");
        create_test_file(&path).expect("Failed to create test file");

        let uid = Uid::current();
        let gid = Gid::current();

        match set_file_ownership(&path, uid, gid).uf_unwrap() {
            Ok(_) => {
                let metadata = fs::metadata(&path).expect("Failed to get metadata");
                let file_uid = metadata.uid();
                let file_gid = metadata.gid();
                assert_eq!(file_uid, uid.as_raw());
                assert_eq!(file_gid, gid.as_raw());
            }
            Err(e) => panic!("Failed to set file ownership: {:?}", e),
        }

        fs::remove_file(&path).expect("Failed to remove test file");
    }

    #[test]
    fn test_set_file_permission() {
        let path = PathBuf::from("/tmp/test_set_file_permission");
        create_test_file(&path).expect("Failed to create test file");

        match set_file_permission(PathType::from(path.clone()), 0o400).uf_unwrap() {
            Ok(_) => {
                let metadata = fs::metadata(&path).expect("Failed to get metadata");
                let permissions = metadata.permissions();
                assert_eq!(permissions.mode() & 0o777, 0o400);
            }
            Err(e) => panic!("Failed to set file permission: {:?}", e),
        }

        fs::remove_file(&path).expect("Failed to remove test file");
    }

    // Testing for tar and untar
    /// Helper function to create a test file with given content.
    fn create_tar_test_file(path: &PathType, file_name: &str, content: &str) {
        let file_path = path.clone().join(file_name);
        let mut file: File = fs::File::create(&file_path).unwrap();
        writeln!(file, "{}", content).unwrap();
    }

    /// Helper function to check if a file exists in a directory.
    fn file_exists_in_dir(dir: &PathType, file_name: &str) -> bool {
        dir.clone().join(file_name).exists()
    }

    #[test]
    fn test_create_tar() {
        // Create a temporary directory with test files
        let input_path = PathType::temp_dir().unwrap();

        create_tar_test_file(&input_path, "test1.txt", "This is test file 1.");
        create_tar_test_file(&input_path, "test2.txt", "This is test file 2.");

        // Create a temporary output file for the tarball
        let tar_file = input_path.to_path().join("test_archive.tar.gz");
        let tar_path = PathType::PathBuf(tar_file.clone());

        assert!(input_path.exists());

        // Call the create_tar function
        assert!(tar(&input_path, &tar_path).is_ok());

        // Check if the tar file is created
        assert!(tar_file.exists());
    }

    #[test]
    fn test_untar() {
        // Create a temporary directory for input files and output extraction
        let input_path = PathType::temp_dir().unwrap();
        let output_path = PathType::temp_dir().unwrap();

        // Create test files and tar them
        create_tar_test_file(&input_path, "test1.txt", "This is test file 1.");
        create_tar_test_file(&input_path, "test2.txt", "This is test file 2.");

        let tar_file = input_path.to_path().join("test_archive.tar.gz");
        let tar_path = PathType::PathBuf(tar_file.clone());

        tar(&input_path, &tar_path).unwrap();

        // Ensure tar file is created
        assert!(tar_file.exists());

        // Extract the tar file
        assert!(untar(&tar_path, &output_path).is_ok());

        // Verify the extracted files
        assert!(file_exists_in_dir(&output_path, "test1.txt"));
        assert!(file_exists_in_dir(&output_path, "test2.txt"));
    }

    #[test]
    fn test_create_tar_empty_folder() {
        // Create a temporary empty directory
        let input_path = PathType::temp_dir().unwrap();

        // Create a tar file path
        let tar_file = input_path.to_path().join("empty_archive.tar.gz");
        let tar_path = PathType::PathBuf(tar_file.clone());

        // Call create_tar on the empty folder
        assert!(tar(&input_path, &tar_path).is_ok());

        // Check if the tar file is created
        assert!(tar_file.exists());
    }

    #[test]
    fn test_untar_invalid_tar_file() {
        // Create a temporary directory
        let input_path = PathType::temp_dir().unwrap();

        // Create an invalid tar file
        let invalid_tar_file = input_path.to_path().join("invalid.tar.gz");
        let mut file = fs::File::create(&invalid_tar_file).unwrap();
        file.write_all(b"This is not a valid tar file").unwrap();

        let invalid_tar_path = PathType::PathBuf(invalid_tar_file.clone());
        let output_path = PathType::temp_dir().unwrap();

        // Try extracting the invalid tar file
        assert!(untar(&invalid_tar_path, &output_path).is_err());
    }
}
