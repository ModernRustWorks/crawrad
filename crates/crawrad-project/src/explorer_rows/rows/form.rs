//! A `.crawform` of a member — a FORM, which is a window, which is the
//! drawing.
//!
//! The one on screen is not a target: reopening it would drop the undo history
//! and change nothing on screen. It is drawn in bold instead, the way the
//! active project of a `.groupproj` is in a classic RAD IDE — what separates it is the
//! weight, not the colour.

use std::path::{Path, PathBuf};

use super::super::icon::{FORM, RowIcon};
use super::super::row::{Activation, ExplorerRow};
use crawrad_note::note;

#[note("explorer-rows-rows-form-form")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Form {
    path: PathBuf,
    name: String,
    /// The one on screen.
    current: bool,
}

impl Form {
    pub fn new(path: &Path, name: impl Into<String>, current: bool) -> Self {
        Self {
            path: path.to_path_buf(),
            name: name.into(),
            current,
        }
    }
}

impl ExplorerRow for Form {
    fn template(&self) -> &'static str {
        if self.current { "FormCurrent" } else { "Form" }
    }

    fn text(&self) -> &str {
        &self.name
    }

    fn icon(&self) -> Option<RowIcon> {
        Some(FORM)
    }

    fn indent(&self) -> usize {
        super::CHILD
    }

    fn activation(&self) -> Activation {
        if self.current {
            // Already open: reopening would drop undo history and change
            // nothing on screen.
            Activation::Inert
        } else {
            Activation::OpenForm(self.path.clone())
        }
    }

    fn item_target(&self) -> Option<(PathBuf, bool)> {
        Some((self.path.clone(), false))
    }
}
