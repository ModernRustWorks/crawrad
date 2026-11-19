//! The PACKAGE MANAGER panel of the Makepad shell — the rows it draws.
//!
//! # It is a PANEL here, and a window there
//!
//! In the egui shell the package store is a separate system window
//! (`package_manager.rs`, a `ViewportBuilder` of 720x520). Here it is a
//! dockable tab, and that is the owner's decision, written down as Emenda 8 of
//! [ADR 0014]. Letting a panel leave the IDE window is and is not
//! part of this round.
//!
//! The consequence shows up in every row below: the column is around 340 px
//! wide, not 720.
//!
//! **The first answer to that was to flatten the card into one column, and the
//! owner refused it on screen:** *"o Manager regrediu, tinha duas colunas"*.
//! He is right, and the reason is not nostalgia — flattened, the button sits
//! next to the NAME, so it moves with every name length, and a list of twenty
//! results has the button in twenty places.
//!
//! So the card is two columns again, with the width paid for by what did NOT
//! come back: the three links (`Documentation`, `Repository`, `Homepage`) are
//! still out, because opening a URL is not wired in this shell yet, and the
//! right column is 150 rather than the egui window's 170.
//!
//! # What is NOT here
//!
//! Searching crates.io, running `cargo add` / `cargo remove`, reading
//! `cargo metadata`, and every sentence the two shells say — all of it lives
//! in [`model`], which both binaries include by
//! `#[path]`. This module only decides what a list row IS.
//!

// The HEADLESS package manager, through the same module path in both binaries —
// see the note in the Explorer panel beside it.
pub use crate::package_manager as model;
use crawrad_note::note;

#[note("package-rows-row")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    /// A sentence in place of a list — why it is empty, or what to do.
    Note(String),
    /// The second, smaller line of a [`Row::Note`].
    Hint(String),
    /// Something went wrong, said in the error colour.
    Error(String),
    /// A crate from the search, and whether the project already has it.
    Found {
        name: String,
        version: String,
        description: String,
        /// The right-hand column, top to bottom — see `estatisticas_de`.
        stats: Vec<String>,
        installed: bool,
        can_update: bool,
    },
    /// A dependency the manifest declares.
    Installed { name: String, req: String },
}

impl Row {
    #[note("package-rows-row-template")]
    pub fn template(&self) -> &'static str {
        match self {
            Self::Note(_) => "Note",
            Self::Hint(_) => "Hint",
            Self::Error(_) => "Error",
            Self::Found {
                installed: true,
                can_update: true,
                ..
            } => "FoundUpdatable",
            Self::Found {
                installed: true,
                can_update: false,
                ..
            } => "FoundInstalled",
            Self::Found {
                installed: false, ..
            } => "Found",
            Self::Installed { .. } => "Dependency",
        }
    }
}

#[note("package-rows-max-stat-lines")]
pub const MAX_STAT_LINES: usize = 3;

#[note("package-rows-package-stats")]
pub fn package_stats(item: &model::PackageItem) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(d) = item.downloads {
        lines.push(format!("All-time: {}", model::format_number(d)));
    }
    if let Some(d) = item.recent_downloads {
        lines.push(format!("Recent: {}", model::format_number(d)));
    }
    if let Some(u) = &item.updated_at {
        lines.push(format!("Updated: {u}"));
    }
    lines.truncate(MAX_STAT_LINES);
    lines
}

#[note("package-rows-rows")]
pub fn rows(state: &model::PackageManagerState, has_project: bool) -> Vec<Row> {
    if !has_project {
        return vec![
            Row::Note(model::NO_PROJECT.to_string()),
            Row::Hint(model::NO_PROJECT_HINT.to_string()),
        ];
    }
    match state.tab {
        model::Tab::Browse => browse(state),
        model::Tab::Installed => installed(state),
    }
}

fn browse(state: &model::PackageManagerState) -> Vec<Row> {
    if let Some(e) = &state.error {
        return vec![Row::Error(e.clone())];
    }
    if state.searching {
        return vec![Row::Note(model::SEARCHING.to_string())];
    }
    if state.results.is_empty() {
        return vec![Row::Note(if state.query.trim().is_empty() {
            model::SEARCH_HINT.to_string()
        } else {
            model::no_results(&state.query)
        })];
    }
    state
        .results
        .iter()
        .map(|i| {
            let req_opt = state.installed_req(&i.name);
            let installed = req_opt.is_some();
            let can_update = if let Some(req) = req_opt {
                let parse_triplet = |s: &str| -> Option<(u64, u64, u64)> {
                    let clean = s.trim_start_matches(['^', '~', '=', 'v', ' ']);
                    let parts: Vec<&str> = clean.split('.').collect();
                    let major = parts.first()?.parse().ok()?;
                    let minor = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);
                    let patch = parts
                        .get(2)
                        .and_then(|p| p.split('-').next())
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(0);
                    Some((major, minor, patch))
                };
                match (parse_triplet(&i.version), parse_triplet(req)) {
                    (Some(v_ver), Some(v_req)) => v_ver > v_req,
                    _ => {
                        let clean_req = req.trim_start_matches(['^', '~', '=', 'v', ' ']);
                        let clean_ver = i.version.trim_start_matches(['v', ' ']);
                        clean_req != clean_ver
                    }
                }
            } else {
                false
            };
            Row::Found {
                installed,
                can_update,
                stats: package_stats(i),
                name: i.name.clone(),
                version: format!("v{}", i.version),
                description: i.description.clone(),
            }
        })
        .collect()
}

fn installed(state: &model::PackageManagerState) -> Vec<Row> {
    if let Some(e) = &state.error {
        return vec![Row::Error(e.clone())];
    }
    if state.installed.is_empty() {
        return vec![
            Row::Note(model::NO_PACKAGES.to_string()),
            Row::Hint(model::NO_PACKAGES_HINT.to_string()),
        ];
    }
    state
        .installed
        .iter()
        .map(|i| Row::Installed {
            name: i.name.clone(),
            req: i.req.clone(),
        })
        .collect()
}

#[note("package-rows-status")]
pub fn status(state: &model::PackageManagerState) -> &str {
    if let Some(e) = &state.error {
        return e;
    }
    match &state.status {
        Some(s) => s,
        None => model::READY,
    }
}

/// One row of every kind, so the DSL gate can instantiate every template.
///
/// Same reason as the Explorer's: the gate covers exactly what the probe
/// instantiates. Public (and not `cfg(test)`) because the gate lives in the
/// shell's tests, another crate.
///
/// # Examples
///
/// ```rust
/// use crawrad_project::package_rows::sample_rows;
///
/// let rows = sample_rows();
/// assert!(!rows.is_empty());
/// ```
#[doc(hidden)]
pub fn sample_rows() -> Vec<Row> {
    let found = |installed, can_update| Row::Found {
        name: "rfd".to_string(),
        version: "v0.17.2".to_string(),
        description: "Rusty File Dialog".to_string(),
        stats: vec![
            "All-time: 29,449,600".to_string(),
            "Recent: 1,208".to_string(),
        ],
        installed,
        can_update,
    };
    vec![
        found(false, false),
        Row::Installed {
            name: "serde".to_string(),
            req: "^1.0".to_string(),
        },
        found(true, false),
        found(true, true),
        Row::Note(model::NO_PROJECT.to_string()),
        Row::Hint(model::NO_PROJECT_HINT.to_string()),
        Row::Error("HTTP request failed".to_string()),
    ]
}
