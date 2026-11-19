//! The two rows that name the GROUP — what it is called, and where it is.
//!
//! They live in one file because they are one statement split over two lines:
//! the name is what you call the thing, the path is where it is on disk, and
//! neither answers without the other. Splitting them would give two files that
//! can only be read together.

use super::super::icon::{GROUP_ROOT, RowIcon};
use super::super::row::ExplorerRow;
use crawrad_note::note;

#[note("explorer-rows-rows-group-groupname")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupName(String, Option<std::path::PathBuf>);

impl GroupName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into(), None)
    }

    #[note("explorer-rows-rows-group-groupname-at")]
    pub fn at(self, root: impl Into<std::path::PathBuf>) -> Self {
        Self(self.0, Some(root.into()))
    }
}

impl ExplorerRow for GroupName {
    fn template(&self) -> &'static str {
        "GroupName"
    }

    fn text(&self) -> &str {
        &self.0
    }

    fn icon(&self) -> Option<RowIcon> {
        Some(GROUP_ROOT)
    }

    fn item_target(&self) -> Option<(std::path::PathBuf, bool)> {
        self.1.clone().map(|root| (root, true))
    }
}

#[note("explorer-rows-rows-group-grouppath")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupPath(String);

impl GroupPath {
    pub fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }
}

impl ExplorerRow for GroupPath {
    fn template(&self) -> &'static str {
        "GroupPath"
    }

    fn text(&self) -> &str {
        &self.0
    }

    fn item_target(&self) -> Option<(std::path::PathBuf, bool)> {
        Some((std::path::PathBuf::from(&self.0), true))
    }
}
