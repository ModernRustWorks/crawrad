//! The EXPLORER panel of the Makepad shell — the rows it draws, and the two
//! pieces of state it owns.
//!
//! # What this module is NOT
//!
//! It is not a second Explorer. Everything that decides **who owns a file** —
//! crossing the generation plan with the disk, reading the group manifest,
//! choosing the badge word — lives in [`model`], which both shells include by
//! `#[path]`. What lives here is the shape of the list: the order of the rows,
//! the indentation, and which template each one asks for.
//!
//! That split is the whole point of the round. The egui panel paints the same
//! information with `CollapsingHeader` and a hand-painted row; this one feeds a
//! `PortalList`, because the `live_design!` is static and there is no public
//! API to inject DSL at runtime (ADR 0027). Two painters, one truth.
//!
//! # A row is a COMPONENT, not a variant
//!
//! Each kind of row is a file under [`rows`](mod@rows), implementing [`row::ExplorerRow`]
//! and owning everything about itself — template, indent, drawing, badge,
//! tooltip and the catalogue command a click runs. Adding a kind costs
//! creating a file. What used to be here was one `enum` with five `match`es
//! over it, in three files, and two of those `match`es ended in a wildcard, so
//! a new variant reached the screen drawn as something else with nothing
//! failing.
//!
//! # The fold state, and why it cannot be forgotten
//!
//! The egui panel keeps what is open inside the memory of the framework, keyed
//! by the member folder (`explorer.rs`, `.id_salt(&m.pasta)`), and its own
//! comment says why it is not a field of the IDE:
//!
//! > *"a field there would be one more state to clear when the document
//! > changes, and forgetting to clear it has already cost dearly in this
//! > house."*
//!
//! Makepad has no such implicit memory, so the state needs an explicit owner —
//! and with it comes back exactly the risk egui was avoiding for free. The
//! answer here is [`Folds`], and it is a GUARD and not a discipline: the state
//! remembers **which document it belongs to**, and every read hands it the
//! document of the moment. A different document rebuilds it on the spot, so
//! there is no call anybody can forget to make.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crawrad_codegen::{Arquivo, Erro};

use crate::workspace;

// The `#[path]` are NOT decorative, and what happens without them was measured.
//
// A module loaded via `#[path]` does not get a subfolder: its children are
// searched in the folder of the file ITSELF, not in a folder named after it.
// Since this file is `ide_app/explorer.rs`, a bare `pub mod icon;`
// would resolve to `ide_app/icon.rs` — which exists, and is the chrome table of
// the rest of the shell. The module would compile, pointing to another file, and the error
// would only show up on the first missing name.
pub use crate::explorer_rows::{icon, problems, row, rows};
pub use problems::{ProblemMap, Problems};

pub use row::{Activation, Row};
pub use rows::CHILD;

use rows::caption::Caption;
use rows::file::File;
use rows::form::Form;
use rows::group::{GroupName, GroupPath};
use rows::legend::Legend;
use rows::member::Member;
use rows::note::Note;
use rows::section::Section;
use rows::summary::Summary;
use rows::timeline_entry::TimelineEntry;

// The headless Explorer, through the SAME module path in both binaries.
//
// `crate::explorer::model` is the file `src/explorer_model.rs`, and each binary
// declares it ONCE: in egui's, it is `explorer.rs` itself that opens it; in Makepad's,
// where `explorer.rs` does not exist, the binary root mounts an `explorer` module
// solely to hold the same name.
//
// A `#[path]` here would be the short path and clippy rejects it: `duplicate_mod`
// — in egui's binary the file would enter as `explorer::model` AND as
// `ide_app::explorer::model`, with two distinct `Line` types that never meet.
// The unique address is what prevents that.
pub use crate::explorer_model as model;
use crawrad_note::note;

#[note("explorer-no-folder")]
pub const NO_FOLDER: &str = "No folder open.";

#[note("explorer-form-outside")]
pub const FORM_OUTSIDE: &str = "the open form is not inside a <member>/src/ folder";

#[note("explorer-caption-workspace")]
pub const CAPTION_WORKSPACE: &str = "Workspace";

#[note("explorer-caption-forms")]
pub const CAPTION_FORMS: &str = "Forms";

#[note("explorer-caption-legend")]
pub const CAPTION_LEGEND: &str = "What the badges mean";

#[note("explorer-action-toggle-legend")]
pub const ACTION_TOGGLE_LEGEND: &str = "explorer.toggle_legend";
pub const ACTION_TOGGLE_OPEN_EDITORS: &str = "explorer.toggle_open_editors";
pub const ACTION_TOGGLE_OUTLINE: &str = "explorer.toggle_outline";
pub const ACTION_TOGGLE_TIMELINE: &str = "explorer.toggle_timeline";
pub const ACTION_OPEN_TIMELINE_ENTRY: &str = "explorer.open_timeline_entry";
pub const ACTION_TOGGLE_DEPENDENCIES: &str = "explorer.toggle_dependencies";

pub const CAPTION_OPEN_EDITORS: &str = "Open Editors";
pub const CAPTION_OUTLINE: &str = "Outline";
pub const CAPTION_TIMELINE: &str = "Timeline";
pub const CAPTION_DEPENDENCIES: &str = "Rust Dependencies";

/// Which explorer sections are open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionFolds<'a> {
    pub folds: &'a BTreeSet<PathBuf>,
    pub legend: bool,
    pub open_editors: bool,
    pub outline: bool,
    pub timeline: bool,
    pub dependencies: bool,
}

#[note("explorer-folds")]
#[derive(Debug, Default)]
pub struct Folds {
    /// The document the state below belongs to. `None` before the first read,
    /// and after a document that has no path.
    document: Option<PathBuf>,
    /// The member folders the user has open.
    open: BTreeSet<PathBuf>,
    /// Is the legend section open?
    ///
    /// **Closed is the default, and that is the whole point.** A `bool` and
    /// not a second set: there is one section, and a set of one invites a
    /// second concept in without anybody deciding to let it.
    legend: bool,
    open_editors: bool,
    outline: bool,
    timeline: bool,
    dependencies: bool,
}

impl Folds {
    #[note("explorer-folds-open")]
    pub fn open(&mut self, document: Option<&Path>, active: Option<&Path>) -> &BTreeSet<PathBuf> {
        self.rebuild_if_needed(document, active);
        &self.open
    }

    #[note("explorer-folds-legend-open")]
    pub fn legend_open(&mut self, document: Option<&Path>, active: Option<&Path>) -> bool {
        self.rebuild_if_needed(document, active);
        self.legend
    }

    pub fn open_editors_open(&mut self, document: Option<&Path>, active: Option<&Path>) -> bool {
        self.rebuild_if_needed(document, active);
        self.open_editors
    }

    pub fn outline_open(&mut self, document: Option<&Path>, active: Option<&Path>) -> bool {
        self.rebuild_if_needed(document, active);
        self.outline
    }

    pub fn timeline_open(&mut self, document: Option<&Path>, active: Option<&Path>) -> bool {
        self.rebuild_if_needed(document, active);
        self.timeline
    }

    pub fn dependencies_open(&mut self, document: Option<&Path>, active: Option<&Path>) -> bool {
        self.rebuild_if_needed(document, active);
        self.dependencies
    }

    /// The open state of each section for this document and active file.
    pub fn sections(&mut self, document: Option<&Path>, active: Option<&Path>) -> SectionFolds<'_> {
        self.rebuild_if_needed(document, active);
        SectionFolds {
            folds: &self.open,
            legend: self.legend,
            open_editors: self.open_editors,
            outline: self.outline,
            timeline: self.timeline,
            dependencies: self.dependencies,
        }
    }

    #[note("explorer-folds-toggle")]
    pub fn toggle(&mut self, document: Option<&Path>, active: Option<&Path>, folder: &Path) {
        self.rebuild_if_needed(document, active);
        if !self.open.remove(folder) {
            self.open.insert(folder.to_path_buf());
        }
    }

    #[note("explorer-folds-toggle-legend")]
    pub fn toggle_legend(&mut self, document: Option<&Path>, active: Option<&Path>) {
        self.rebuild_if_needed(document, active);
        self.legend = !self.legend;
    }

    pub fn toggle_open_editors(&mut self, document: Option<&Path>, active: Option<&Path>) {
        self.rebuild_if_needed(document, active);
        self.open_editors = !self.open_editors;
    }

    pub fn toggle_outline(&mut self, document: Option<&Path>, active: Option<&Path>) {
        self.rebuild_if_needed(document, active);
        self.outline = !self.outline;
    }

    pub fn toggle_timeline(&mut self, document: Option<&Path>, active: Option<&Path>) {
        self.rebuild_if_needed(document, active);
        self.timeline = !self.timeline;
    }

    pub fn toggle_dependencies(&mut self, document: Option<&Path>, active: Option<&Path>) {
        self.rebuild_if_needed(document, active);
        self.dependencies = !self.dependencies;
    }

    #[note("explorer-folds-collapse-all")]
    pub fn collapse_all(&mut self, document: Option<&Path>, active: Option<&Path>) {
        self.rebuild_if_needed(document, active);
        self.open.clear();
        self.legend = false;
        self.open_editors = false;
        self.outline = false;
        self.timeline = false;
        self.dependencies = false;
    }

    #[note("explorer-folds-rebuild-if-needed")]
    fn rebuild_if_needed(&mut self, document: Option<&Path>, active: Option<&Path>) {
        let same = match (&self.document, document) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        };
        if !same {
            self.document = document.map(Path::to_path_buf);
            self.open = active.map(Path::to_path_buf).into_iter().collect();
            // The legend and cards return CLOSED together.
            self.legend = false;
            self.open_editors = false;
            self.outline = false;
            self.timeline = false;
            self.dependencies = false;
        }
    }
}

#[note("explorer-group-root-of")]
pub fn group_root_of(document: &Path) -> Option<PathBuf> {
    let member = workspace::document_member(document)?;
    let parent = member.parent()?;
    if workspace::members(parent)
        .iter()
        .any(|m| model::same_location(m, &member))
    {
        return Some(parent.to_path_buf());
    }
    Some(member)
}

#[note("explorer-view")]
pub struct View<'a> {
    /// The group folder, or `None` when the open form is not inside a member.
    pub root: Option<&'a Path>,
    /// The form on screen.
    pub document: Option<&'a Path>,
    /// What the next `Generate` would write, or why it cannot be planned.
    pub plan: &'a Option<Result<Vec<Arquivo>, Erro>>,
    /// The member folders that are open.
    pub folds: &'a BTreeSet<PathBuf>,
    /// The errors and warnings the language server published, by file.
    pub problems: &'a crate::explorer_rows::problems::ProblemMap,
    /// The files with unsaved editor buffer edits, keyed by problems::key.
    pub unsaved: &'a std::collections::BTreeSet<String>,
    /// Is the legend section open?
    pub legend_open: bool,
    /// Are the card sections open?
    pub open_editors_open: bool,
    pub outline_open: bool,
    pub timeline_open: bool,
    pub dependencies_open: bool,
    /// The file's local history lines, newest first; the shell fills it only when the section is open.
    /// `None` when the local history is turned off in Options.
    pub timeline: Option<&'a [String]>,
}

impl<'a> View<'a> {
    /// The view built from the open sections.
    pub fn from_sections(
        root: Option<&'a Path>,
        document: Option<&'a Path>,
        plan: &'a Option<Result<Vec<Arquivo>, Erro>>,
        sections: SectionFolds<'a>,
        problems: &'a crate::explorer_rows::problems::ProblemMap,
        unsaved: &'a BTreeSet<String>,
        timeline: Option<&'a [String]>,
    ) -> Self {
        Self {
            root,
            document,
            plan,
            folds: sections.folds,
            problems,
            unsaved,
            legend_open: sections.legend,
            open_editors_open: sections.open_editors,
            outline_open: sections.outline,
            timeline_open: sections.timeline,
            dependencies_open: sections.dependencies,
            timeline,
        }
    }
}

#[note("explorer-rows")]
pub fn rows(view: &View) -> Vec<Row> {
    let Some(root) = view.root else {
        return vec![Box::new(Note::new(NO_FOLDER, false, 0))];
    };

    let members = model::group_tree(root, view.document);
    let lines = match (members.iter().find(|m| m.active), view.plan) {
        (Some(m), Some(Ok(a))) => model::intersect(a, &m.folder),
        _ => Vec::new(),
    };

    let mut out: Vec<Row> = vec![
        Box::new(Caption::new(CAPTION_WORKSPACE, 0)),
        Box::new(GroupName::new(model::name_of(root)).at(root)),
        Box::new(GroupPath::new(root.display().to_string())),
    ];

    if view.document.is_some() && !members.iter().any(|m| m.active) {
        out.push(Box::new(Note::new(FORM_OUTSIDE, true, 0)));
    }

    for m in &members {
        let open = view.folds.contains(&m.folder);
        out.push(Box::new(
            Member::new(
                &m.folder,
                model::name_of(&m.folder),
                model::member_badge(m),
                open,
            )
            .with_problems(Problems::under(view.problems, &m.folder)),
        ));
        if open {
            member_body(&mut out, m, view, &lines);
        }
    }

    let legend = model::legend(&lines, &members);
    if !legend.is_empty() {
        // THE HEADER ENTERS EVEN WHEN CLOSED, and that is what separates "collapsed"
        // from "deleted": closed, the section remains on screen as a single line, and
        // whoever never saw the badges has where to ask. It is what VS Code's Outline does.
        out.push(Box::new(Section::new(
            CAPTION_LEGEND,
            view.legend_open,
            ACTION_TOGGLE_LEGEND,
        )));
        if view.legend_open {
            out.extend(
                legend
                    .into_iter()
                    .map(|l| -> Row { Box::new(Legend::new(l.mark, l.phrase, l.warn)) }),
            );
        }
    }

    // Additional card sections in VS Code style (Ref 29)
    out.push(Box::new(Section::new(
        CAPTION_OUTLINE,
        view.outline_open,
        ACTION_TOGGLE_OUTLINE,
    )));
    if view.outline_open {
        if let Some(doc) = view.document {
            out.push(Box::new(Note::new(
                format!("Form: {}", model::name_of(doc)),
                false,
                CHILD,
            )));
        } else {
            out.push(Box::new(Note::new("No active form", false, CHILD)));
        }
    }

    out.push(Box::new(Section::new(
        CAPTION_TIMELINE,
        view.timeline_open,
        ACTION_TOGGLE_TIMELINE,
    )));
    if view.timeline_open {
        match view.timeline {
            None => out.push(Box::new(Note::new(
                "Local history is off \u{2014} Tools \u{203a} Options \u{203a} History",
                false,
                CHILD,
            ))),
            Some([]) => out.push(Box::new(Note::new(
                "No local history for this file",
                false,
                CHILD,
            ))),
            Some(lines) => {
                for (index, line) in lines.iter().enumerate() {
                    out.push(Box::new(TimelineEntry::new(index, line)));
                }
            }
        }
    }

    out.push(Box::new(Section::new(
        CAPTION_DEPENDENCIES,
        view.dependencies_open,
        ACTION_TOGGLE_DEPENDENCIES,
    )));
    if view.dependencies_open {
        out.push(Box::new(Note::new("makepad-widgets 1.0.0", false, CHILD)));
        out.push(Box::new(Note::new("crawrad-runtime 0.1.0", false, CHILD)));
    }

    out.push(Box::new(Section::new(
        CAPTION_OPEN_EDITORS,
        view.open_editors_open,
        ACTION_TOGGLE_OPEN_EDITORS,
    )));
    if view.open_editors_open {
        if let Some(doc) = view.document {
            out.push(Box::new(Note::new(
                format!("Active editor: {}", model::name_of(doc)),
                false,
                CHILD,
            )));
        } else {
            out.push(Box::new(Note::new("No active editor", false, CHILD)));
        }
    }
    out
}

#[note("explorer-member-body")]
fn member_body(out: &mut Vec<Row>, m: &model::MemberNode, view: &View, lines: &[model::Line]) {
    let forms = workspace::member_documents(&m.folder);
    if !forms.is_empty() {
        out.push(Box::new(Caption::new(CAPTION_FORMS, CHILD)));
        for f in &forms {
            out.push(Box::new(Form::new(
                f,
                model::name_of(f),
                view.document.is_some_and(|d| model::same_location(d, f)),
            )));
        }
    }

    // **The decision is not taken here.** `member_files` answers "may
    // this member show files with a badge?", and it answers with EITHER the
    // slice OR the sentence that goes in its place — so forgetting to keep
    // quiet stops compiling instead of attributing one project's files to
    // another.
    let lines = match model::member_files(m, view.plan, lines, !forms.is_empty()) {
        Ok(l) => l,
        Err(model::Silence::Phrase(s)) => {
            out.push(Box::new(Note::new(s, false, CHILD)));
            return;
        }
        Err(model::Silence::CannotPlan(e)) => {
            out.push(Box::new(Note::new(
                format!("cannot plan: {e}"),
                true,
                CHILD,
            )));
            return;
        }
        Err(model::Silence::Empty) => return,
    };

    out.push(Box::new(Summary::new(model::summary(lines), CHILD)));

    // Grouped by folder and nothing more: one level of grouping says where the
    // file lands without turning into a navigation tree, which is the
    // non-objective of ADR 0016.
    let mut folder: Option<PathBuf> = None;
    for l in lines {
        let dir = l.path.parent().unwrap_or(Path::new("")).to_path_buf();
        if folder.as_deref() != Some(dir.as_path()) {
            if let Some(label) = model::folder_label(&dir) {
                out.push(Box::new(Caption::new(label, CHILD)));
            }
            folder = Some(dir);
        }
        let that_path = m.folder.join(&l.path);
        let problems = Problems::of(view.problems, &that_path);
        let unsaved = view.unsaved.contains(&problems::key(&that_path));
        let file = File::new(&l.path, l.state, l.owner, CHILD)
            .with_problems(problems)
            .with_unsaved(unsaved);
        out.push(Box::new(file));
    }
}

#[note("explorer-rows-json")]
pub fn rows_json(rows: &[Row], rects: &[(usize, [f64; 4])]) -> serde_json::Value {
    let one = |(i, r): (usize, &Row)| {
        let rect = rects.iter().find(|(n, _)| *n == i).map(|(_, r)| r);
        serde_json::json!({ "template": r.template(), "icon": r.icon(), "text": r.text(), "badge": r.badge(), "indent": r.indent(), "rect": rect })
    };
    serde_json::Value::Array(rows.iter().enumerate().map(one).collect())
}

#[note("explorer-sample-rows")]
pub fn sample_rows() -> Vec<Row> {
    use crawrad_codegen::Dono;
    use model::FileState;
    let file = |state, owner| -> Row {
        Box::new(File::new(Path::new("src/main.rs"), state, owner, CHILD))
    };
    vec![
        file(FileState::OnDisk, Some(Dono::Gerador)),
        file(FileState::OnDisk, Some(Dono::Usuario)),
        file(FileState::Pending, Some(Dono::PrimeiraVez)),
        file(FileState::User, Some(Dono::Usuario)),
        file(FileState::Stale, None),
        // The manifest, which is the only file in the plan with its own drawing.
        Box::new(File::new(
            Path::new("Cargo.toml"),
            FileState::User,
            Some(Dono::Usuario),
            CHILD,
        )),
        // THE LONGEST NAME THE LIST SHOWS, and it is here because of a defect:
        // with `face` on `RightWrap`, this was the name pushing `Rewritten`
        // down to the next line, where it read as another tree item.
        // See `the_badge_can_never_drop_to_the_next_line`.
        Box::new(File::new(
            Path::new("src/formPrincipal.designer.rs"),
            FileState::OnDisk,
            Some(Dono::Gerador),
            CHILD,
        )),
        Box::new(Note::new(NO_FOLDER, false, 0)),
        Box::new(Note::new(FORM_OUTSIDE, true, 0)),
        Box::new(Caption::new(CAPTION_WORKSPACE, 0)),
        Box::new(GroupName::new("app")),
        Box::new(GroupPath::new("C:/app")),
        Box::new(Member::new(
            Path::new("app"),
            "app",
            Some(model::MARK_ACTIVE),
            false,
        )),
        Box::new(Member::new(Path::new("app"), "app", None, true)),
        // The problems the language server published colour a row.
        Box::new(
            Member::new(Path::new("app"), "app", None, true).with_problems(Problems {
                errors: 1,
                warnings: 0,
            }),
        ),
        Box::new(
            Member::new(Path::new("app"), "app", None, true).with_problems(Problems {
                errors: 0,
                warnings: 2,
            }),
        ),
        Box::new(
            File::new(
                Path::new("src/main.rs"),
                FileState::OnDisk,
                Some(Dono::Usuario),
                CHILD,
            )
            .with_problems(Problems {
                errors: 1,
                warnings: 2,
            }),
        ),
        Box::new(
            File::new(
                Path::new("src/lib.rs"),
                FileState::OnDisk,
                Some(Dono::Usuario),
                CHILD,
            )
            .with_problems(Problems {
                errors: 0,
                warnings: 1,
            }),
        ),
        Box::new(
            File::new(
                Path::new("src/buffer.rs"),
                FileState::OnDisk,
                Some(Dono::Usuario),
                CHILD,
            )
            .with_unsaved(true),
        ),
        Box::new(Form::new(
            Path::new("form_main.crawform"),
            "form_main.crawform",
            false,
        )),
        Box::new(Form::new(
            Path::new("form_main.crawform"),
            "form_main.crawform",
            true,
        )),
        Box::new(Summary::new("1 file", CHILD)),
        Box::new(Section::new(CAPTION_LEGEND, false, ACTION_TOGGLE_LEGEND)),
        Box::new(Section::new(CAPTION_LEGEND, true, ACTION_TOGGLE_LEGEND)),
        Box::new(Section::new(
            CAPTION_OPEN_EDITORS,
            false,
            ACTION_TOGGLE_OPEN_EDITORS,
        )),
        Box::new(Section::new(CAPTION_OUTLINE, false, ACTION_TOGGLE_OUTLINE)),
        Box::new(Section::new(
            CAPTION_TIMELINE,
            false,
            ACTION_TOGGLE_TIMELINE,
        )),
        Box::new(TimelineEntry::new(0, "Save · 14:32:10 · today")),
        Box::new(Section::new(
            CAPTION_DEPENDENCIES,
            false,
            ACTION_TOGGLE_DEPENDENCIES,
        )),
        Box::new(Legend::new(
            model::MARK_ACTIVE,
            "Generate and Run write here",
            false,
        )),
        Box::new(Legend::new(
            model::BADGE_STALE,
            "from a form that is gone",
            true,
        )),
    ]
}

/// `p` joined to `base` when relative.
pub fn resolve_path(base: Option<&Path>, p: &Path) -> PathBuf {
    let base = base.unwrap_or_else(|| Path::new("."));
    if p.as_os_str().is_empty() {
        base.to_path_buf()
    } else {
        base.join(p)
    }
}

/// The folder a new item goes into: the path itself when a folder, else its parent.
pub fn target_folder(base: Option<&Path>, p: Option<&Path>) -> PathBuf {
    if let Some(caminho) = p.filter(|c| !c.as_os_str().is_empty()) {
        let resolvido = resolve_path(base, caminho);
        if resolvido.is_dir() {
            resolvido
        } else if let Some(pai) = resolvido.parent() {
            pai.to_path_buf()
        } else {
            resolvido
        }
    } else {
        let base = resolve_path(base, Path::new(""));
        let src = base.join("src");
        if src.exists() { src } else { base }
    }
}

/// Deletes a file or a whole folder.
pub fn delete_path(target: &Path) -> std::io::Result<()> {
    if target.is_file() {
        std::fs::remove_file(target)
    } else if target.is_dir() {
        std::fs::remove_dir_all(target)
    } else {
        Ok(())
    }
}

/// Shows the path in the system's file manager.
pub fn reveal_in_file_manager(target: &Path) {
    #[cfg(windows)]
    {
        let mut cmd = crawrad_os::process::no_console("explorer");
        if target.is_file() {
            cmd.arg(format!("/select,{}", target.display()));
        } else {
            cmd.arg(target);
        }
        let _ = cmd.spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = crawrad_os::process::no_console("open")
            .arg("-R")
            .arg(target)
            .spawn();
    }
    #[cfg(all(not(windows), not(target_os = "macos")))]
    {
        let folder = if target.is_file() {
            target.parent().unwrap_or(target)
        } else {
            target
        };
        let _ = crawrad_os::process::no_console("xdg-open")
            .arg(folder)
            .spawn();
    }
}

#[cfg(test)]
#[path = "test/explorer/test.rs"]
mod explorer_test;
