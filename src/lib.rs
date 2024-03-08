pub mod errors;

use errors::SystemError;
use flate2::bufread::GzDecoder;
use sha2::{Digest, Sha256};
use tar::Archive;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{
    fs::{self, remove_file, File},
    os::unix::prelude::PermissionsExt,
    str,
};

/// Checking if file contains string
pub fn is_string_in_file(file_path: &str, target_string: &str) -> Result<bool, SystemError> {
    let file = match File::open(file_path) {
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

/// Create 256 bit hash
pub fn create_hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hash: String = hex::encode(result);
    return hash;
    // 256 because its responsible for generating the writing keys
}

/// Trimming the size of &str from the end
pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

/// Folder manipulation
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

#[deprecated(since="0.1.0", note="please use `path_present` instead")]
pub fn is_path(path: &str) -> bool {
    if std::path::Path::new(path).exists() {
        return true;
    } else {
        return false;
    }
}

pub fn path_present(path: PathBuf) -> Result<bool, SystemError> {
    match path.try_exists() {
        Ok(d) => return Ok(d),
        Err(e) => return Err(SystemError::new_details(errors::SystemErrorType::ErrorReadingFile, &e.to_string())),
    }
}

pub fn make_dir(path: &str) -> Result<bool, SystemError> {
    match path_present(PathBuf::from(path)) {
        Ok(d) => match d {
            true => return Ok(true),
            false => match std::fs::create_dir_all(path){
                Ok(_) => return Ok(true),
                Err(e) => return Err(SystemError::new_details(errors::SystemErrorType::ErrorCreatingDir, &e.to_string())),
            },
        },
        Err(e) => return Err(SystemError::new_details(errors::SystemErrorType::ErrorCreatingDir, &e.to_string())),
    }
}

pub fn remake_dir(path: &str) -> Result<(), SystemError> {
    match del_dir(path) {
        Ok(_) => match make_dir(path) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}

pub fn make_file(path: &str) -> Result<bool, SystemError> {
    match path_present(PathBuf::from(path)) {
        Ok(d) => match d {
            true => return Ok(false), // This will fail since we did not create a new file
            false => match File::create(path) {
                Ok(_) => return Ok(true),
                Err(e) => return Err(SystemError::new_details(
                    errors::SystemErrorType::ErrorCreatingFile,
                    &e.to_string(),
                )),
            },
        },
        Err(e) => return Err(SystemError::new_details(
            errors::SystemErrorType::ErrorCreatingFile,
            &e.to_string(),
        )),
    }
}

pub fn del_dir(path: &str) -> Result<bool, SystemError> {
    match path_present(PathBuf::from(path)) {
        Ok(d) => match d {
            true => match std::fs::remove_dir_all(path) {
                Ok(_) => return Ok(true),
                Err(e) => return Err(SystemError::new_details(
                    errors::SystemErrorType::ErrorDeletingDir,
                    &e.to_string(),
                )),
            },
            false => return Ok(true),
        },
        Err(e) => return Err(SystemError::new_details(
            errors::SystemErrorType::ErrorDeletingDir,
            &e.to_string(),
        )),
    }
}

pub fn del_file(path: &str) -> Result<(), SystemError> {
    match remove_file(path) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorDeletingFile,
                &e.to_string(),
            ))
        }
    }
}

pub fn untar(file_path: &str, output_folder: &str) -> Result<(), SystemError> {
    let tar_file: File = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(SystemError::new_details(
                errors::SystemErrorType::ErrorOpeningFile,
                &e.to_string(),
            ))
        }
    };

    let tar_reader = BufReader::new(tar_file);
    let tar = GzDecoder::new(tar_reader);
    let mut archive = Archive::new(tar);

    match archive.unpack(output_folder) {
        Ok(_) => Ok(()),
        Err(e) => return Err(SystemError::new_details(
            errors::SystemErrorType::ErrorUntaringFile,
            &e.to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_path() -> String {
        let name = create_hash("dummy_test".to_string());
        let mut path = String::new();
        path.push_str("/tmp/");
        path.push_str(&name);
        return path;
    }

    fn get_file() -> String {
        let name = create_hash("dummy_test".to_string());
        let mut path = String::new();
        path.push_str("/tmp/");
        path.push_str(&name);
        path.push_str(".file");
        return path;
    }

    #[test]
    fn trimming() {
        let result = truncate("Hello, World", 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn ispath_test() {
        let result = is_path("/tmp/definatly_real_path");
        assert_eq!(result, false);
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
        let result = make_dir(&get_path()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn destroy_dir() {
        let _ = make_dir(&get_path());
        let result = del_dir(&get_path()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn create_file() {
        let result = make_file(&get_file());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn delete_file() {
        let _ = make_file(&get_file());
        let _ = del_file(&get_file());
        assert_eq!(is_path(&get_file()), false);
    }

}
