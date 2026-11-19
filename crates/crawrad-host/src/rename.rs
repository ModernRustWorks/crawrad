//! **Rename Symbol**: the RAD guard and the dialog's state.
//!
//! In a RAD project part of the names **belong to the form**, not to the
//! code: a component's name (`btn_ok`) is declared in the generated
//! `.designer.rs`, and an event handler (`btn_ok_click`) is bound in the
//! `.crawform`. A rename through the language server would edit the
//! generated file (the next generation undoes it) and never the form — the
//! project stops compiling, or the event unbinds in silence. So:
//!
//! 1. [`form_owner`] — a name the form owns is refused before the server is
//!    asked, with the right road (the Object Inspector);
//! 2. [`forbidden`] — a `WorkspaceEdit` with any edit in a generated source
//!    or outside the project is refused **whole**: a rename applies to every
//!    file or to none;
//! 3. the edits are shown ([`Rename::preview`]) and applied only on Apply,
//!    in memory: nothing is written to the disk.

use std::path::{Path, PathBuf};

use crate::navigation::{Pending, is_foreign};
use crawrad_note::note;

#[note("rename-owner")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Owner {
    /// A component's name.
    Component { form: String, name: String },
    /// A handler bound to an event of a component.
    Handler {
        form: String,
        component: String,
        event: String,
    },
}

impl Owner {
    #[note("rename-owner-refusal")]
    pub fn refusal(&self) -> String {
        match self {
            Owner::Component { form, name } => format!(
                "`{name}` is a component of {form}: rename it from the Object Inspector (Name, \
                 `component.rename`), which changes the form and its code together"
            ),
            Owner::Handler {
                form,
                component,
                event,
            } => format!(
                "this handler is bound to `{component}.{event}` in {form}: rename it from the \
                 Object Inspector (Events, `event.rename`), which changes the form and its code \
                 together"
            ),
        }
    }
}

#[note("rename-form-owner")]
pub fn form_owner(name: &str, forms: &[(String, &crawrad_ir::Document)]) -> Option<Owner> {
    for (form, doc) in forms {
        for node in doc.nodes.values() {
            if node.name() == name && node.id != doc.root {
                return Some(Owner::Component {
                    form: form.clone(),
                    name: name.to_string(),
                });
            }
            if let Some((event, _)) = node.events.iter().find(|(_, h)| h.as_str() == name) {
                return Some(Owner::Handler {
                    form: form.clone(),
                    component: node.name().to_string(),
                    event: event.clone(),
                });
            }
        }
    }
    None
}

#[note("rename-refused")]
pub fn refused(
    files: &[PathBuf],
    project: &Path,
    foreign: &[PathBuf],
    generated_suffix: &str,
) -> Option<String> {
    let bad = forbidden(files, project, foreign, generated_suffix);
    let names: Vec<String> = bad.iter().map(|p| p.display().to_string()).collect();
    (!names.is_empty()).then(|| names.join(", "))
}

/// Why a rename cannot go ahead, if it cannot.
pub fn check_refusal(
    files: &[PathBuf],
    project: &Path,
    foreign: &[PathBuf],
    generated_suffix: &str,
) -> Option<String> {
    refused(files, project, foreign, generated_suffix).map(|names| {
        format!("Rename refused, nothing changed: it would edit {names} (generated from the form, or not the project's)")
    })
}

/// Why a form's owner refuses the rename, if it does.
pub fn check_form_owner_refusal(
    name: &str,
    forms: &[(String, &crawrad_ir::Document)],
) -> Option<String> {
    form_owner(name, forms).map(|o| o.refusal())
}

#[note("rename-forbidden")]
pub fn forbidden(
    files: &[PathBuf],
    project: &Path,
    foreign: &[PathBuf],
    generated_suffix: &str,
) -> Vec<PathBuf> {
    files
        .iter()
        .filter(|f| {
            let generated = f
                .file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(generated_suffix));
            generated || is_foreign(f, foreign) || !is_foreign(f, &[project.to_path_buf()])
        })
        .cloned()
        .collect()
}

#[note("rename-filechange")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    pub file: PathBuf,
    pub count: usize,
    pub before: String,
    pub after: String,
}

#[note("rename-stage")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Stage {
    /// Closed.
    #[default]
    Closed,
    /// Asking the server whether the name at the caret can be renamed.
    Preparing,
    /// The user types the new name.
    Naming,
    /// Asking the server for the edits.
    Asking,
    /// The edits, shown; Apply applies them.
    Preview(Vec<FileChange>),
}

#[note("rename-rename")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Rename {
    pub stage: Stage,
    pub file: PathBuf,
    pub caret: (usize, usize),
    pub version: i32,
    pub old: String,
    pub new: String,
    pub message: String,
    pub pending: Pending,
    /// The field should take the keys as soon as it is drawn.
    pub wants_focus: bool,
}

impl Rename {
    pub fn is_open(&self) -> bool {
        self.stage != Stage::Closed
    }

    #[note("rename-rename-start")]
    pub fn start(&mut self, file: PathBuf, caret: (usize, usize), version: i32, old: &str) -> u64 {
        *self = Self {
            stage: Stage::Preparing,
            caret,
            version,
            old: old.to_string(),
            new: old.to_string(),
            ..Self::default()
        };
        self.file = file.clone();
        self.pending.ask(file, version)
    }

    #[note("rename-rename-typed")]
    pub fn typed(&mut self, new: &str) {
        self.new = new.to_string();
        if matches!(self.stage, Stage::Preview(_)) {
            self.stage = Stage::Naming;
        }
        self.message.clear();
    }

    /// Takes the planned changes into the preview, or the file that blocked them.
    pub fn apply_preview(&mut self, planned: Result<Vec<FileChange>, PathBuf>) {
        match planned {
            Ok(p) if p.is_empty() => {
                self.stage = Stage::Naming;
                self.message = "Nothing to rename".into();
            }
            Ok(p) => self.stage = Stage::Preview(p),
            Err(f) => {
                self.stage = Stage::Naming;
                self.message = format!("Cannot read {}", f.display());
            }
        }
    }

    #[note("rename-rename-cannot-ask")]
    pub fn cannot_ask(&self) -> Option<&'static str> {
        let new = self.new.trim();
        if new.is_empty() {
            Some("Type the new name")
        } else if new == self.old {
            Some("The new name is the same as the old one")
        } else {
            None
        }
    }

    #[note("rename-rename-preview")]
    pub fn preview(&self) -> String {
        let Stage::Preview(files) = &self.stage else {
            return String::new();
        };
        let total: usize = files.iter().map(|f| f.count).sum();
        let mut out = format!(
            "{total} occurrence(s) in {} file(s), applied in memory (Save All writes them):",
            files.len()
        );
        for f in files {
            let name = f
                .file
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();
            out.push_str(&format!("\n  {name}  \u{2014}  {}", f.count));
        }
        out
    }

    #[note("rename-rename-close")]
    pub fn close(&mut self) {
        *self = Self::default();
    }

    #[note("rename-rename-to-json")]
    pub fn to_json(&self) -> serde_json::Value {
        let (stage, files) = match &self.stage {
            Stage::Closed => ("closed", vec![]),
            Stage::Preparing => ("preparing", vec![]),
            Stage::Naming => ("naming", vec![]),
            Stage::Asking => ("asking", vec![]),
            Stage::Preview(f) => (
                "preview",
                f.iter()
                    .map(|c| serde_json::json!({ "file": c.file.display().to_string(), "count": c.count }))
                    .collect(),
            ),
        };
        serde_json::json!({
            "open": self.is_open(), "stage": stage, "old": self.old, "new": self.new,
            "message": self.message, "files": files,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refs {
    Loading,
    Found(Vec<FileChange>),
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentRename {
    pub node: String,
    pub old: String,
    pub new: String,
    pub form_file: String,
    pub handlers: Vec<(String, String)>,
    pub refs: Refs,
    pub rename_handlers: bool,
    pub rename_refs: bool,
    /// The generated form file the references are asked from.
    pub designer: PathBuf,
    /// When rust-analyzer may be asked again (it was still loading).
    pub retry_at: Option<std::time::Instant>,
}

impl ComponentRename {
    pub fn new(
        node: String,
        old: String,
        new: String,
        form_file: String,
        handlers: Vec<(String, String)>,
        refs: Refs,
    ) -> Self {
        let rename_refs = matches!(refs, Refs::Found(ref f) if !f.is_empty());
        Self {
            node,
            old,
            new,
            form_file,
            handlers,
            refs,
            rename_handlers: true,
            rename_refs,
            designer: PathBuf::new(),
            retry_at: None,
        }
    }

    pub fn needs_card(&self) -> bool {
        let has_handlers = !self.handlers.is_empty();
        let has_refs = match &self.refs {
            Refs::Found(files) => files.iter().any(|f| f.count > 0),
            _ => false,
        };
        has_handlers || has_refs
    }

    pub fn handler_line(&self) -> String {
        self.handlers
            .iter()
            .map(|(old, new)| format!("{old} \u{2192} {new}"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn refs_line(&self) -> String {
        match &self.refs {
            Refs::Loading | Refs::Unavailable => "rust-analyzer is still loading".to_string(),
            Refs::Found(files) => {
                let total: usize = files.iter().map(|f| f.count).sum();
                let names = files
                    .iter()
                    .filter_map(|f| f.file.file_name().map(|n| n.to_string_lossy()))
                    .collect::<Vec<_>>()
                    .join(", ");
                let place = if total == 1 { "place" } else { "places" };
                if names.is_empty() {
                    format!("{total} {place} (rust-analyzer)")
                } else {
                    format!("{total} {place} in {names} (rust-analyzer)")
                }
            }
        }
    }

    pub fn everywhere(&self) -> bool {
        self.rename_handlers
    }

    pub fn edits_to_apply(&self) -> &[FileChange] {
        if self.rename_refs
            && let Refs::Found(files) = &self.refs
        {
            return files.as_slice();
        }
        &[]
    }
}

pub fn find_default_handlers(
    events: &std::collections::BTreeMap<String, String>,
    old: &str,
    new: &str,
) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for (event, handler) in events {
        if *handler == crawrad_ir::default_handler_name(old, event) {
            pairs.push((
                handler.clone(),
                crawrad_ir::default_handler_name(new, event),
            ));
        } else if *handler == crawrad_ir::legacy_handler_name(old, event) {
            pairs.push((handler.clone(), crawrad_ir::legacy_handler_name(new, event)));
        }
    }
    pairs
}

/// Where `pub <old>:` is declared in the generated form file, as the
/// (line, byte) caret rust-analyzer renames from.
pub fn find_field_caret(text: &str, old: &str) -> Option<(usize, usize)> {
    let target = format!("pub {old}:");
    let mut inside_struct = false;
    for (line_idx, line) in text.lines().enumerate() {
        if line.contains("pub struct ") {
            inside_struct = true;
        }
        if inside_struct {
            if let Some(col) = line.find(&target) {
                return Some((line_idx, col + 4));
            }
            if line.trim_start().starts_with('}') {
                inside_struct = false;
            }
        }
    }
    // Fallback search across file
    for (line_idx, line) in text.lines().enumerate() {
        if let Some(col) = line.find(&target) {
            return Some((line_idx, col + 4));
        }
    }
    None
}

#[cfg(test)]
#[path = "test/rename/test.rs"]
mod tests;
