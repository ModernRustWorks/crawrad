//! Code generation: the one place in the product that turns a document into
//! the user's project on disk.
//!
//! # Why this module exists
//!
//! `Generate` used to live inside `main.rs`, as a method on the `egui` shell's
//! `Ide`. Two separate rounds ran into the same wall and wrote it down: the
//! Makepad shell has the button, the catalogue has the action, and the path
//! dies at `main.rs::gerar_projeto` — a function in the *other* binary. The
//! symptom measured by the round that built the dock: `F12 > Code` on a form
//! that was never generated shows a warning instead of the code, because the
//! shell that could write it cannot reach the writer.
//!
//! So this module knows no framework: no `egui`, no `makepad`, not even a
//! window. A document goes in, files come out, and a sentence for the status
//! bar comes back.
//!
//! # The three laws that travelled with it
//!
//! Each of these already cost a round with the owner, and none of them may be
//! split in half by the move:
//!
//! 1. **The user's code is never overwritten, it only grows** — the file
//!    ownership rule of ADR 0010. It is not implemented here: it lives inside
//!    [`crawrad_codegen::escrever`], which is the only writer. This module's job
//!    is to make sure nobody reaches the disk by another door.
//! 2. **The pruning of ADR 0025** — an empty event handler leaves the user's
//!    file on generate, linked or not, exactly as a classic RAD IDE does (measured by the
//!    owner in his own a classic RAD IDE on 2026-09-08). The stub is recognised by the
//!    **derived name**, never by the shape of a marker, and that distinction
//!    is `crawrad_codegen`'s to keep.
//! 3. **The pre-pass runs FIRST.** [`plan_with_prepass`] asks
//!    [`crawrad_codegen::vinculos_vencidos`] which links expired *before* it asks
//!    the generator for a plan, and it is what stops `Generate` from writing a
//!    broken link. Written as two statements in the old method, the order was
//!    held by a comment: moving the pre-pass after the plan left the whole
//!    suite green, and the price was `E0599` in the *client's* project. Here
//!    the order is structural — the caller gets the pruned plan and has no way
//!    to ask for the other one.
//!
//! # What stayed in the shell, and why
//!
//! Three things did **not** come along, and none of them by omission:
//!
//! - **confirming what is being typed.** The `Name` and `OnClick` fields live
//!   in a shell buffer that only becomes a transaction when the widget loses
//!   focus, and `Generate` does not take focus away from anything. Whoever
//!   calls [`prepare`] confirms first; that is a widget fact, and it is the
//!   shell's;
//! - **the status field.** This module *returns* the sentence. Where it is
//!   shown is not its business;
//! - **mutating the document.** The pre-pass proposes, the shell applies. The
//!   only mutation door is the shell's own, and [`Plan::unlink`] hands it a
//!   transaction instead of writing behind its back.
//!
//! # What this module knows of the crate
//!
//! One module, `workspace`, for the manifest name and its parser — see
//! [`package_name_in`]. Reaching this file from the Makepad binary therefore
//! costs three `#[path]` declarations (`generator`, `workspace`, and the
//! `launcher` that `workspace` calls), and that is the whole bill: no other
//! sibling is touched, and no framework at all.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};

use crawrad_doc::{Command, Transaction};
use crawrad_ir::Document;

use crate::workspace;
use crawrad_note::note;

// ------------------------------------------------------------- the refusal

#[note("generator-failure-prefix")]
pub const FAILURE_PREFIX: &str = "generate:";

#[note("generator-refusal")]
pub fn refusal(reason: impl std::fmt::Display) -> String {
    format!("{FAILURE_PREFIX} {reason}")
}

#[note("generator-no-member")]
pub const NO_MEMBER: &str = "no form open — a form lives in <member>/src/";

#[note("generator-no-document")]
pub const NO_DOCUMENT: &str = "no form open";

#[note("generator-refused")]
pub fn refused(status: &str) -> bool {
    status.starts_with(FAILURE_PREFIX)
}

// -------------------------------------------------------------- the target

#[note("generator-active-target")]
static ACTIVE_TARGET: AtomicU8 = AtomicU8::new(0);

#[note("generator-active-target-2")]
pub fn active_target() -> crawrad_codegen::AlvoGerador {
    match ACTIVE_TARGET.load(Ordering::Relaxed) {
        1 => crawrad_codegen::AlvoGerador::Egui,
        2 => crawrad_codegen::AlvoGerador::Slint,
        _ => crawrad_codegen::AlvoGerador::Makepad,
    }
}

#[note("generator-target-key")]
pub fn target_key() -> &'static str {
    active_target().alvo_curto()
}

#[note("generator-set-active-target")]
pub fn set_active_target(target: crawrad_codegen::AlvoGerador) {
    let value = match target {
        crawrad_codegen::AlvoGerador::Makepad => 0,
        crawrad_codegen::AlvoGerador::Egui => 1,
        crawrad_codegen::AlvoGerador::Slint => 2,
    };
    ACTIVE_TARGET.store(value, Ordering::Relaxed);
}

#[note("generator-target-of-manifest")]
pub fn target_of_manifest(manifest: &str) -> Option<crawrad_codegen::AlvoGerador> {
    use crawrad_codegen::AlvoGerador;
    let named = |krate: &str| {
        manifest.lines().any(|line| {
            let name = line.trim_start().split(['=', '.', ' ']).next();
            name.map(|n| n.trim_matches('"')) == Some(krate)
        })
    };
    // ONE TARGET OR NONE, over however many targets exist. This read two
    // booleans until 28/11/2026, so a Slint project answered `None`, the active
    // target stayed at its default of Makepad, and the Object Inspector listed
    // MAKEPAD properties for a project the person had created as Slint - the
    // dialog honoured the choice and reopening the project lost it.
    let only = |hits: &[(bool, AlvoGerador)]| -> Option<AlvoGerador> {
        let mut found = None;
        for &(hit, alvo) in hits {
            if hit {
                // Two at once is not an answer, which is what the old pair said
                // too: the Makepad example app carries `eframe` as well.
                if found.is_some() {
                    return None;
                }
                found = Some(alvo);
            }
        }
        found
    };
    // Our own runtime crate names the target with no room for doubt, so it is
    // asked first; the toolkit crate is the fallback for a hand-written project.
    only(&[
        (named("crawrad-runtime-egui"), AlvoGerador::Egui),
        (named("crawrad-runtime-makepad"), AlvoGerador::Makepad),
        (named("crawrad-runtime-slint"), AlvoGerador::Slint),
    ])
    .or_else(|| {
        only(&[
            (named("eframe") || named("egui"), AlvoGerador::Egui),
            (named("makepad-widgets"), AlvoGerador::Makepad),
            (named("slint"), AlvoGerador::Slint),
        ])
    })
}

#[note("generator-project-target")]
pub fn project_target(member: &std::path::Path) -> Option<crawrad_codegen::AlvoGerador> {
    let manifest = std::fs::read_to_string(member.join(crate::workspace::MANIFEST)).ok()?;
    target_of_manifest(&manifest)
}

#[note("generator-project-generator")]
pub fn project_generator() -> Box<dyn crawrad_codegen::Gerador> {
    active_target().create_generator()
}

#[note("generator-has-a-target")]
pub fn has_a_target(member: &Path) -> bool {
    member.join("src").join("main.rs").is_file() || member.join("src").join("lib.rs").is_file()
}

#[note("generator-not-generated")]
pub const NOT_GENERATED: &str = "rust-analyzer (project not generated — Ctrl+S)";

pub fn package_name_in(member: &Path) -> String {
    std::fs::read_to_string(member.join(workspace::MANIFEST))
        .ok()
        .and_then(|text| workspace::package_name_from_manifest(&text))
        .unwrap_or_else(|| "app".into())
}

#[note("generator-preview-plan")]
pub fn preview_plan(
    member: Option<&Path>,
    ir: &Document,
    editing: &dyn Fn(&Path) -> Option<String>,
) -> Result<Vec<crawrad_codegen::Arquivo>, crawrad_codegen::Erro> {
    let package = member.map(package_name_in).unwrap_or_else(|| "app".into());
    plan_with_prepass(member, ir, &package, editing).1
}

// ---------------------------------------------------------------- the plan

#[note("generator-catalog")]
pub(crate) fn catalog() -> &'static crawrad_registry::Registry {
    &crate::installed::ide_catalog().catalog
}

#[note("generator-is-base-of-any")]
fn is_base_of_any(destination: Option<&Path>, ir: &Document) -> bool {
    let Some(d) = destination else {
        return false;
    };
    let Some(root) = ir.node(&ir.root) else {
        return false;
    };
    let my_name = root.name();
    let dirs = [d.to_path_buf(), d.join("src")];
    for dir in &dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "crawform")
                && let Ok(other_doc) = Document::load(&path)
                && other_doc.inherits() == Some(my_name)
            {
                return true;
            }
        }
    }
    false
}

#[note("generator-plan-for")]
pub fn plan_for(
    ir: &Document,
    package: &str,
) -> Result<Vec<crawrad_codegen::Arquivo>, crawrad_codegen::Erro> {
    plan_for_with_bases(ir, package, &[], false)
}

#[note("generator-plan-for-with-bases")]
fn plan_for_with_bases(
    ir: &Document,
    package: &str,
    bases: &[Document],
    is_base: bool,
) -> Result<Vec<crawrad_codegen::Arquivo>, crawrad_codegen::Erro> {
    use crawrad_codegen::Gerador as _;
    project_generator().generate_with_bases(ir, package, catalog(), bases, is_base)
}

#[note("generator-load-bases")]
pub(crate) fn load_bases(destination: Option<&Path>, ir: &Document) -> Vec<Document> {
    let Some(d) = destination else {
        return Vec::new();
    };
    let mut bases: Vec<Document> = Vec::new();
    let mut next = ir.inherits().map(str::to_string);
    while let Some(name) = next {
        // a cycle is refused where the form is saved; here it only stops
        let seen = name == ir.root || bases.iter().any(|b| b.root == name);
        let p1 = d.join("src").join(format!("{name}.crawform"));
        let path = if p1.exists() {
            p1
        } else {
            d.join(format!("{name}.crawform"))
        };
        let Some(base) = (!seen).then(|| Document::load(&path).ok()).flatten() else {
            break;
        };
        next = base.inherits().map(str::to_string);
        bases.push(base);
    }
    bases
}

#[note("generator-plan-with-prepass")]
pub fn plan_with_prepass(
    destination: Option<&Path>,
    ir: &Document,
    package: &str,
    editing: &dyn Fn(&Path) -> Option<String>,
) -> (
    Vec<crawrad_codegen::Desligamento>,
    Result<Vec<crawrad_codegen::Arquivo>, crawrad_codegen::Erro>,
) {
    let is_base = is_base_of_any(destination, ir);
    let bases = load_bases(destination, ir);
    let expired = destination
        .map(|d| crawrad_codegen::stale_bindings_editing(d, ir, editing))
        .unwrap_or_default();
    let has_locales = destination.is_some_and(|d| !crate::locales::read_all(d).is_empty());
    let _scope = crawrad_codegen::LocalesScope::new(has_locales);
    let (expired, files) = if expired.is_empty() {
        (expired, plan_for_with_bases(ir, package, &bases, is_base))
    } else {
        // A COPY, and not the open document: what writes into the document is
        // `crawrad-doc`, through a transaction, and that is where undo comes from
        // (ADR 0006). This copy exists only to answer "what would the generator
        // say with the link already out", and it dies at the end of the function.
        let mut pruned = ir.clone();
        for x in &expired {
            for (node, event) in &x.vinculos {
                if let Some(n) = pruned.node_mut(node) {
                    n.events.remove(event);
                }
            }
        }
        (
            expired,
            plan_for_with_bases(&pruned, package, &bases, is_base),
        )
    };
    let files = files.map(|f| with_project_locales(destination, f));
    (expired, files)
}

// ------------------------------------------------------- the editor's side

#[note("generator-pendingedits")]
pub trait PendingEdits {
    #[note("generator-pendingedits-flush")]
    fn flush(&mut self) -> Result<Option<PathBuf>, String>;

    #[note("generator-pendingedits-reload")]
    fn reload(&mut self) -> Vec<PathBuf>;
}

/// A shell with no editor open — and the one the Makepad side starts from.
///
/// It is not a stub for tests: a shell without a code tab has nothing to
/// flush and nothing to reload, and saying so in the type is cheaper than a
/// pile of `Option` at the call site.
impl PendingEdits for () {
    fn flush(&mut self) -> Result<Option<PathBuf>, String> {
        Ok(None)
    }

    fn reload(&mut self) -> Vec<PathBuf> {
        Vec::new()
    }
}

#[note("generator-plan")]
pub struct Plan {
    /// The editor file that went to disk **before** the pre-pass, when there
    /// was one to write. It is what the bar names: saving without being asked
    /// is acceptable, saving without saying so is not.
    pub saved: Option<PathBuf>,
    /// The links the pre-pass found expired. The shell turns them into a
    /// document change through [`Plan::unlink`] — never this module.
    pub expired: Vec<crawrad_codegen::Desligamento>,
    /// The files [`write`](fn@write) would write, or why there are none.
    pub files: Result<Vec<crawrad_codegen::Arquivo>, crawrad_codegen::Erro>,
}

impl Plan {
    #[note("generator-plan-unlink")]
    pub fn unlink(&self) -> Option<Transaction> {
        if self.expired.is_empty() {
            return None;
        }
        let mut tx = Transaction::user(crawrad_codegen::ROTULO_DO_DESLIGAMENTO);
        for d in &self.expired {
            for (node, event) in &d.vinculos {
                tx = tx.with(Command::SetEvent {
                    node: node.clone(),
                    event: event.clone(),
                    handler: None,
                });
            }
        }
        Some(tx)
    }
}

#[note("generator-prepare")]
pub fn prepare(
    destination: &Path,
    pending: &mut dyn PendingEdits,
    ir: &Document,
    package: &str,
) -> Result<Plan, String> {
    // THIS LINE COMES FIRST. Moving it after `plan_with_prepass` gives the bug
    // back whole.
    let saved = pending.flush().map_err(refusal)?;
    let (expired, files) = plan_with_prepass(Some(destination), ir, package, &|_| None);
    Ok(Plan {
        saved,
        expired,
        files,
    })
}

#[note("generator-write")]
pub fn write(destination: &Path, plan: Plan, pending: &mut dyn PendingEdits) -> String {
    // ATTENTION: the generation target is NOT the selector on the bar. That one
    // switches the **design surface** — who draws the form inside the IDE. The
    // target belongs to the PROJECT (ADR 0004), and it is what the bar names
    // here: suggesting a choice that `Run` does not honour is the worst way of
    // being wrong.
    use crawrad_codegen::Gerador as _;
    let target = project_generator().alvo();
    let files = match plan.files {
        Ok(f) => with_project_icon(destination, f),
        Err(e) => return refusal(e),
    };
    let files = with_project_locales(Some(destination), files);
    match crawrad_codegen::escrever(destination, &files) {
        Ok(report) => {
            let stale = pending.reload();
            let mut sentence = generate_sentence(
                plan.saved.as_deref(),
                report_sentence(target, destination, &report),
            );
            if let Some(note) = stale_note(&stale) {
                sentence.push_str(&note);
            }
            sentence
        }
        Err(e) => refusal(e),
    }
}

/// A project with its own `icon.png` (Project Options › Application) shows it in
/// the running egui window; Makepad takes it from the exe's resource 1.
fn with_project_icon(
    dir: &Path,
    mut files: Vec<crawrad_codegen::Arquivo>,
) -> Vec<crawrad_codegen::Arquivo> {
    if dir.join(crate::project_options::ICON_PNG).is_file() {
        for f in files
            .iter_mut()
            .filter(|f| f.caminho == Path::new("src/main.rs"))
        {
            f.conteudo = crawrad_codegen::targets::egui::with_window_icon(&f.conteudo);
        }
    }
    files
}

/// Adds `src/locales.rs` and ensures `mod locales;` in `main.rs` when the project ships locales.
fn with_project_locales(
    dir: Option<&Path>,
    mut files: Vec<crawrad_codegen::Arquivo>,
) -> Vec<crawrad_codegen::Arquivo> {
    let Some(d) = dir else {
        return files;
    };
    let entries = crate::locales::read_all(d);
    if entries.is_empty() {
        return files;
    }
    if !files
        .iter()
        .any(|f| f.caminho == Path::new("src/locales.rs"))
    {
        let locales_rs = crawrad_codegen::locales::locales_module(&entries);
        files.push(crawrad_codegen::Arquivo {
            caminho: PathBuf::from("src/locales.rs"),
            conteudo: locales_rs,
            dono: crawrad_codegen::Dono::Gerador,
            adicoes: Vec::new(),
        });
    }
    for f in files
        .iter_mut()
        .filter(|f| f.caminho == Path::new("src/main.rs"))
    {
        if !f.conteudo.contains("mod locales;") {
            if let Some(pos) = f.conteudo.find("pub mod ") {
                f.conteudo.insert_str(pos, "mod locales;\n\n");
            } else if let Some(pos) = f.conteudo.find("fn main()") {
                f.conteudo.insert_str(pos, "mod locales;\n\n");
            } else {
                f.conteudo.insert_str(0, "mod locales;\n");
            }
        }
    }
    files
}

#[note("generator-generate-for-new-project")]
pub fn generate_for_new_project(member: &Path, doc: &Document) -> Result<(), String> {
    generate_for_new_project_with(member, doc, active_target())
}

#[note("generator-generate-for-new-project-with")]
pub fn generate_for_new_project_with(
    member: &Path,
    doc: &Document,
    target: crawrad_codegen::AlvoGerador,
) -> Result<(), String> {
    use crawrad_codegen::Gerador as _;
    let has_locales = !crate::locales::read_all(member).is_empty();
    let _scope = crawrad_codegen::LocalesScope::new(has_locales);
    let files = target
        .create_generator()
        .gerar(doc, &package_name_in(member), catalog())
        .map_err(|e| format!("project created, but the code was not generated: {e}"))?;
    let files = with_project_icon(member, files);
    let files = with_project_locales(Some(member), files);
    crawrad_codegen::escrever(member, &files)
        .map_err(|e| format!("project created, but the code was not generated: {e}"))?;
    Ok(())
}

// ----------------------------------------------------------- the sentences

#[note("generator-generate-sentence")]
pub fn generate_sentence(saved: Option<&Path>, generation: String) -> String {
    match saved {
        Some(c) if !refused(&generation) => {
            format!("{} — {generation}", saved_sentence(c))
        }
        _ => generation,
    }
}

#[note("generator-saved-sentence")]
pub fn saved_sentence(path: &Path) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    format!("saved {name}")
}

#[note("generator-report-sentence")]
pub fn report_sentence(target: &str, destination: &Path, r: &crawrad_codegen::Relatorio) -> String {
    let mut sentence = format!(
        "generated for {} in {} — {} new, {} rewritten, {} kept, {} grown{}",
        target,
        destination.display(),
        r.criados.len(),
        r.reescritos.len(),
        r.preservados.len(),
        r.cresceram.len(),
        // Deleting a file is the only thing here that cannot be undone by
        // looking: when it happens, it shows.
        if r.removidos.is_empty() && r.renomeados.is_empty() {
            String::new()
        } else {
            format!(
                ", {} renamed, {} stale removed",
                r.renomeados.len(),
                r.removidos.len()
            )
        },
    );
    if let Some(pruning) = r.frase_da_poda() {
        sentence.push_str(&format!(" — {pruning}"));
    }
    sentence
}

#[note("generator-stale-note")]
pub fn stale_note(stale: &[PathBuf]) -> Option<String> {
    if stale.is_empty() {
        return None;
    }
    let names: Vec<String> = stale
        .iter()
        .map(|c| {
            c.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| c.display().to_string())
        })
        .collect();
    Some(format!(
        " — the Code tab still shows the old {} (unsaved edits kept)",
        names.join(", ")
    ))
}

#[note("generator-should-generate-on-save")]
pub fn should_generate_on_save(write_failed: bool, has_document: bool) -> bool {
    !write_failed && has_document
}

/// The project kind for the status bar (console or library when there is no form
/// and no UI target) and whether the project has no form.
pub fn project_kind_now(
    open: bool,
    has_forms: bool,
    has_binary: bool,
    ui_target_is_some: bool,
) -> (Option<&'static str>, bool) {
    let without_form = open && !has_forms;
    let kind = if has_binary { "console" } else { "library" };
    let kind = (without_form && !ui_target_is_some).then_some(kind);
    (kind, without_form)
}

// THE TESTS OF THIS MODULE ARE DECLARED HERE AGAIN, and the debt written in
// this spot is the reason they can be.
//
// It used to say: *"a shared module whose tests only compile from one of its
// two consumers is a module with its fixture in the wrong place. Closing it
// means moving `fixture` and `projeto_de_mentira` into a file of their own
// that both roots declare"*.
//
// ADR 0032 took the egui shell away, the debt came due, and the fixture is now
// `src/test/fixture/test.rs` — declared by the one root that is left.
#[cfg(test)]
#[path = "test/generator/test.rs"]
mod tests;
