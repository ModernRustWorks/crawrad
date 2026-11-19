//! `File > New > Inherited Form…`: the forms a new one can inherit
//! from, the name it gets, and the file it is born as.
//!
//! The dialog only paints this model: the list, the check of the name and the
//! file written are decided here, where a test reaches them.

use crawrad_ir::Document;
use crawrad_note::note;
use std::path::{Path, PathBuf};

/// One form of the project, as the dialog lists it.
#[note("inherited-form-formentry")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormEntry {
    /// The form's root id, which is also its file stem.
    pub name: String,
    pub path: PathBuf,
    /// The form it inherits from, if any.
    pub base: Option<String>,
    /// How far under its base it is drawn: 0 for a form that inherits nothing.
    pub depth: usize,
}

#[note("inherited-form-forms-in")]
pub fn forms_in(dir: &Path) -> Vec<FormEntry> {
    let mut found: Vec<FormEntry> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| crate::form_file::is_form(p))
        .filter_map(|path| {
            let doc = Document::load(&path).ok()?;
            Some(FormEntry {
                name: doc.root.clone(),
                base: doc.inherits().map(str::to_string),
                path,
                depth: 0,
            })
        })
        .collect();
    found.sort_by(|a, b| a.name.cmp(&b.name));
    let known = |n: &str| found.iter().any(|f| f.name == n);
    let mut out = Vec::with_capacity(found.len());
    for top in found
        .iter()
        .filter(|f| !f.base.as_deref().is_some_and(known))
    {
        place(top, 0, &found, &mut out);
    }
    out
}

fn place(form: &FormEntry, depth: usize, all: &[FormEntry], out: &mut Vec<FormEntry>) {
    if depth > 16 || out.iter().any(|f| f.name == form.name) {
        return;
    }
    out.push(FormEntry {
        depth,
        ..form.clone()
    });
    for child in all.iter().filter(|f| f.base.as_deref() == Some(&form.name)) {
        place(child, depth + 1, all, out);
    }
}

/// What the dialog shows and creates.
#[note("inherited-form-newinheritedform")]
#[derive(Debug, Clone)]
pub struct NewInheritedForm {
    /// The folder the forms live in, and where the new one is written.
    pub dir: PathBuf,
    pub forms: Vec<FormEntry>,
    /// The base form picked in the list.
    pub selected: usize,
    /// The name typed for the new form.
    pub name: String,
    /// What the filter field holds: the list shows the forms whose name has it.
    pub filter: String,
}

impl NewInheritedForm {
    #[note("inherited-form-newinheritedform-new")]
    pub fn new(dir: &Path, base: Option<&str>) -> Self {
        let forms = forms_in(dir);
        let selected = base
            .and_then(|b| forms.iter().position(|f| f.name == b))
            .unwrap_or(0);
        let mut n = 1;
        while forms.iter().any(|f| f.name == format!("form_{n}"))
            || dir.join(format!("form_{n}.crawform")).exists()
        {
            n += 1;
        }
        Self {
            dir: dir.to_path_buf(),
            forms,
            selected,
            name: format!("form_{n}"),
            filter: String::new(),
        }
    }

    #[note("inherited-form-newinheritedform-rows")]
    pub fn rows(&self) -> Vec<usize> {
        let filter = self.filter.trim().to_lowercase();
        (0..self.forms.len())
            .filter(|&i| self.forms[i].name.to_lowercase().contains(&filter))
            .collect()
    }

    pub fn base(&self) -> Option<&FormEntry> {
        self.forms.get(self.selected)
    }

    #[note("inherited-form-newinheritedform-name-problem")]
    pub fn name_problem(&self) -> Option<String> {
        let name = self.name.trim();
        if name.is_empty() {
            return Some("Type a name for the new form".into());
        }
        if !crawrad_ir::is_snake_case_name(name) {
            let hint = crawrad_ir::snake_case_hint(name).unwrap_or_default();
            return Some(format!("Use snake_case, like {hint}"));
        }
        let taken = self.forms.iter().any(|f| f.name == name)
            || self.dir.join(format!("{name}.crawform")).exists();
        taken.then(|| format!("{name} is already a form of this project"))
    }

    #[note("inherited-form-newinheritedform-can-create")]
    pub fn can_create(&self) -> bool {
        self.base().is_some() && self.name_problem().is_none()
    }

    #[note("inherited-form-newinheritedform-summary")]
    pub fn summary(&self) -> Option<(String, String)> {
        let base = self.base()?;
        let own = Document::load(&base.path).ok()?;
        let resolved = crate::form_file::design_doc(&base.path, own);
        let doc = resolved.ir();
        let parts: Vec<&str> = doc
            .walk()
            .into_iter()
            .filter(|n| n.id != doc.root)
            .map(|n| n.id.as_str())
            .collect();
        let handlers: usize = doc.nodes.values().map(|n| n.events.len()).sum();
        let count = |n: usize, one: &str| match n {
            1 => format!("1 {one}"),
            _ => format!("{n} {one}s"),
        };
        Some((
            format!(
                "{} \u{b7} {} \u{b7} {}",
                base.name,
                count(parts.len(), "component"),
                count(handlers, "handler")
            ),
            parts.join(", "),
        ))
    }

    #[note("inherited-form-newinheritedform-files")]
    pub fn files(&self) -> (String, String) {
        let name = self.name.trim();
        let base = self.base().map(|b| b.name.as_str()).unwrap_or_default();
        let folder = self
            .dir
            .file_name()
            .map(|f| format!("{}/", f.to_string_lossy()))
            .unwrap_or_default();
        (
            format!("Creates {folder}{name}.crawform and {folder}{name}.rs"),
            format!(
                "struct {} {{ base: {}, \u{2026} }}",
                crawrad_codegen::tipo_do_form(name),
                crawrad_codegen::tipo_do_form(base)
            ),
        )
    }

    #[note("inherited-form-newinheritedform-create")]
    pub fn create(&self) -> Result<PathBuf, String> {
        if let Some(why) = self.name_problem() {
            return Err(why);
        }
        let base = self.base().ok_or("Pick the base form")?;
        let base_doc = Document::load(&base.path).map_err(|e| e.to_string())?;
        let class = base_doc
            .node(&base_doc.root)
            .map(|n| n.class.clone())
            .unwrap_or_else(|| "Form".into());
        let name = self.name.trim();
        let doc = Document::new_inherited(name, class, &base.name);
        let json = doc.to_json().map_err(|e| e.to_string())?;
        let path = self.dir.join(format!("{name}.crawform"));
        std::fs::write(&path, json).map_err(|e| e.to_string())?;
        Ok(path)
    }
}

#[cfg(test)]
#[path = "test/inherited_form/test.rs"]
mod test;
