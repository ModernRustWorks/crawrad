#![forbid(unsafe_code)]
//! CrawRAD Extensions runtime and host integration.
//!
//! Exposes declarative manifests loading, concrete Editor, Project, and Designer services,
//! and command registration for the CrawRAD Extension API.

pub mod cards;
pub mod host;
pub mod install;
pub mod line_marks;
pub mod process;
pub mod services;
pub mod views;

pub use cards::{ExtensionCardManager, UiCardState};
pub use host::{ShellExtensionContext, ShellExtensionHost};
pub use process::{ProcessExtension, ProcessExtensionHost, ProcessStatus};
pub use services::{HostDesignerService, HostEditorService, HostProjectService, apply_text_edit};

#[cfg(test)]
#[path = "test/extensions/test.rs"]
mod tests;

#[cfg(test)]
pub(crate) mod extensions {
    pub use crate::cards::ExtensionCardManager;
    pub mod host {
        pub use crate::host::*;
    }
    pub mod process {
        pub use crate::process::*;
    }
    pub mod services {
        pub use crate::services::*;
    }
}
