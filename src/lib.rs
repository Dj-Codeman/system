use std::{str, fs::{File, remove_file} };
use sha2::{Digest, Sha256};


/// Create 256 bit hash
pub fn create_hash(data: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hash: String = hex::encode(result);
    return hash
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

pub fn is_path(path: &str) -> bool {
    if std::path::Path::new(path).exists() { 
        return true;
    } else {
        return false;
    }
}

pub fn make_dir(path: &str) -> Option<bool> {
    std::fs::create_dir_all(path).unwrap();
    return Some(is_path(path));
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
    if is_path(path) { // deleting the original one
        
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
        assert_eq!(result, "d04b98f48e8f8bcc15c6ae5ac050801cd6dcfd428fb5f9e65c4e16e7807340fa".to_string());
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
