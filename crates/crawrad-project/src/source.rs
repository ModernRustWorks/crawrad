//! What the CENTRE shows when it is not the drawing — and where that text
//! comes from.
//!
//! # The defect this module exists to close
//!
//! The Makepad shell had a `Code` tab that was an **empty text box**:
//! `toggle_view` flipped two `visible` flags and nothing ever wrote into
//! `code_view`. The owner named the shape of the bug on 15/09/2026, about the
//! egui shell, and it applies here with interest:
//!
//! > *"And when it goes to the code design it must fire the generator, so that
//! > the code is up to date."*
//!
//! Showing derived text without re-deriving it has already bitten this house:
//! `pousar_na_linha` (`main.rs:3744-3750`) resyncs first because *"`Run`
//! generates before compiling: the `.designer.rs` on screen is the one from
//! before generation, and the error lines are the ones from after"*. A stale
//! `.designer.rs` is worse than a blank one — the user reads it and believes
//! it.
//!
//! # The two texts, and why they are not the same
//!
//! | key | what it shows | where it comes from |
//! |---|---|---|
//! | `Alt+F12` | the form as TEXT — the `.designer.rs`, our `.dfm` | **planned now**, in memory, from the open document |
//! | `F12` | the unit where the handlers live | read from DISK at that instant |
//!
//! The split is the one `codigo::ModoDesign` already draws in
//! the egui shell: `Design Code` is a *view of the drawing*, and `Code` is the
//! file you edit. Moving a component changes the first and never the second,
//! which is exactly why `Alt+F12` is the one the owner asked about.
//!
//! # What this module does NOT do, and it is the honest limit
//!
//! **It never writes to disk.** Planning is pure
//! ([`crawrad_codegen::Gerador::gerar`]), and that is all that happens here. The
//! real `Generate` — which writes the files, runs the `vinculos_vencidos`
//! pre-pass and prunes the empty handlers (ADR 0025) — lives in the egui shell
//! (`main.rs`, `gerar_projeto`), and reaching it from here means giving it a
//! neutral home first. So `F12` shows the unit **as it is on disk**, and says
//! so when there is nothing there yet.

use std::path::{Path, PathBuf};

use crawrad_codegen::Gerador;
use crawrad_note::note;

#[note("source-not-generated-yet")]
pub const NOT_GENERATED_YET: &str = "// This form has no generated unit on disk yet.\n\
     //\n\
     // The unit is where your handlers live, and it is written by `Generate`\n\
     // (Project menu, or the Run button, which generates before it builds).\n\
     //\n\
     // Alt+F12 shows the form source instead, and that one is planned live\n\
     // from the document you are editing — no disk needed.";

#[note("source-package-name")]
pub fn package_name(form: &Path) -> String {
    let Some(member) = member_of(form) else {
        return "app".into();
    };
    let Ok(text) = std::fs::read_to_string(member.join("Cargo.toml")) else {
        return "app".into();
    };
    package_name_in(&text).unwrap_or_else(|| "app".into())
}

#[note("source-package-name-in")]
pub fn package_name_in(manifest: &str) -> Option<String> {
    let mut in_package = false;
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package {
            continue;
        }
        if let Some(rest) = line.strip_prefix("name") {
            let rest = rest.trim_start().strip_prefix('=')?.trim();
            let value = rest.trim_matches(|c| c == '"' || c == '\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

#[note("source-member-of")]
pub fn member_of(form: &Path) -> Option<PathBuf> {
    let src = form.parent()?;
    let name = src.file_name()?;
    (name == "src")
        .then(|| src.parent())?
        .map(Path::to_path_buf)
}

#[note("source-form-source")]
pub fn form_source(doc: &crawrad_ir::Document, package: &str) -> Result<String, String> {
    let target = crate::generator::active_target();
    let files = target
        .create_generator()
        .gerar(doc, package, crate::generator::catalog())
        .map_err(|e| format!("{e}"))?;
    let suffix = target.sufixo_gerado();
    files
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().ends_with(suffix))
        .map(|a| a.conteudo)
        .ok_or_else(|| format!("the generator produced no `{}` for this form", suffix))
}

#[note("source-unit-path")]
pub fn unit_path(form: &Path) -> PathBuf {
    form.with_extension("rs")
}

#[note("source-user-unit")]
pub fn user_unit(form: &Path, doc: Option<&crawrad_ir::Document>) -> String {
    if let Ok(no_disco) = std::fs::read_to_string(unit_path(form)) {
        return without_cr(&no_disco);
    }
    let Some(doc) = doc else {
        return NOT_GENERATED_YET.to_string();
    };
    match user_unit_skeleton(doc, &package_name(form)) {
        Some(esqueleto) => format!("{NOT_ON_DISK_YET}{esqueleto}"),
        None => NOT_GENERATED_YET.to_string(),
    }
}

#[note("source-without-cr")]
pub fn without_cr(text: &str) -> String {
    text.replace('\r', "")
}

#[note("source-not-on-disk-yet")]
pub const NOT_ON_DISK_YET: &str =
    "// ---------------------------------------------------------------------
     // NOT ON DISK YET — this is the file `Generate` will create for you.
     //
     // It is planned from the form you are editing right now, so it already
     // has the header and the marker where your event handlers go. Wire an
     // event in the Object Inspector, or hit Generate, and it lands on disk
     // exactly like this.
     // ---------------------------------------------------------------------

";

#[note("source-user-unit-skeleton")]
fn user_unit_skeleton(doc: &crawrad_ir::Document, package: &str) -> Option<String> {
    crate::generator::active_target()
        .create_generator()
        .gerar(doc, package, crate::generator::catalog())
        .ok()?
        .into_iter()
        .find(|a| a.dono == crawrad_codegen::Dono::Usuario)
        .map(|a| a.conteudo)
}

/// What the center shows as text: the user's unit or the generated form source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterView {
    Code,
    FormSource,
}

/// The center's text and, when generating failed, the status message.
pub fn center_content(
    active_tab: Option<&Path>,
    active_file: Option<&Path>,
    view: CenterView,
    doc: Option<&crawrad_ir::Document>,
) -> (String, Option<String>) {
    let unit_of_form = active_file.map(unit_path);
    if let Some(opened) = active_tab
        && unit_of_form.as_deref() != Some(opened)
    {
        let text = match std::fs::read_to_string(opened) {
            Ok(t) => without_cr(&t),
            Err(e) => format!("// {}: {e}", opened.display()),
        };
        return (text, None);
    }
    let Some(path) = active_file else {
        return (NOT_GENERATED_YET.to_string(), None);
    };
    match view {
        CenterView::Code => (user_unit(path, doc), None),
        CenterView::FormSource => {
            let Some(doc) = doc else {
                return (NOT_GENERATED_YET.to_string(), None);
            };
            let package = package_name(path);
            match form_source(doc, &package) {
                Ok(t) => (t, None),
                Err(e) => (format!("// generate: {e}"), Some(format!("generate: {e}"))),
            }
        }
    }
}

/// Locates the 0-based handler line in a unit file on disk.
pub fn find_handler_line(unit: &Path, handler: &str) -> Option<usize> {
    let text = std::fs::read_to_string(unit).ok()?;
    crawrad_codegen::elo::localizar_handler(&text, handler).map(|d| d.linha)
}

/// Finds the unit path and handler line for an active file path.
pub fn handler_location(
    active_path: Option<&Path>,
    handler: &str,
) -> (Option<PathBuf>, Option<usize>) {
    let unit = active_path.map(unit_path);
    let line = unit.as_deref().and_then(|u| find_handler_line(u, handler));
    (unit, line)
}

#[cfg(test)]
#[path = "test/source/test.rs"]
mod source_test;
