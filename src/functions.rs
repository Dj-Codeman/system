use crate::errors::{ErrorArray, ErrorArrayItem, Errors, WarningArray, WarningArrayItem, Warnings};
use crate::{errors, types};
use std::io::{BufRead, BufReader, Read};
use std::os::unix::fs::{chown, MetadataExt};
use std::{
    fs::{self, remove_file, File},
    os::unix::prelude::PermissionsExt,
    str,
};

use errors::{OkWarning, UnifiedResult as uf};
use flate2::bufread::GzDecoder;
use sha2::{Digest, Sha256};
use tar::Archive;
use types::{ClonePath, CopyPath, PathType};
use walkdir::WalkDir;

/// Generates a random string of the specified length using alphanumeric characters.
///
/// # Arguments
///
/// * `length` - The length of the random string to generate.
///
/// # Returns
///
/// A random string of the specified length.
pub fn generate_random_string(length: usize, error_vec: ErrorArray) -> uf<String> {
    let mut errors = error_vec.0.write().unwrap();
    let mut file: File = File::open("/dev/urandom").map_err(|e| {
        let err = ErrorArrayItem::from(e);
        errors.push(err);
        return uf::new(Err(error_vec.clone()));
    })?;

    let mut buffer = vec![0; length];

    file.read_exact(&mut buffer).map_err(|_e| {
        errors.push(ErrorArrayItem::new(
            Errors::ReadingFile,
            String::from("Error while reading from /dev/urandom"),
        ));
        return uf::new(Err(error_vec.clone()));
    })?;

    uf::new(Ok(buffer
        .iter()
        .map(|&x| (x % 26 + 97) as u8 as char)
        .collect::<String>()))
}

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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
// pub fn is_string_in_file(file_path: &PathType, target_string: &str) -> Result<bool, ErrorArrayItem> {
pub fn is_string_in_file(
    file_path: &PathType,
    target_string: &str,
    mut errors: ErrorArray,
) -> uf<bool> {
    let file: File = open_file(file_path.clone_path(), errors.clone()).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(d) => d,
            Err(e) => {
                let err = ErrorArrayItem::from(e);
                errors.push(err);
                return uf::new(Err(errors.clone()));
            }
        };

        // Check if the current line is equal to the target string
        if line.trim() == target_string {
            return uf::new(Ok(true)); // Found a match
        }
    }

    uf::new(Ok(false)) // Target string not found in the file
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn make_dir_perm(folder_name: &str, permissions: u32, mut errors: ErrorArray) -> uf<()> {
    let permissions = fs::Permissions::from_mode(permissions);

    uf::new(Ok(fs::create_dir(folder_name).map_err(|err| {
        let err: ErrorArrayItem = ErrorArrayItem::new(
            Errors::CreatingDirectory,
            format!("Error creating folder: {}", err),
        );
        errors.push(err);

        fs::set_permissions(folder_name, permissions).map_err(|err| {
            let err: ErrorArrayItem = ErrorArrayItem::new(
                Errors::CreatingDirectory,
                format!("Error creating folder: {}", err),
            );
            errors.push(err);
            return uf::new(Err(errors.clone()));
        })?;

        return uf::new(Err(errors));
    })?))
}

/// Recursively changes ownership of all files and directories in the given directory.
///
/// # Arguments
///
/// * `dir` - A path to the directory whose contents will have their ownership changed.
/// * `uid` - An optional new UID (user ID) to set for the files and directories. If `None`, the UID
///           of the files and directories will not be changed.
/// * `gid` - An optional new GID (group ID) to set for the files and directories. If `None`, the GID
///           of the files and directories will not be changed.
///
/// # Errors
///
/// This function returns an error if there are any issues traversing the directory or changing
/// ownership of its contents.
///
/// # Example
///
///
/// use std::io;
///
/// fn main() -> Result<(), io::Error> {
///     # Apply chown recursively to /path/to/directory with UID 1000 and GID 1000
///     chown_recursive("/path/to/directory", Some(1000), Some(1000))?;
///     Ok(())
/// }
///
pub fn chown_recursive(
    dir: PathType,
    uid: Option<u32>,
    gid: Option<u32>,
) -> Result<(), ErrorArrayItem> {
    let needed_type = dir.to_path_buf();
    for entry in WalkDir::new(needed_type.as_path()).follow_links(false) {
        let entry = entry?;
        let path = entry.path();

        // Retrieve metadata of the file/directory
        let metadata = fs::metadata(&path)?;

        // Change ownership if it's a file or directory
        if metadata.is_file() || metadata.is_dir() {
            // Set new ownership using the `chown` function
            match (uid, gid) {
                (Some(uid), Some(gid)) => {
                    chown(&path, Some(uid), Some(gid))?;
                }
                (Some(uid), None) => {
                    chown(&path, Some(uid), Some(metadata.permissions().mode()))?;
                }
                (None, Some(gid)) => {
                    chown(&path, Some(metadata.uid()), Some(gid))?;
                }
                _ => {}
            }
        }
    }
    Ok(())
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
#[deprecated(since = "0.0.1", note = "please use `path_present` instead")]
pub fn is_path(path: &str) -> uf<bool> {
    if std::path::Path::new(path).exists() {
        return uf::new(Ok(true));
    } else {
        return uf::new(Ok(false));
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn path_present(path: &PathType, mut errors: ErrorArray) -> uf<bool> {
    match path.to_path_buf().try_exists() {
        Ok(d) => return uf::new(Ok(d)),
        Err(e) => {
            let err = ErrorArrayItem::from(e);
            errors.push(err);
            return uf::new(Err(errors));
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn make_dir(path: PathType, mut errors: ErrorArray) -> uf<bool> {
    match path_present(&path, errors.clone()).uf_unwrap() {
        Ok(d) => match d {
            true => return uf::new(Ok(true)),
            false => match std::fs::create_dir_all(path.copy_path()) {
                Ok(_) => return uf::new(Ok(true)),
                Err(e) => {
                    let err = ErrorArrayItem::new(Errors::CreatingDirectory, e.to_string());
                    errors.push(err);
                    return uf::new(Err(errors));
                }
            },
        },
        Err(e) => return uf::new(Err(e)),
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn remake_dir(path: PathType, errors: ErrorArray) -> uf<()> {
    match del_dir(&path, errors.clone()).uf_unwrap() {
        Ok(_) => match make_dir(path, errors).uf_unwrap() {
            Ok(_) => return uf::new(Ok(())),
            Err(e) => return uf::new(Err(e)),
        },
        Err(e) => {
            return uf::new(Err(e));
        }
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn make_file(path: PathType, mut errors: ErrorArray) -> uf<bool> {
    match path_present(&path, errors.clone()).uf_unwrap() {
        Ok(d) => match d {
            true => return uf::new(Ok(false)), // This will fail since we did not create a new file
            false => match File::create(path.to_path_buf()) {
                Ok(_) => return uf::new(Ok(true)),
                Err(e) => {
                    errors.push(ErrorArrayItem::from(e));
                    return uf::new(Err(errors));
                }
            },
        },
        Err(e) => return uf::new(Err(e)),
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn del_dir(path: &PathType, mut errors: ErrorArray) -> uf<bool> {
    match path_present(&path, errors.clone()).uf_unwrap() {
        Ok(d) => match d {
            true => match std::fs::remove_dir_all(path.to_path_buf()) {
                Ok(_) => return uf::new(Ok(true)),
                Err(e) => {
                    errors.push(ErrorArrayItem::from(e));
                    return uf::new(Err(errors));
                }
            },
            false => return uf::new(Ok(true)),
        },
        Err(e) => return uf::new(Err(e)),
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn del_file(path: PathType, mut errors: ErrorArray, mut warnings: WarningArray) -> uf<()> {
    // std::fs::read(path)?
    match path_present(&path, errors.clone()).uf_unwrap() {
        Ok(b) => match b {
            true => match remove_file(path.to_path_buf()) {
                Ok(_) => return uf::new(Ok(())),
                Err(e) => {
                    errors.push(ErrorArrayItem::from(e));
                    return uf::new(Err(errors));
                }
            },
            false => {
                warnings.push(WarningArrayItem::new(Warnings::FileNotDeleted));
                return uf::new_warn(Ok(OkWarning{
                data: (),
                warning: warnings})) // If the file never existed in the first place
            }
        },
        Err(e) => return uf::new(Err(e)),
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn untar(file_path: &PathType, output_folder: &str, mut errors: ErrorArray) -> uf<()> {
    let tar_file: File = open_file(file_path.clone_path(), errors.clone()).unwrap();
    let tar_reader: BufReader<File> = BufReader::new(tar_file);
    let tar: GzDecoder<BufReader<File>> = GzDecoder::new(tar_reader);
    let mut archive: Archive<GzDecoder<BufReader<File>>> = Archive::new(tar);

    match archive.unpack(output_folder) {
        Ok(_) => uf::new(Ok(())),
        Err(e) => {
            errors.push(ErrorArrayItem::from(e));
            return uf::new(Err(errors));
        }
    }
}

fn open_file(file: PathType, mut errors: ErrorArray) -> uf<File> {
    let file: File = File::open(file.to_path_buf()).map_err(|e| {
        let err = errors::ErrorArrayItem::from(e);
        errors.push(err);
        return uf::new(Err(errors.clone()));
    })?;
    return uf::new(Ok(file));
}

// #[cfg(test)]
// mod tests {
//     use super::generate_random_string;
//     use super::*;
//     use std::path::PathBuf;

//     fn get_path() -> PathType {
//         let name = create_hash("dummy_test".to_string());
//         let mut path = String::new();
//         path.push_str("/tmp/");
//         path.push_str(&name);
//         return PathType::Content(path);
//     }

//     fn get_file() -> PathType {
//         let name = create_hash("dummy_test".to_string());
//         let mut path = String::new();
//         path.push_str("/tmp/");
//         path.push_str(&name);
//         path.push_str(".file");
//         return PathType::Content(path);
//     }

//     #[test]
//     fn random_string() {
//         let test_string: String = generate_random_string(10).unwrap();
//         assert_eq!(test_string.len(), 10);
//     }

//     #[test]
//     fn trimming() {
//         let result = truncate("Hello, World", 5);
//         assert_eq!(result, "Hello");
//     }

//     #[test]
//     #[allow(deprecated)]
//     fn ispath_test() {
//         let result = is_path("/tmp/definitely_real_path");
//         assert_eq!(result, UnifiedResult::new(Ok(false)));
//     }

//     #[test]
//     fn path_present_test() {
//         let path: &PathType = &PathType::Str("/tmp/definitely_real_path".into());
//         let result: bool = path_present(path).unwrap();
//         assert_eq!(result, false)
//     }

//     #[test]
//     fn hash() {
//         let result = create_hash("hash".to_string());
//         assert_eq!(
//             result,
//             "d04b98f48e8f8bcc15c6ae5ac050801cd6dcfd428fb5f9e65c4e16e7807340fa".to_string()
//         );
//     }

//     #[test]
//     fn create_dir() {
//         let result = make_dir(get_path()).unwrap();
//         assert_eq!(result, true);
//     }

//     #[test]
//     fn destroy_dir() {
//         make_dir(get_path()).unwrap();
//         let result = del_dir(&get_path()).unwrap();
//         assert_eq!(result, true);
//     }

//     #[test]
//     fn create_file() {
//         let result: bool = make_file(get_file()).unwrap();
//         assert_eq!(result, true);
//     }

//     #[test]
//     fn delete_file() {
//         let _ = make_file(get_file());
//         let _ = del_file(get_file());
//         assert_eq!(path_present(&get_file()).unwrap(), false);
//     }

//     #[test]
//     fn test_is_string_in_file() {
//         use std::io::Write;
//         // Create a temporary file for testing
//         let tmp_file_path = "test_file.txt";
//         let mut file = File::create(tmp_file_path).unwrap();
//         writeln!(file, "Line 1").unwrap();
//         writeln!(file, "Line 2").unwrap();
//         writeln!(file, "Line 3").unwrap();

//         // Test with a string that exists in the file
//         let path_buf = PathType::Str(tmp_file_path.into());
//         let target_string = "Line 2";
//         assert_eq!(
//             is_string_in_file(&PathType::PathBuf(path_buf.to_path_buf()), target_string).unwrap(),
//             true
//         );

//         // Test with a string that does not exist in the file
//         let non_existing_target = "Non-existing line";
//         assert_eq!(
//             is_string_in_file(
//                 &PathType::PathBuf(path_buf.to_path_buf()),
//                 non_existing_target
//             )
//             .unwrap(),
//             false
//         );

//         // Test with a file that does not exist
//         let non_existing_file: &str = "non_existing_file.txt";
//         assert_eq!(
//             is_string_in_file(
//                 &PathType::PathBuf(PathBuf::from(non_existing_file)),
//                 target_string
//             ),
//             Err(ErrorArrayItem::new_details(
//                 errors_dep::SystemErrorType::ErrorOpeningFile,
//                 "No such file or directory (os error 2)"
//             ))
//         );

//         // Clean up the temporary file
//         del_file(PathType::Str(tmp_file_path.into())).unwrap();
//     }
// }