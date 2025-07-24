use std::{io, path};

/// Returns the relative path from `from_dir` to `to_file` if `to_file`.
fn get_relative_path<P: AsRef<path::Path>>(from_path: P, to_path: P) -> io::Result<path::PathBuf> {
    let from_path = path::absolute(from_path)?;
    let to_path = path::absolute(to_path)?;
    let mut from_dir_components = from_path.components();
    let mut to_file_components = to_path.components();

    // Check if the `to_file` is in the same directory tree as `from_dir` on windows
    #[cfg(target_os = "windows")]
    if !from_dir_components.next().eq(&to_file_components.next()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The `to_file` is not in the same directory tree as `from_dir`",
        ));
    }

    // Skip the common components
    let mut from_dir_component = from_dir_components.next();
    let mut to_file_component = to_file_components.next();
    while from_dir_component.eq(&to_file_component) {
        from_dir_component = from_dir_components.next();
        to_file_component = to_file_components.next();
    }

    let mut relative_path = path::PathBuf::new();
    if from_dir_component.is_some() {
        // Add `..` for each component in `from_dir` that is not in `to_file`
        // Manually add one `..` because it was skipped in the loop above
        relative_path.push("..");
        for _ in from_dir_components {
            relative_path.push("..");
        }
    }

    // Add the remaining components of `to_file`
    if let Some(component) = to_file_component {
        relative_path.push(component);
        for component in to_file_components {
            relative_path.push(component);
        }
    }

    Ok(relative_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_relative_path_different_drive() {
        let base = path::Path::new("C:\\");
        let file = path::Path::new("D:\\file.txt");
        let result = get_relative_path(base, file);
        assert!(result.is_err());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_relative_path_same_drive() {
        let base = path::Path::new("C:\\foo\\bar\\sub");
        let file = path::Path::new("C:\\xyz\\baz\\file.txt");
        let rel = get_relative_path(base, file).unwrap();
        assert_eq!(
            rel,
            path::PathBuf::from_iter(["..", "..", "..", "xyz", "baz", "file.txt"])
        );
    }

    #[test]
    fn test_relative_path_same_dir() {
        let base = path::Path::new("/foo/bar");
        let file = path::Path::new("/foo/bar/file.txt");
        let rel = get_relative_path(base, file).unwrap();
        assert_eq!(rel, path::PathBuf::from("file.txt"));
    }

    #[test]
    fn test_relative_path_subdir() {
        let base = path::Path::new("/foo/bar");
        let file = path::Path::new("/foo/bar/sub/file.txt");
        let rel = get_relative_path(base, file).unwrap();
        assert_eq!(rel, path::PathBuf::from_iter(["sub", "file.txt"]));
    }

    #[test]
    fn test_relative_path_parent_dir() {
        let base = path::Path::new("/foo/bar/sub");
        let file = path::Path::new("/foo/bar/file.txt");
        let rel = get_relative_path(base, file).unwrap();
        assert_eq!(rel, path::PathBuf::from_iter(["..", "file.txt"]));
    }

    #[test]
    fn test_relative_path() {
        let base = path::Path::new("/foo/bar/sub");
        let file = path::Path::new("/xyz/baz/file.txt");
        let rel = get_relative_path(base, file).unwrap();
        assert_eq!(
            rel,
            path::PathBuf::from_iter(["..", "..", "..", "xyz", "baz", "file.txt"])
        );
    }
}
