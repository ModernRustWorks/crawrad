//! Path to URI and URI to Path conversions following the Language Server Protocol specification.

use crawrad_note::note;
#[cfg(windows)]
use std::path::Component;
use std::path::{Path, PathBuf};

#[note("uri-path-to-uri")]
#[must_use]
pub fn path_to_uri(path: &Path) -> String {
    let mut uri = String::from("file://");

    #[cfg(windows)]
    {
        let mut components = path.components();
        if let Some(Component::Prefix(prefix_component)) = components.next() {
            let prefix_str = prefix_component.as_os_str().to_string_lossy();
            // Example: "C:" or "C:\"
            let drive = prefix_str.trim_end_matches(['\\', '/']);
            uri.push('/');
            uri.push_str(drive);
            uri.push('/');
            let rest: Vec<_> = components
                .filter_map(|c| match c {
                    Component::Normal(p) => Some(p.to_string_lossy()),
                    _ => None,
                })
                .collect();
            uri.push_str(&rest.join("/"));
            return uri;
        }
    }

    #[cfg(not(windows))]
    {
        uri.push_str(&path.to_string_lossy());
        uri
    }

    #[cfg(windows)]
    {
        let normalized = path.to_string_lossy().replace('\\', "/");
        if !normalized.starts_with('/') {
            uri.push('/');
        }
        uri.push_str(&normalized);
        uri
    }
}

#[note("uri-uri-to-path")]
#[must_use]
pub fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let path_str = uri.strip_prefix("file://")?;

    #[cfg(windows)]
    {
        // Example: "/C:/path/to/file.rs" -> "C:\path\to\file.rs"
        let trimmed = path_str.strip_prefix('/').unwrap_or(path_str);
        // Replace forward slashes with Windows separators
        let win_path = trimmed.replace('/', "\\");
        Some(PathBuf::from(win_path))
    }

    #[cfg(not(windows))]
    {
        Some(PathBuf::from(path_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn test_windows_path_to_uri_and_back() {
        let path = PathBuf::from(r"C:\workspace\project\src\main.rs");
        let uri = path_to_uri(&path);
        assert!(uri.starts_with("file:///C:"));
        assert!(uri.ends_with("/src/main.rs"));

        let recovered = uri_to_path(&uri).expect("must parse back to path");
        assert_eq!(recovered, path);
    }

    #[test]
    #[cfg(not(windows))]
    fn test_unix_path_to_uri_and_back() {
        let path = PathBuf::from("/workspace/project/src/main.rs");
        let uri = path_to_uri(&path);
        assert_eq!(uri, "file:///workspace/project/src/main.rs");

        let recovered = uri_to_path(&uri).expect("must parse back to path");
        assert_eq!(recovered, path);
    }
}
