//! The **headless** Explorer — crossing the generation plan with the disk, the
//! group tree, and the words the interface writes.
//!
//! # Why it is a separate file
//!
//! The panel logic — why it shows owner, why it shows the group, and why it
//! keeps quiet on a member without a plan — lives in [`crate::explorer`], which
//! paints it in egui. What lives HERE is the framework-independent part, in
//! its own file for a concrete reason: the Makepad shell is another `[[bin]]`,
//! with another module root, and a `mod` does not cross from one binary to
//! another. Without this separation, wiring the Makepad dock Explorer would mean
//! rewriting `intersect` and `group_tree` on the other side — two copies of the
//! rule deciding **who owns each file**, with the second aging silently.
//!
//! Both shells include THIS file via `#[path]`, which is why its tests cover both.
//!
//! What is NOT here is everything that needs `egui::Ui` or `Palette`: the
//! color of a line, the click target, the tooltip. Those stayed in
//! [`crate::explorer`], because the Makepad shell paints the same information
//! with other pieces.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crawrad_codegen::{Arquivo, Dono};

use crate::workspace::MANIFEST;
use crawrad_note::note;

#[note("explorer-model-filestate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileState {
    /// Planned and already on disk. The normal case.
    OnDisk,
    /// Planned and not yet generated.
    Pending,
    /// On disk, outside the plan, and **belongs to the user**: `utils.rs` they
    /// wrote, or `Cargo.toml` created by `cargo new`.
    ///
    /// This variant was born with Amendment 1 of [ADR 0017], and it fixes a
    /// defect that the new layout would have created. Previously, generated code lived
    /// in a folder of its own and every `.rs` outside the plan was indeed leftover.
    /// Now `src/` is shared — design, generated code, and user code side by
    /// side —, and calling their file `stale`, in warning color, with a tooltip
    /// saying "delete when sure", would be the UI offering the most expensive
    /// advice it could possibly give.
    User,
    /// On disk, outside the plan, and next `Generate` **deletes it**: a
    /// `*.designer.rs` from a form that no longer exists.
    ///
    /// The suffix is the boundary, not a choice of this screen: `sweep_orphans`
    /// only deletes `*.designer.rs`, and this badge states exactly what that
    /// code does.
    Stale,
}

#[note("explorer-model-line")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    /// Relative to the generated project root, like [`Arquivo::caminho`].
    pub path: PathBuf,
    /// `None` only for [`FileState::Stale`] — files outside the plan have no owner
    /// we can assert, and inventing one is worse than keeping silent. It is an
    /// `Option` for the compiler to demand this case from whoever draws the line.
    pub owner: Option<Dono>,
    pub state: FileState,
}

#[note("explorer-model-intersect")]
pub fn intersect(plan: &[Arquivo], member: &Path) -> Vec<Line> {
    let planned: BTreeSet<&Path> = plan.iter().map(|a| a.caminho.as_path()).collect();

    let mut lines: Vec<Line> = plan
        .iter()
        .map(|a| Line {
            path: a.caminho.clone(),
            owner: Some(a.dono),
            // `is_file` and not `exists`: a FOLDER named `src/main.rs` is not
            // the generated file, and saying that it is already there would hide
            // the problem instead of showing it.
            state: if member.join(&a.caminho).is_file() {
                FileState::OnDisk
            } else {
                FileState::Pending
            },
        })
        .collect();

    if member.join(MANIFEST).is_file() {
        lines.push(Line {
            path: PathBuf::from(MANIFEST),
            owner: Some(Dono::Usuario),
            state: FileState::User,
        });
    }

    for name in sources_on_disk(member) {
        let path = PathBuf::from("src").join(&name);
        if planned.contains(path.as_path()) {
            continue;
        }
        // **The boundary is the suffix, and it is the same as `sweep_orphans`.** Only
        // `*.designer.rs` outside the plan is stale, because only it is deleted by
        // the next `Generate`. Any other `.rs` in `src/` belongs to the user — and
        // since Amendment 1 of ADR 0017 the folder is shared, so this stopped being
        // a rare case to become the normal case.
        let (state, owner) = if name.ends_with(crawrad_codegen::SUFIXO_GERADO) {
            (FileState::Stale, None)
        } else {
            (FileState::User, Some(Dono::Usuario))
        };
        lines.push(Line { path, owner, state });
    }

    // Order by path, and not by generator order: the panel groups by folder
    // immediately following, and an out-of-order list would make the same folder
    // appear twice.
    lines.sort_by(|a, b| a.path.cmp(&b.path));
    lines
}

#[note("explorer-model-sources-on-disk")]
fn sources_on_disk(member: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(member.join("src")) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.ends_with(".rs"))
        .collect()
}

// -------------------------------------------------------------------- the group

#[note("explorer-model-membernode")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberNode {
    /// Folder of the member, absolute, as [`crate::workspace::members`] returns.
    pub folder: PathBuf,
    /// Member of the **open document**.
    ///
    /// And that is why it is the `active` of the UI: `Generate` writes to
    /// `<this member>/src/` and `Run` compiles `cargo build -p <its package>`
    /// — both derived from the open document, and nothing else.
    pub active: bool,
    /// Listed in `default-members` of the group manifest.
    ///
    /// It is the *active project* of [ADR 0017] — what a `cargo run` in root, without
    /// `-p`, would choose. Almost always the same as [`MemberNode::active`], because
    /// it is from where the IDE extracts the form to open. When the two **disagree**,
    /// the UI names both: hiding one of them would be the panel choosing which of the
    /// two truths to tell, and both rule over different things.
    ///
    pub is_default: bool,
}

#[note("explorer-model-group-tree")]
pub fn group_tree(root: &Path, document: Option<&Path>) -> Vec<MemberNode> {
    let open = document.and_then(crate::workspace::document_member);
    // Read here and not in `workspace::members`: it returns the list TOGETHER, with
    // active in front, and information on which was which is lost along the way.
    let manifest = std::fs::read_to_string(root.join(MANIFEST)).unwrap_or_default();
    let defaults: Vec<PathBuf> = crate::workspace::workspace_list(&manifest, "default-members")
        .into_iter()
        .map(|n| root.join(n))
        .collect();

    let mut nodes: Vec<MemberNode> = crate::workspace::members(root)
        .into_iter()
        .map(|folder| MemberNode {
            active: open.as_deref().is_some_and(|a| same_location(a, &folder)),
            is_default: defaults.iter().any(|d| same_location(d, &folder)),
            folder,
        })
        .collect();
    // The order the group declares: switching the active project only
    // changes its colour, never its place (owner, 13/11).
    let declared: Vec<PathBuf> = crate::workspace::workspace_list(&manifest, "members")
        .into_iter()
        .map(|n| root.join(n))
        .collect();
    nodes.sort_by_key(|m| {
        declared
            .iter()
            .position(|d| same_location(d, &m.folder))
            .unwrap_or(usize::MAX)
    });

    if let Some(folder) = open
        && !nodes.iter().any(|m| m.active)
    {
        nodes.push(MemberNode {
            folder,
            active: true,
            is_default: false,
        });
    }
    nodes
}

#[note("explorer-model-same-location")]
pub fn same_location(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(x), Ok(y)) => x == y,
        _ => a == b,
    }
}

#[note("explorer-model-badge-for")]
pub fn badge_for(owner: Dono) -> &'static str {
    match owner {
        Dono::Gerador => "Rewritten",
        Dono::Usuario => "Yours",
        Dono::PrimeiraVez => "Yours now",
    }
}

#[note("explorer-model-explanation")]
pub fn explanation(dono: Dono) -> &'static str {
    match dono {
        Dono::Gerador => "Rewritten in full on every Generate. Anything you type here is lost.",
        Dono::Usuario => "Yours. Written once, never touched again — not even to fix it.",
        Dono::PrimeiraVez => {
            "Ours the first time, yours from then on. Generate only adds what is missing."
        }
    }
}

#[note("explorer-model-mark-active")]
pub const MARK_ACTIVE: &str = "active";

#[note("explorer-model-mark-default")]
pub const MARK_DEFAULT: &str = "default";

#[note("explorer-model-member-badge")]
pub fn member_badge(m: &MemberNode) -> Option<&'static str> {
    match (m.active, m.is_default) {
        (true, _) => Some(MARK_ACTIVE),
        (false, true) => Some(MARK_DEFAULT),
        (false, false) => None,
    }
}

#[note("explorer-model-silence")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Silence {
    /// No open document in this member, therefore no plan, therefore no owner that
    /// can be asserted. The phrase is what goes to screen in EN.
    Phrase(&'static str),
    /// The member is the active one, but the plan could not be assembled — the same
    /// error that `Generate` would yield.
    CannotPlan(String),
    /// No open document in the entire IDE. Today the shell does not reach here.
    Empty,
}

#[note("explorer-model-no-plan-with-design")]
pub const NO_PLAN_WITH_DESIGN: &str = "Open a form here to see who owns each file.";

#[note("explorer-model-no-design")]
pub const NO_DESIGN: &str = "No form here yet.";

#[note("explorer-model-member-files")]
pub fn member_files<'a>(
    m: &MemberNode,
    plan: &Option<Result<Vec<Arquivo>, crawrad_codegen::Erro>>,
    lines: &'a [Line],
    has_design: bool,
) -> Result<&'a [Line], Silence> {
    if !m.active {
        return Err(Silence::Phrase(if has_design {
            NO_PLAN_WITH_DESIGN
        } else {
            NO_DESIGN
        }));
    }
    match plan {
        Some(Ok(_)) => Ok(lines),
        Some(Err(e)) => Err(Silence::CannotPlan(e.to_string())),
        // Active member without plan does not exist: active IS the one with the open
        // document, and with an open document `generation_plan` returns something.
        None => Err(Silence::Empty),
    }
}

#[note("explorer-model-summary")]
pub fn summary(lines: &[Line]) -> String {
    let count = |e: FileState| lines.iter().filter(|l| l.state == e).count();
    let planned = lines.len() - count(FileState::Stale) - count(FileState::User);
    let mut parts = vec![format!(
        "{planned} file{}",
        if planned == 1 { "" } else { "s" }
    )];
    let pending = count(FileState::Pending);
    if pending > 0 {
        parts.push(format!("{pending} not generated yet"));
    }
    let stale = count(FileState::Stale);
    if stale > 0 {
        parts.push(format!("{stale} stale"));
    }
    parts.join("  ·  ")
}

#[note("explorer-model-folder-label")]
pub fn folder_label(folder: &Path) -> Option<String> {
    if folder.as_os_str().is_empty() {
        None
    } else {
        Some(folder.to_string_lossy().replace('\\', "/") + "/")
    }
}

#[note("explorer-model-name-of")]
pub fn name_of(path: &Path) -> &str {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("?"))
}

// ------------------------------------------------------------------ the legend

#[note("explorer-model-badge-stale")]
pub const BADGE_STALE: &str = "stale";

#[note("explorer-model-legendrow")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegendRow {
    /// The badge or the member mark, exactly as it appears above.
    pub mark: &'static str,
    /// What it means, in the short form that fits a narrow column.
    pub phrase: &'static str,
    /// Draw it in the warning colour — the one case where somebody has to do
    /// something about it.
    pub warn: bool,
}

#[note("explorer-model-legend")]
pub fn legend(lines: &[Line], members: &[MemberNode]) -> Vec<LegendRow> {
    let mut out = Vec::new();
    for (owner, phrase) in [
        (Dono::Gerador, "every Generate replaces it"),
        (Dono::Usuario, "never touched"),
        (Dono::PrimeiraVez, "ours once, yours after"),
    ] {
        if lines.iter().any(|l| l.owner == Some(owner)) {
            out.push(LegendRow {
                mark: badge_for(owner),
                phrase,
                warn: false,
            });
        }
    }
    // `stale` is the only one appearing due to ABSENCE of owner, and is the one that
    // alarms most when it appears — so it also only enters when there is one.
    if lines.iter().any(|l| l.owner.is_none()) {
        out.push(LegendRow {
            mark: BADGE_STALE,
            phrase: "from a form that is gone — Generate deletes it",
            warn: true,
        });
    }
    // Member marks, by the same rule. `active` says where `Generate` writes, which is
    // the question the entire tree exists to answer; `default` only appears when it
    // disagrees with it — see [`MemberNode::is_default`].
    for (mark, phrase) in [
        (MARK_ACTIVE, "Generate and Run write here"),
        (MARK_DEFAULT, "the group's default-members"),
    ] {
        // Through [`member_badge`], and not an `if` over flags: it decides which word
        // goes to screen, and asking it guarantees the legend explains what is up there.
        if members.iter().any(|m| member_badge(m) == Some(mark)) {
            out.push(LegendRow {
                mark,
                phrase,
                warn: false,
            });
        }
    }
    out
}
