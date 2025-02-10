use crate::errors;
use crate::errors::{ErrorArrayItem, WarningArrayItem, Warnings};
use crate::types::pathtype::PathType;
use crate::types::stringy::Stringy;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Read};
use std::os::unix::fs::{chown, MetadataExt};
use std::path::PathBuf;
use std::{
    fs::{self, remove_file, File},
    os::unix::prelude::PermissionsExt,
    str,
};

use errors::{OkWarning, UnifiedResult as uf};
use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use nix::unistd::{Gid, Uid};
use sha2::{Digest, Sha256};
use tar::{Archive, Builder};
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
pub fn generate_random_string(length: usize) -> uf<String> {
    let mut buffer = vec![0; length];

    let file_raw: Result<File, ErrorArrayItem> =
        File::open("/dev/urandom").map_err(|e| ErrorArrayItem::from(e));

    let mut file: File = match file_raw {
        Ok(f) => f,
        Err(e) => {
            return uf::new(Err(e));
        }
    };

    if let Err(err) = file.read_exact(&mut buffer) {
        let error_item: ErrorArrayItem = ErrorArrayItem::from(err);
        return uf::new(Err(error_item));
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
pub fn is_string_in_file<S>(file_path: &PathType, target_string: S) -> uf<bool>
where
    S: Into<String>,
    for<'a> &'a str: PartialEq<S>,
{
    let file = if let Ok(data) = open_file(file_path.clone(), false) {
        data
    } else {
        return uf::new(Ok(false));
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(line_data) => line_data,
            Err(e) => {
                return uf::new(Err(ErrorArrayItem::from(e)));
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
pub fn create_hash<S>(data: S) -> Stringy
where
    S: Into<String> + std::convert::AsRef<[u8]>,
    for<'a> &'a str: PartialEq<S>,
{
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hash: String = hex::encode(result);
    return Stringy::from(hash);
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
pub fn truncate<S>(string: S, max_chars: usize) -> Stringy
where
    S: Into<String>,
{
    let data: String = string.into();
    match data.char_indices().nth(max_chars) {
        None => Stringy::from(data),
        Some((idx, _)) => {
            let result = &data[..idx];
            Stringy::from(result)
        }
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
pub fn make_dir_perm<S>(folder_name: S, permissions: u32) -> uf<()>
where
    S: Into<String> + Clone,
{
    let permissions = fs::Permissions::from_mode(permissions);
    let file_creation_result =
        fs::create_dir(folder_name.clone().into()).map_err(|err| ErrorArrayItem::from(err));

    match file_creation_result {
        Ok(_) => {
            let set_permission = fs::set_permissions(folder_name.into(), permissions)
                .map_err(|err| ErrorArrayItem::from(err));
            match set_permission {
                Ok(_) => return uf::new(Ok(())),
                Err(e) => {
                    return uf::new(Err(e));
                }
            }
        }
        Err(e) => {
            return uf::new(Err(e));
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
/// use dusa_collection_utils::types::pathtype::PathType;
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
pub fn path_present(path: &PathType) -> uf<bool> {
    match path.to_path_buf().try_exists() {
        Ok(d) => return uf::new(Ok(d)),
        Err(e) => {
            return uf::new(Err(ErrorArrayItem::from(e)));
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
pub fn make_dir(path: &PathType) -> uf<bool> {
    match path.exists() {
        true => return uf::new(Ok(true)),
        false => match std::fs::create_dir_all(path) {
            Ok(_) => return uf::new(Ok(true)),
            Err(error) => return uf::new(Err(ErrorArrayItem::from(error))),
        },
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
pub fn remake_dir(path: &PathType, recursive: bool) -> uf<()> {
    match path.exists() {
        true => match recursive {
            true => match std::fs::remove_dir_all(path) {
                Ok(_) => return uf::new(Ok(())),
                Err(error) => return uf::new(Err(ErrorArrayItem::from(error))),
            },
            false => match std::fs::remove_dir(path) {
                Ok(_) => return uf::new(Ok(())),
                Err(error) => return uf::new(Err(ErrorArrayItem::from(error))),
            },
        },
        false => {
            return uf::new(Err(ErrorArrayItem::from(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} not found", path),
            ))))
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
/// Returns Error is file already exists
pub fn make_file(path: PathType) -> uf<()> {
    match path.exists() {
        true => {
            return uf::new(Err(ErrorArrayItem::from(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "",
            ))))
        }
        false => match File::create_new(path) {
            Ok(_) => return uf::new(Ok(())),
            Err(error) => return uf::new(Err(ErrorArrayItem::from(error))),
        },
    }
}

/// Deletes a directory RECURSIVELY.
///
/// # Arguments
///
/// * `path` - The path of the directory to delete.
///
/// # Returns
///
/// Returns `Ok(true)` if the directory is deleted successfully or if it does not exist.
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
/// This function will delete a file and ALL contents in it. USE WITH CAUTION
pub fn del_dir(file: &PathType) -> uf<()> {
    match file.exists() {
        true => match std::fs::remove_dir_all(file) {
            Ok(_) => return uf::new(Ok(())),
            Err(e) => return uf::new(Err(ErrorArrayItem::from(e))),
        },
        false => {
            return uf::new_warn(Ok(OkWarning::new_from_item(
                (),
                WarningArrayItem::new_details(
                    Warnings::Warning,
                    String::from("The file didn't exist"),
                ),
            )))
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
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn del_file(file: &PathType) -> uf<()> {
    match file.exists() {
        true => match remove_file(file) {
            Ok(_) => return uf::new(Ok(())),
            Err(error) => return uf::new(Err(ErrorArrayItem::from(error))),
        },
        false => {
            return uf::new_warn(Ok(OkWarning::new_from_item(
                (),
                WarningArrayItem::new_details(
                    Warnings::Warning,
                    String::from("The file didn't exist"),
                ),
            )))
        }
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
pub fn untar(file_path: &PathType, output_folder: &PathType) -> uf<()> {
    let tar_file: File = match open_file(file_path.clone(), false) {
        Ok(d) => d,
        Err(e) => {
            return uf::new(Err(e));
        }
    };

    let tar_reader: BufReader<File> = BufReader::new(tar_file);
    let tar: GzDecoder<BufReader<File>> = GzDecoder::new(tar_reader);
    let mut archive: Archive<GzDecoder<BufReader<File>>> = Archive::new(tar);

    match archive.unpack(output_folder) {
        Ok(_) => uf::new(Ok(())),
        Err(e) => {
            return uf::new(Err(ErrorArrayItem::from(e)));
        }
    }
}

/// Creates a tar.gz file from the specified input folder and saves it to the given file path.
///
/// # Arguments
///
/// * `input_folder` - The path of the folder whose contents will be archived.
/// * `output_file_path` - The path where the tar.gz file will be created.
///
/// # Returns
///
/// Returns `Ok(())` if the creation is successful.
/// Returns an error of type `ErrorArrayItem` if there is any issue encountered during the process.
pub fn tar(input_folder: &PathType, output_file_path: &PathType) -> uf<()> {
    let output_file = match OpenOptions::new()
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .truncate(true) // Truncate the file if it exists
        .open(output_file_path.clone())
    {
        Ok(file) => file,
        Err(e) => {
            return uf::new(Err(e.into()));
        }
    };

    let output_writer: BufWriter<File> = BufWriter::new(output_file);
    let encoder: GzEncoder<BufWriter<File>> = GzEncoder::new(output_writer, Compression::default());
    let mut tar_builder: Builder<GzEncoder<BufWriter<File>>> = Builder::new(encoder);

    match tar_builder.append_dir_all(".", input_folder.clone()) {
        Ok(_) => uf::new(Ok(())),
        Err(e) => {
            return uf::new(Err(ErrorArrayItem::from(e)));
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
/// match set_file_ownership(&path, uid, gid).uf_unwrap() {
///     Ok(_) => println!("Ownership set successfully"),
///     Err(e) => eprintln!("Failed to set ownership: {:?}", e),
/// }
/// ```
pub fn set_file_ownership(path: &PathBuf, uid: Uid, gid: Gid) -> uf<()> {
    if let Err(err) = chown(path, Some(uid.into()), Some(gid.into())) {
        return uf::new(Err(ErrorArrayItem::from(err)));
    };

    uf::new(Ok(()))
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
/// use dusa_collection_utils::types::pathtype::PathType;
///
/// let socket_path = PathType::from("/path/to/socket");
///
/// match set_file_permission(socket_path, 0o777).uf_unwrap() {
///     Ok(_) => println!("Permissions set successfully"),
///     Err(e) => eprintln!("Failed to set permissions: {:?}", e),
/// }
/// ```
pub fn set_file_permission(socket_path: PathType, permissions: u32) -> uf<()> {
    // Changing the permissions of the socket
    let socket_metadata = match fs::metadata(socket_path.clone()) {
        Ok(d) => d,
        Err(e) => return uf::new(Err(ErrorArrayItem::from(e))),
    };

    let mut current_permissions = socket_metadata.permissions();
    current_permissions.set_mode(permissions); // Set desired permissions

    if let Err(err) = fs::set_permissions(socket_path.clone(), current_permissions) {
        return uf::new(Err(ErrorArrayItem::from(err)));
    }

    uf::new(Ok(()))
}

/// Retrieves the current Unix timestamp in seconds.
pub fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}
