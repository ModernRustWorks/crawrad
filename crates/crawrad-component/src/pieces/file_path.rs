use std::fmt;
use std::path::{Path, PathBuf};

/// A strongly-typed file path wrapper for component properties.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FilePath {
    path: String,
}

impl FilePath {
    /// Creates an empty file path.
    pub const fn empty() -> Self {
        Self {
            path: String::new(),
        }
    }

    /// Creates a file path from a string or path slice.
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    /// Returns the file path as a string slice.
    pub fn as_str(&self) -> &str {
        &self.path
    }

    /// Returns true if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Returns the length of the path in bytes.
    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Returns the file name component, if any.
    pub fn file_name(&self) -> Option<&str> {
        Path::new(&self.path).file_name().and_then(|s| s.to_str())
    }

    /// Returns the file extension, if any.
    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.path).extension().and_then(|s| s.to_str())
    }

    /// Returns true if the file exists on the local filesystem.
    pub fn exists(&self) -> bool {
        if self.path.is_empty() {
            return false;
        }
        Path::new(&self.path).exists()
    }

    /// Returns a normalized path string with standard forward slashes.
    pub fn normalized(&self) -> String {
        self.path.replace('\\', "/")
    }

    /// Converts into the underlying String.
    pub fn into_string(self) -> String {
        self.path
    }

    /// Converts into a PathBuf.
    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl From<&str> for FilePath {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for FilePath {
    fn from(s: String) -> Self {
        Self { path: s }
    }
}

impl From<PathBuf> for FilePath {
    fn from(p: PathBuf) -> Self {
        Self {
            path: p.to_string_lossy().into_owned(),
        }
    }
}

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}
