use crate::errors::{ErrorArray, ErrorArrayItem, Errors, WarningArray, WarningArrayItem, Warnings};
use crate::{errors, types};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::fs::{chown, MetadataExt};
use std::path::PathBuf;
use std::{
    fs::{self, remove_file, File},
    os::unix::prelude::PermissionsExt,
    str,
};

use errors::{OkWarning, UnifiedResult as uf};
use flate2::bufread::GzDecoder;
use nix::unistd::{Gid, Uid};
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
pub fn generate_random_string(length: usize, mut errors: ErrorArray) -> uf<String> {
    let mut buffer = vec![0; length];

    let file_raw: Result<File, ErrorArrayItem> =
        File::open("/dev/urandom").map_err(|e| ErrorArrayItem::from(e));

    let mut file: File = match file_raw {
        Ok(f) => f,
        Err(e) => {
            errors.push(e);
            return uf::new(Err(errors));
        }
    };

    let _ = file.read_exact(&mut buffer).map_err(|e| {
        errors.push(ErrorArrayItem::from(e));
    });

    if errors.len() > 0 {
        return uf::new(Err(errors.clone()));
    }

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
pub fn is_string_in_file(
    file_path: &PathType,
    target_string: &str,
    mut errors: ErrorArray,
) -> uf<bool> {
    let file = if let Ok(data) = open_file(file_path.clone_path(), false) {
        data
    } else {
        return uf::new(Ok(false));
    };

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
    let file_creation_result = fs::create_dir(folder_name).map_err(|err| ErrorArrayItem::from(err));

    match file_creation_result {
        Ok(_) => {
            let set_permission = fs::set_permissions(folder_name, permissions)
                .map_err(|err| ErrorArrayItem::from(err));
            match set_permission {
                Ok(_) => return uf::new(Ok(())),
                Err(e) => {
                    errors.push(ErrorArrayItem::from(e));
                    return uf::new(Err(errors));
                }
            }
        }
        Err(e) => {
            errors.push(e);
            return uf::new(Err(errors));
        }
    }
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
/// ```rust
/// use std::io;
/// use dusa_collection_utils::functions::chown_recursive;
/// use dusa_collection_utils::types::PathType;
///
/// fn main() -> Result<(), io::Error> {
///     let path = PathType::Content(String::from("/tmp/file"));
///     chown_recursive(path, Some(1000), Some(1000)); // Apply chown recursively to /path/to/directory with UID 1000 and GID 1000
///     Ok(())
/// }
///```
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
            errors.push(ErrorArrayItem::from(e));
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
pub fn make_dir(path: &PathType, mut errors: ErrorArray) -> uf<bool> {
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
pub fn remake_dir(path: &PathType, errors: ErrorArray) -> uf<()> {
    match del_dir(path, errors.clone()).uf_unwrap() {
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
                return uf::new_warn(Ok(OkWarning {
                    data: (),
                    warning: warnings,
                })); // If the file never existed in the first place
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
#[allow(deprecated)]
pub fn untar(file_path: &PathType, output_folder: &str, mut errors: ErrorArray) -> uf<()> {
    let tar_file: File = match open_file(file_path.clone_path(), false) {
        Ok(d) => d,
        Err(e) => {
            errors.push(e);
            return uf::new(Err(errors));
        }
    };

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

/// Opens a file.
///
/// # Arguments
///
/// * `path` - The path of the file to delete.
///
/// # Returns
/// Returns `Ok(file)` if the file exists and can be opened.
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn open_file(file: PathType, create: bool) -> Result<File, ErrorArrayItem> {
    let file_path = file.canonicalize().map_err(|err| ErrorArrayItem::from(err));

    let file_result = OpenOptions::new()
        .read(true) // Open file with read
        .write(true) // Open file with write
        .append(true)
        .create(create)
        .open(file_path?)
        .map_err(|err| ErrorArrayItem::from(err));

    return file_result;
}

/// Sets the ownership of a file or directory to the specified user and group.
///
/// # Arguments
///
/// * `path` - A reference to a `PathBuf` that specifies the path to the file or directory.
/// * `uid` - The user ID to set as the owner of the file or directory.
/// * `gid` - The group ID to set as the group of the file or directory.
///
/// # Returns
///
/// * `Result<(), ErrorArrayItem>` - Returns `Ok(())` if the ownership was successfully set.
///   Returns an `ErrorArrayItem` if an error occurred while setting the ownership.
///
/// # Errors
///
/// This function will return an `ErrorArrayItem` if the `chown` system call fails.
///
/// # Example
///
/// ```rust
/// use std::path::PathBuf;
/// use nix::unistd::{Uid, Gid};
/// use dusa_collection_utils::functions::set_file_ownership;
///
/// let path = PathBuf::from("/path/to/file");
/// let uid = Uid::from_raw(1000); // example user ID
/// let gid = Gid::from_raw(1000); // example group ID
///
/// match set_file_ownership(&path, uid, gid) {
///     Ok(_) => println!("Ownership set successfully"),
///     Err(e) => eprintln!("Failed to set ownership: {:?}", e),
/// }
/// ```
pub fn set_file_ownership(path: &PathBuf, uid: Uid, gid: Gid) -> Result<(), ErrorArrayItem> {
    if let Err(err) = chown(path, Some(uid.into()), Some(gid.into())) {
        return Err(ErrorArrayItem::from(err));
    };

    Ok(())
}

/// Sets the permissions of a socket file to read and write for the owner and group.
///
/// # Arguments
///
/// * `socket_path` - The path to the socket file as a `PathType`.
///
/// # Returns
///
/// * `Result<(), ErrorArrayItem>` - Returns `Ok(())` if the permissions were successfully set.
///   Returns an `ErrorArrayItem` if an error occurred while setting the permissions.
///
/// # Errors
///
/// This function will return an `ErrorArrayItem` if the `metadata` or `set_permissions`
/// calls from the `fs` module fail.
///
/// # Example
///
/// ```rust
/// use std::path::PathBuf;
/// use dusa_collection_utils::functions::set_file_permission;
/// use dusa_collection_utils::types::PathType;
///
/// let socket_path = PathType::from("/path/to/socket");
///
/// match set_file_permission(socket_path) {
///     Ok(_) => println!("Permissions set successfully"),
///     Err(e) => eprintln!("Failed to set permissions: {:?}", e),
/// }
/// ```
pub fn set_file_permission(socket_path: PathType) -> Result<(), ErrorArrayItem> {
    // Changing the permissions of the socket
    let socket_metadata = match fs::metadata(socket_path.clone()) {
        Ok(d) => d,
        Err(e) => return Err(ErrorArrayItem::from(e)),
    };

    let mut permissions = socket_metadata.permissions();
    permissions.set_mode(0o660); // Set desired permissions

    if let Err(err) = fs::set_permissions(socket_path.clone(), permissions) {
        return Err(ErrorArrayItem::from(err));
    }

    Ok(())
}

#[cfg(rust_comp_feature = "try_trait_v2")]
mod tests {
    #[test]
    fn try_trait() {
        assert_eq!(true, true)
    }
}
