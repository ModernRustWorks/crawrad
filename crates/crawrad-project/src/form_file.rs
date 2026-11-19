//! The form's file follows the form's `Name`.
//!
//! The generated `<name>.designer.rs` and the user's `<name>.rs` were already
//! named after the form's `Name`, and `Generate` carries the user's file along
//! when it changes. The `.crawform` stayed behind, so renaming the form in the
//! Inspector left the folder with two names for one thing. The owner chose
//! that all three follow the `Name` (25/10/2026): saving the form moves it
//! to `<name>.crawform` first, and the next `Generate` carries the two `.rs`.

use std::path::{Path, PathBuf};

use crawrad_ir::Document;
use crawrad_note::note;

#[note("form-file-is-form")]
pub fn is_form(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e == crate::workspace::EXTENSION)
}

#[note("form-file-path-for")]
pub fn path_for(current: &Path, ir: &Document) -> PathBuf {
    let Some(root) = ir.node(&ir.root) else {
        return current.to_path_buf();
    };
    let Ok(name) = crawrad_codegen::identificador(root.name()) else {
        return current.to_path_buf();
    };
    let target = current.with_file_name(format!("{name}.{}", crate::workspace::EXTENSION));
    let same_file = current
        .file_name()
        .zip(target.file_name())
        .is_some_and(|(a, b)| a.eq_ignore_ascii_case(b));
    if target == current || (target.exists() && !same_file) {
        return current.to_path_buf();
    }
    target
}

#[note("form-file-save")]
pub fn save(ir: &Document, current: &Path) -> Result<PathBuf, String> {
    let target = path_for(current, ir);
    if target != current && current.exists() {
        // A rename, not write + remove: a case-only change is the same file
        // on Windows, and removing "the old one" would remove the form.
        std::fs::rename(current, &target).map_err(|e| format!("{}: {e}", current.display()))?;
    }
    ir.save(&target).map_err(|e| e.to_string())?;
    Ok(target)
}

#[note("form-file-design-doc")]
pub fn design_doc(path: &Path, ir: Document) -> crawrad_doc::Doc {
    let bases = crate::generator::load_bases(path.parent(), &ir);
    match Document::resolve_chain(&bases) {
        Some(base) => {
            crawrad_doc::Doc::inheriting(&ir, base).unwrap_or_else(|_| crawrad_doc::Doc::new(ir))
        }
        None => crawrad_doc::Doc::new(ir),
    }
}

#[cfg(test)]
#[path = "test/form_file/test.rs"]
mod test;
