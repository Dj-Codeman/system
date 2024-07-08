#[cfg(test)]
mod tests {
    use crate::types::{ClonePath, CopyPath, PathType};

    use std::{
        ops::Deref,
        path::{Path, PathBuf},
    };

    #[test]
    fn test_pathbuf_variant() {
        let path_buf = PathBuf::from("/some/path");
        let path_type = PathType::PathBuf(path_buf.clone());

        assert_eq!(path_type.to_path_buf(), path_buf);
        assert_eq!(format!("{}", path_type), "/some/path");
    }

    #[test]
    fn test_path_variant() {
        let path = Path::new("/some/other/path");
        let path_type = PathType::Path(Box::from(path));

        assert_eq!(path_type.to_path_buf(), PathBuf::from("/some/other/path"));
        assert_eq!(format!("{}", path_type), "/some/other/path");
    }

    #[test]
    fn test_str_variant() {
        let path_str = "/yet/another/path";
        let path_type = PathType::Str(path_str.into());

        assert_eq!(path_type.to_path_buf(), PathBuf::from(path_str));
        assert_eq!(format!("{}", path_type), path_str);
    }

    #[test]
    fn test_content_variant() {
        let content = String::from("/content/path");
        let path_type = PathType::Content(content.clone());

        assert_eq!(path_type.to_path_buf(), PathBuf::from(content.clone()));
        assert_eq!(format!("{}", path_type), content);
    }

    #[test]
    fn test_clone_path() {
        let path_buf = PathBuf::from("/clone/path");
        let path_type = PathType::PathBuf(path_buf.clone());

        let cloned_path_type = path_type.clone_path();
        assert_eq!(cloned_path_type, PathType::PathBuf(path_buf));
    }

    #[test]
    fn test_copy_path() {
        let path_buf = PathBuf::from("/copy/path");
        let path_type = PathType::PathBuf(path_buf.clone());

        let copied_path_buf = path_type.copy_path();
        assert_eq!(copied_path_buf, path_buf);
    }

    #[test]
    fn test_deref() {
        let path_buf = PathBuf::from("/deref/path");
        let path_type = PathType::PathBuf(path_buf.clone());

        assert_eq!(path_type.deref(), path_buf.as_path());
    }

    #[test]
    fn test_as_ref() {
        let path_buf = PathBuf::from("/asref/path");
        let path_type = PathType::PathBuf(path_buf.clone());

        assert_eq!(&path_type.as_ref(), path_buf.as_path());
    }

    #[test]
    fn test_from_pathbuf() {
        let path_buf = PathBuf::from("/from/pathbuf");
        let path_type: PathType = path_buf.clone().into();

        assert_eq!(path_type, PathType::PathBuf(path_buf));
    }

    #[test]
    fn test_from_box_path() {
        let path = Path::new("/from/box/path");
        let boxed_path: Box<Path> = Box::from(path);
        let path_type: PathType = boxed_path.clone().into();

        assert_eq!(path_type, PathType::Path(boxed_path));
    }
}
