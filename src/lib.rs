use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader};
use std::{
    fs::{self, remove_file, File},
    os::unix::prelude::PermissionsExt,
    str,
};
use zip::ZipArchive;

/// Checking if file contains string
pub fn is_string_in_file(file_path: &str, target_string: &str) -> bool {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap(); // Panics if there's an error reading a line

        // Check if the current line is equal to the target string
        if line.trim() == target_string {
            return true; // Found a match
        }
    }

    false // Target string not found in the file
}

/// Create 256 bit hash
pub fn create_hash(data: &String) -> String {
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
pub fn make_dir_perm(folder_name: &str, permissions: u32) -> Result<(), String> {
    let permissions = fs::Permissions::from_mode(permissions);

    fs::create_dir(folder_name)
        .map_err(|err| format!("Error creating folder: {}", err))
        .and_then(|()| {
            fs::set_permissions(folder_name, permissions)
                .map_err(|err| format!("Error setting permissions: {}", err))
        })
}

pub fn is_path(path: &str) -> bool {
    if std::path::Path::new(path).exists() {
        return true;
    } else {
        return false;
    }
}

pub fn make_dir(path: &str) -> Option<bool> {
    if is_path(path) {
        return Some(true);
    } else {
        match std::fs::create_dir_all(path) {
            Ok(_) => return Some(true),
            Err(_) => return Some(false),
        }
    }
}

pub fn make_file(path: &str) -> bool {
    if is_path(path) {
        eprintln!("File already exists");
        return false;
    } else {
        File::create(path).unwrap();
        return is_path(path);
    }
}

pub fn del_dir(path: &str) -> Option<bool> {
    if is_path(path) {
        // deleting the original one

        std::fs::remove_dir_all(path).unwrap();
        return Some(true);
    } else {
        eprintln!("File cannot be erased if it doesn't exist");
        return Some(false);
    }
}

pub fn del_file(path: &str) -> bool {
    remove_file(path).unwrap();
    return !is_path(path);
}

pub fn unzip_folder(zip_path: &str, output_folder: &str) -> Result<bool, String> {
    // Open the zip file
    let file: Option<File> = match is_path(zip_path) {
        true => match File::open(zip_path) {
            Ok(f) => Some(f),
            Err(e) => {
                let message: String = format!("Unzipping failed: {}", e);
                return Err(message);
            }
        },
        false => None,
    };

    match file {
        Some(file) => {
            let mut archive = match ZipArchive::new(file) {
                Ok(d) => d,
                Err(_) => {
                    return Err("An error was encountered while reading the archive".to_string())
                }
            };

            // Create the output folder if it doesn't exist
            match make_dir(output_folder) {
                Some(_) => {
                    // Iterate over each file in the zip archive
                    for i in 0..archive.len() {

                        let mut file = match archive.by_index(i){
                            Ok(file_index) => file_index,
                            Err(_) => return Err("An error occoured while reading the zip, Possible corruption ?".to_string()),
                        };

                        // Extract file information
                        let file_path: String = format!(
                            "{}/{}",
                            output_folder,
                            file.mangled_name().to_string_lossy()
                        );

                        let mut output_file: File = match File::create(&file_path) {
                            Ok(file) => file,
                            Err(e) => return Err(format!("An error occoured while reading files in archive:\n{}", e)),
                        };

                        // Copy the file content to the output file
                        match std::io::copy(&mut file, &mut output_file) {
                            Ok(_) => print!("{}", &file_path),
                            Err(e) => return Err(format!("An error occoured while writing file to directory:\n{}", e)),
                        }
                    }
                    return Ok(true)
                }
                None => return Err("Failed to create the destination directory".to_string()),
            }
        }
        None => {
            return Err("Zip path provided was not vailid".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_path() -> String {
        let name = create_hash(&"dummy_test".to_string());
        let mut path = String::new();
        path.push_str("/tmp/");
        path.push_str(&name);
        return path;
    }

    fn get_file() -> String {
        let name = create_hash(&"dummy_test".to_string());
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
        let result = create_hash(&"hash".to_string());
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
        make_dir(&get_path());
        let result = del_dir(&get_path()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn create_file() {
        let result = make_file(&get_file());
        assert_eq!(result, true);
    }

    #[test]
    fn delete_file() {
        make_file(&get_file());
        let result = del_file(&get_file());
        assert_eq!(result, true);
    }

    //del_dir(!get_path());
}
