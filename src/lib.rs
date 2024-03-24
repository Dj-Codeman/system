mod custom_type;
pub mod errors;

use custom_type::{CopyPath, PathType};
use errors::SystemError;
use flate2::bufread::GzDecoder;
use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader};
use std::{
    fs::{self, remove_file, File},
    os::unix::prelude::PermissionsExt,
    str,
};
use tar::Archive;

/// Checking if file contains a specific string.
///
/// # Arguments
///
/// * `file_path` - The path to the file to be searched.
/// * `target_string` - The string to search for in the file.
///
/// # Returns
///
/// Returns `Ok(true)` if the target string is found, otherwise `Ok(false)`.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn is_string_in_file(file_path: &PathType, target_string: &str) -> Result<bool, SystemError> {
    let file: File = match File::open(file_path) {
        Ok(d) => d,
        Err(e) => {
            let data = e.to_string();
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorOpeningFile,
                &data,
            ));
        }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(d) => d,
            Err(_) => return Err(SystemError::new(errors::SystemErrorType::ErrorReadingFile)),
        }; // Panics if there's an error reading a line

        // Check if the current line is equal to the target string
        if line.trim() == target_string {
            return Ok(true); // Found a match
        }
    }

    Ok(false) // Target string not found in the file
}


/// Create a 256-bit hash for the given data.
///
/// # Arguments
///
/// * `data` - The data for which the hash will be generated.
///
/// # Returns
///
/// Returns the generated hash as a hexadecimal string.
pub fn create_hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hash: String = hex::encode(result);
    return hash;
    // 256 because its responsible for generating the writing keys
}


/// Trims a string to a maximum number of characters.
///
/// # Arguments
///
/// * `s` - The string to be truncated.
/// * `max_chars` - The maximum number of characters allowed in the truncated string.
///
/// # Returns
///
/// Returns the truncated string.
pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

/// Creates a directory with the specified permissions.
///
/// # Arguments
///
/// * `folder_name` - The name of the folder to be created.
/// * `permissions` - The permissions to be set for the folder.
///
/// # Returns
///
/// Returns `Ok(())` if the folder creation and permission setting are successful.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn make_dir_perm(folder_name: &str, permissions: u32) -> Result<(), SystemError> {
    let permissions = fs::Permissions::from_mode(permissions);

    fs::create_dir(folder_name)
        .map_err(|err| {
            SystemError::new_details(
                errors::SystemErrorType::ErrorCreatingDir,
                &format!("Error creating folder: {}", err),
            )
        })
        .and_then(|()| {
            fs::set_permissions(folder_name, permissions).map_err(|err| {
                SystemError::new_details(
                    errors::SystemErrorType::ErrorSettingPermDir,
                    &format!("Error setting permissions: {}", err),
                )
            })
        })
}


/// Deprecated function. Use `path_present` instead.
///
/// # Arguments
///
/// * `path` - The path to check for existence.
///
/// # Returns
///
/// Returns `true` if the path exists, otherwise `false`.
#[deprecated(since = "0.1.0", note = "please use `path_present` instead")]
pub fn is_path(path: &str) -> bool {
    if std::path::Path::new(path).exists() {
        return true;
    } else {
        return false;
    }
}

/// Checks if a path exists.
///
/// # Arguments
///
/// * `path` - The path to check for existence.
///
/// # Returns
///
/// Returns `Ok(true)` if the path exists, otherwise `Ok(false)`.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn path_present(path: &PathType) -> Result<bool, SystemError> {
    match path.to_path_buf().try_exists() {
        Ok(d) => return Ok(d),
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorReadingFile,
                &e.to_string(),
            ))
        }
    }
}


/// Creates a directory if it does not exist.
///
/// # Arguments
///
/// * `path` - The path of the directory to create.
///
/// # Returns
///
/// Returns `Ok(true)` if the directory is created successfully or if it already exists.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn make_dir(path: PathType) -> Result<bool, SystemError> {
    match path_present(&path) {
        Ok(d) => match d {
            true => return Ok(true),
            false => match std::fs::create_dir_all(path.copy_path()) {
                Ok(_) => return Ok(true),
                Err(e) => {
                    return Err(SystemError::new_details(
                        errors::SystemErrorType::ErrorCreatingDir,
                        &e.to_string(),
                    ))
                }
            },
        },
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorCreatingDir,
                &e.to_string(),
            ))
        }
    }
}


/// Recreates a directory by first deleting it if it exists and then creating it.
///
/// # Arguments
///
/// * `path` - The path of the directory to recreate.
///
/// # Returns
///
/// Returns `Ok(())` if the directory is recreated successfully.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn remake_dir(path: PathType) -> Result<(), SystemError> {
    match del_dir(&path) {
        Ok(_) => match make_dir(path) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}


/// Creates a file if it does not exist.
///
/// # Arguments
///
/// * `path` - The path of the file to create.
///
/// # Returns
///
/// Returns `Ok(true)` if the file is created successfully.
/// Returns `Ok(false)` if the file already exists.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn make_file(path: PathType) -> Result<bool, SystemError> {
    match path_present(&path) {
        Ok(d) => match d {
            true => return Ok(false), // This will fail since we did not create a new file
            false => match File::create(path.to_path_buf()) {
                Ok(_) => return Ok(true),
                Err(e) => {
                    return Err(SystemError::new_details(
                        errors::SystemErrorType::ErrorCreatingFile,
                        &e.to_string(),
                    ))
                }
            },
        },
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorCreatingFile,
                &e.to_string(),
            ))
        }
    }
}


/// Deletes a directory.
///
/// # Arguments
///
/// * `path` - The path of the directory to delete.
///
/// # Returns
///
/// Returns `Ok(true)` if the directory is deleted successfully or if it does not exist.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn del_dir(path: &PathType) -> Result<bool, SystemError> {
    match path_present(&path) {
        Ok(d) => match d {
            true => match std::fs::remove_dir_all(path.to_path_buf()) {
                Ok(_) => return Ok(true),
                Err(e) => {
                    return Err(SystemError::new_details(
                        errors::SystemErrorType::ErrorDeletingDir,
                        &e.to_string(),
                    ))
                }
            },
            false => return Ok(true),
        },
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorDeletingDir,
                &e.to_string(),
            ))
        }
    }
}


/// Deletes a file.
///
/// # Arguments
///
/// * `path` - The path of the file to delete.
///
/// # Returns
///
/// Returns `Ok(())` if the file is deleted successfully or if it does not exist.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn del_file(path: PathType) -> Result<(), SystemError> {
    // std::fs::read(path)?
    match path_present(&path) {
        Ok(b) => match b {
            true => match remove_file(path.to_path_buf()) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    return Err(SystemError::new_details(
                        errors::SystemErrorType::ErrorDeletingFile,
                        &e.to_string(),
                    ))
                }
            },
            false => return Ok(()), // If the file never existed in the first place
        },
        Err(e) => return Err(e),
    }
}


/// Extracts the contents of a tar.gz file to a specified output folder.
///
/// # Arguments
///
/// * `file_path` - The path of the tar.gz file to extract.
/// * `output_folder` - The path of the folder where the contents will be extracted.
///
/// # Returns
///
/// Returns `Ok(())` if the extraction is successful.
/// Returns an error of type `SystemError` if there is any issue encountered during the process.
pub fn untar(file_path: &PathType, output_folder: &str) -> Result<(), SystemError> {
    let tar_file: File = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorOpeningFile,
                &e.to_string(),
            ))
        }
    };

    let tar_reader: BufReader<File> = BufReader::new(tar_file);
    let tar: GzDecoder<BufReader<File>> = GzDecoder::new(tar_reader);
    let mut archive: Archive<GzDecoder<BufReader<File>>> = Archive::new(tar);

    match archive.unpack(output_folder) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorUntaringFile,
                &e.to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn get_path() -> PathType {
        let name = create_hash("dummy_test".to_string());
        let mut path = String::new();
        path.push_str("/tmp/");
        path.push_str(&name);
        return PathType::Content(path);
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
    fn trimming() {
        let result = truncate("Hello, World", 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    #[allow(deprecated)]
    fn ispath_test() {
        let result = is_path("/tmp/definatly_real_path");
        assert_eq!(result, false);
    }

    #[test]
    fn path_present_test() {
        let path: &PathType = &PathType::Str("/tmp/definatly_real_path".into());
        let result: bool = path_present(path).unwrap();
        assert_eq!(result, false)
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
        let result = make_dir(get_path()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn destroy_dir() {
        make_dir(get_path()).unwrap();
        let result = del_dir(&get_path()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn create_file() {
        let result: bool = make_file(get_file()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn delete_file() {
        let _ = make_file(get_file());
        let _ = del_file(get_file());
        assert_eq!(path_present(&get_file()).unwrap(), false);
    }

    #[test]
    fn test_is_string_in_file() {
        use std::io::Write;
        // Create a temporary file for testing
        let tmp_file_path = "test_file.txt";
        let mut file = File::create(tmp_file_path).unwrap();
        writeln!(file, "Line 1").unwrap();
        writeln!(file, "Line 2").unwrap();
        writeln!(file, "Line 3").unwrap();

        // Test with a string that exists in the file
        let path_buf = PathType::Str(tmp_file_path.into());
        let target_string = "Line 2";
        assert_eq!(
            is_string_in_file(&PathType::PathBuf(path_buf.to_path_buf()), target_string).unwrap(),
            true
        );

        // Test with a string that does not exist in the file
        let non_existing_target = "Non-existing line";
        assert_eq!(
            is_string_in_file(&PathType::PathBuf(path_buf.to_path_buf()), non_existing_target).unwrap(),
            false
        );

        // Test with a file that does not exist
        let non_existing_file: &str = "non_existing_file.txt";
        assert_eq!(
            is_string_in_file(
                &PathType::PathBuf(PathBuf::from(non_existing_file)),
                target_string
            ),
            Err(SystemError::new_details(
                errors::SystemErrorType::ErrorOpeningFile,
                "No such file or directory (os error 2)"
            ))
        );

        // Clean up the temporary file
        del_file(PathType::Str(tmp_file_path.into())).unwrap();
    }
}
