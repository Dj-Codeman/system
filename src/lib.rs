use std::str;
use sha2::{Digest, Sha256};
// use aes::Aes256;


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

pub fn ispath(path: &str) -> bool {
    if std::path::Path::new(path).exists() { 
        return true;
    } else {
        return false;
    }
}

pub fn makedir(path: &str) -> Option<bool> {
    std::fs::create_dir_all(path).unwrap();
    if ispath(path) { 
        return Some(true); 
    } else {
        return Some(false);
    }
}

pub fn deldir(path: &str) -> Option<bool> {
    if ispath(path) { // deleting the original one
        
        std::fs::remove_dir_all(path).unwrap();
        return Some(true);

    } else {
        eprintln!("File cannot be erased if it doesn't exist");
        return Some(false);
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

    #[test]
    fn trimming() {
        let result = truncate("Hello, World", 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn ispath_test() {
        let result = ispath("/tmp/definatly_real_path");
        assert_eq!(result, false);
    }

    #[test]
    fn hash() {
        let result = create_hash(&"hash".to_string());
        assert_eq!(result, "d04b98f48e8f8bcc15c6ae5ac050801cd6dcfd428fb5f9e65c4e16e7807340fa".to_string());
    }

    #[test]
    fn create() {
        let result = makedir(&get_path()).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn destroy() {
        makedir(&get_path());
        let result = deldir(&get_path()).unwrap();
        assert_eq!(result, true);
    }

}
