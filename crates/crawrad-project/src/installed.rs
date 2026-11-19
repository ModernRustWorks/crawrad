//! **The IDE's catalogue — builtin PLUS what is installed.**
//!
//! Step §9 of ADR 0033:
//! the IDE reads `manifest.json` files of installed packages and adds them to
//! the catalogue, similar to how classic RAD IDEs load design-time packages on startup.
//!
//! # What was missing, measured on 18/09/2026
//!
//! The third-party component chain was proven end-to-end — from file to Rust code
//! in the user project — and **nothing in the shell read a manifest**.
//! `Manifest`, `to_components` and `register_external` had zero call sites in `crawrad-ide`:
//! the running IDE always used `Registry::builtin()`.
//!
//! In other words: the architecture accepted it, and the IDE did not load it.
//!
//! # Where it looks, and why in TWO places
//!
//! ```text
//! <next to executable>/components/<package>/manifest.json   ← installation
//! <current working directory>/components/<package>/manifest.json   ← this repository
//! ```
//!
//! The first is the layout described by `components/README.md`, which applies to
//! whoever installed the IDE. The second exists because during development the
//! executable lives in `target/debug/` and components live at the repository root —
//! without it, the only way to exercise this path would be installing the IDE on
//! every change.
//!
//! Both are scanned and the result is combined. A package appearing in both is
//! registered once: the second attempt hits the `ClassTaken` refusal already
//! present in the gate.
//!
//! # Refusal is not silent
//!
//! A manifest that fails **does not crash the IDE and does not disappear**: it
//! becomes a line in [`InstalledPackages::refusals`], which the shell displays.
//! A component that disappeared without explanation is the worst thing this gate
//! could do — as `registration.rs` states, and here is where that sentence reaches
//! the user.

use std::path::{Path, PathBuf};

use crawrad_note::note;
use crawrad_package::manifest::{FILE_NAME, Manifest};
use crawrad_registry::Registry;

/// Where a component came from: an installed package, the open project, or user configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentScope {
    /// Next to the IDE (or in this repository during development): no badge.
    Installed,
    /// `<project root>/components/`: travels with the project's git.
    Project,
    /// `<config folder>/components/`: the user's own, in every project.
    Mine,
}

impl ComponentScope {
    /// The small tag the palette shows next to the component (`None` for installed ones).
    pub fn badge(self) -> Option<&'static str> {
        match self {
            Self::Installed => None,
            Self::Project => Some("project"),
            Self::Mine => Some("mine"),
        }
    }
}

#[note("installed-installedpackages")]
pub struct InstalledPackages {
    pub catalog: Registry,
    /// One sentence per refused package. Empty in normal operation.
    pub refusals: Vec<String>,
    /// **Which FOLDER each class came from.**
    ///
    /// This allows the generated project to depend on the component by PATH rather
    /// than through crates.io — and that is not a development shortcut:
    /// ADR 0033
    /// says the IDE **downloads source and compiles**, so an installed component
    /// IS local source. `cargo add --path` is the exact translation of this.
    pub origin: std::collections::BTreeMap<String, PathBuf>,
    /// **THE PACKAGES that loaded**, by manifest `[package] name`.
    ///
    /// Not derivable from [`Self::origin`], and trying to derive it gave a wrong
    /// number in the status bar: `origin` only stores what has a `Cargo.toml`,
    /// because it exists for `cargo add --path`. This repository's
    /// `portadb-crawrad` is only a manifest — counted through there, two packages
    /// became one.
    pub packages: Vec<String>,
    /// **Where each registered component class came from** (`Installed`, `Project`, or `Mine`).
    pub scope: std::collections::BTreeMap<String, ComponentScope>,
}

/// The folders where a component package may reside, each with its scope, in this order:
/// the executable's `components` (Installed) — or, for an exe built in a checkout
/// (`<repo>/target/<profile>/`), the checkout's `components` — the current directory's
/// `components` (Installed),
/// the user's config `components` (Mine), then the project's `components` (Project).
///
/// Returns only folders that exist, so callers don't have to check any; a folder reached twice
/// keeps the FIRST scope it was seen with.
pub fn component_folders(
    exe_dir: Option<&Path>,
    cwd: Option<&Path>,
    config: Option<&Path>,
    project: Option<&Path>,
) -> Vec<(PathBuf, ComponentScope)> {
    let mut candidates = Vec::new();
    if let Some(dir) = exe_dir {
        candidates.push((dir.join("components"), ComponentScope::Installed));
        // The suite that comes with the IDE (PortaDB, the data grid…) is pre-installed:
        // a release ships it next to the exe, and an exe built in a checkout finds it
        // two levels up, whatever folder the IDE was started from.
        if let Some(checkout) = dir
            .parent()
            .filter(|t| t.file_name().is_some_and(|n| n == "target"))
            .and_then(Path::parent)
        {
            candidates.push((checkout.join("components"), ComponentScope::Installed));
        }
    }
    if let Some(dir) = cwd {
        candidates.push((dir.join("components"), ComponentScope::Installed));
    }
    if let Some(dir) = config {
        candidates.push((dir.join("components"), ComponentScope::Mine));
    }
    if let Some(dir) = project {
        candidates.push((dir.join("components"), ComponentScope::Project));
    }
    candidates.retain(|(p, _)| p.is_dir());
    let mut v = Vec::new();
    for (path, scope) in candidates {
        if !v
            .iter()
            .any(|(existing, _): &(PathBuf, _)| *existing == path)
        {
            v.push((path, scope));
        }
    }
    v
}

#[note("installed-current")]
static CURRENT: std::sync::Mutex<Option<&'static InstalledPackages>> = std::sync::Mutex::new(None);
/// The open project's root: its `components/` folder joins the catalogue.
static PROJECT_ROOT: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/// Stores the open project's root and, only when changed, rebuilds the IDE catalogue.
pub fn set_project_root(root: Option<&Path>) -> &'static InstalledPackages {
    let new_root = root.map(Path::to_path_buf);
    let mut lock = PROJECT_ROOT.lock().unwrap_or_else(|e| e.into_inner());
    if *lock != new_root {
        *lock = new_root;
        drop(lock);
        reload_ide_catalog()
    } else {
        drop(lock);
        ide_catalog()
    }
}

/// The open project's root, if any has been set.
pub fn project_root() -> Option<PathBuf> {
    PROJECT_ROOT
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
}

#[note("installed-ide-catalog")]
pub fn ide_catalog() -> &'static InstalledPackages {
    let mut current = CURRENT.lock().unwrap_or_else(|e| e.into_inner());
    current.get_or_insert_with(|| Box::leak(Box::new(catalog())))
}

#[note("installed-reload-ide-catalog")]
pub fn reload_ide_catalog() -> &'static InstalledPackages {
    // The disk is read before the lock is taken: a frame asking for the
    // catalog meanwhile gets the old one, never a wait.
    let fresh: &'static InstalledPackages = Box::leak(Box::new(catalog()));
    *CURRENT.lock().unwrap_or_else(|e| e.into_inner()) = Some(fresh);
    fresh
}

#[note("installed-ide-registry")]
pub fn ide_registry() -> std::rc::Rc<Registry> {
    std::rc::Rc::new(ide_catalog().catalog.clone())
}

#[note("installed-summary")]
pub fn summary() -> String {
    let i = ide_catalog();
    let external = i
        .catalog
        .all()
        .filter(|d| !d.crate_module.is_empty())
        .count();
    // **Packages come from the packages list**, not from `origin`.
    //
    // `origin` only stores what has a `Cargo.toml` — it exists for
    // `cargo add --path`. Counted from there, the two packages in this
    // repository became ONE in the status bar, because `portadb-crawrad`
    // is only a manifest. A wrong count in a sentence that exists to inform
    // is worse than no sentence.
    let base = format!(
        "{} package(s) installed, {external} component(s)",
        i.packages.len()
    );
    if i.refusals.is_empty() {
        return base;
    }
    format!("{base} — REFUSED: {}", i.refusals.join("; "))
}

#[note("installed-refusal-notice")]
pub fn refusal_notice() -> Option<String> {
    let i = ide_catalog();
    if i.refusals.is_empty() {
        return None;
    }
    Some(format!(
        "{} component package(s) refused: {}",
        i.refusals.len(),
        i.refusals.join("; ")
    ))
}

/// Points the catalogue at the project in `root`: the registry the Designer uses from
/// now on, and the status-bar notice when a component package was refused.
pub fn open_project(root: &Path) -> (std::rc::Rc<Registry>, Option<String>) {
    set_project_root(Some(root));
    (ide_registry(), refusal_notice())
}

/// `status` with `notice` appended after ` — `; either one alone when the other is missing.
pub fn with_notice(status: Option<String>, notice: Option<String>) -> Option<String> {
    match (status, notice) {
        (Some(status), Some(notice)) => Some(format!("{status} — {notice}")),
        (status, notice) => status.or(notice),
    }
}

#[note("installed-declared-drawings")]
pub fn declared_drawings() -> crawrad_ir::drawing::DeclaredLooks {
    declared_looks(&ide_catalog().catalog)
}

/// The declared drawings and the inheriting classes' bases of `reg`.
pub fn declared_looks(reg: &Registry) -> crawrad_ir::drawing::DeclaredLooks {
    crawrad_ir::drawing::DeclaredLooks {
        drawings: reg
            .all()
            .filter(|d| !d.desenho.is_empty())
            .map(|d| (d.class.to_string(), d.desenho.clone()))
            .collect(),
        bases: reg
            .all()
            .filter_map(|d| Some((d.class.to_string(), d.inherits?.to_string())))
            .collect(),
        icons: reg
            .all()
            .filter_map(|d| Some((d.class.to_string(), crate::icon_strip::icon_of(d)?)))
            .collect(),
        non_visual: reg
            .all()
            .filter(|d| !d.visual)
            .map(|d| d.class.to_string())
            .collect(),
    }
}

#[note("installed-catalog")]
pub fn catalog() -> InstalledPackages {
    let mut i = InstalledPackages {
        catalog: Registry::builtin(),
        refusals: Vec::new(),
        origin: std::collections::BTreeMap::new(),
        packages: Vec::new(),
        scope: std::collections::BTreeMap::new(),
    };
    let exe = std::env::current_exe().ok();
    let exe_dir = exe.as_deref().and_then(Path::parent);
    let cwd = std::env::current_dir().ok();
    let config = crate::workspace::config_folder();
    let root = project_root();
    for (folder, scope) in
        component_folders(exe_dir, cwd.as_deref(), config.as_deref(), root.as_deref())
    {
        load_folder(&folder, scope, &mut i);
    }
    i
}

#[note("installed-from-folder")]
pub fn from_folder(folder: &Path) -> InstalledPackages {
    from_folders(&[(folder.to_path_buf(), ComponentScope::Installed)])
}

/// Builds an installed package catalogue from an explicit list of folders and scopes.
pub fn from_folders(folders: &[(PathBuf, ComponentScope)]) -> InstalledPackages {
    let mut i = InstalledPackages {
        catalog: Registry::builtin(),
        refusals: Vec::new(),
        origin: std::collections::BTreeMap::new(),
        packages: Vec::new(),
        scope: std::collections::BTreeMap::new(),
    };
    for (folder, scope) in folders {
        load_folder(folder, *scope, &mut i);
    }
    i
}

/// Scans a folder of packages: each subfolder with a `manifest.json` is one.
fn load_folder(folder: &Path, scope: ComponentScope, i: &mut InstalledPackages) {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path().join(FILE_NAME))
        .filter(|p| p.is_file())
        .collect();
    // Stable order: two machines with the same packages load in the same
    // order, and a refusal by `ClassTaken` always accuses the same package.
    paths.sort();
    for path in paths {
        load_manifest(&path, scope, i);
    }
}

fn load_manifest(path: &Path, scope: ComponentScope, i: &mut InstalledPackages) {
    let name = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            i.refusals.push(format!("{name}: {e}"));
            return;
        }
    };
    let manifest: Manifest = match Manifest::parse(&text) {
        Ok(m) => m,
        Err(e) => {
            i.refusals.push(format!("{name}: {e}"));
            return;
        }
    };
    let components = match manifest.to_components() {
        Ok(c) => c,
        Err(e) => {
            i.refusals.push(format!("{name}: {e}"));
            return;
        }
    };
    i.packages.push(manifest.package.name.clone());
    let folder = path.parent().map(|p| p.to_path_buf());
    for c in components {
        let class = c.class.clone();
        match i.catalog.register_external(c) {
            Ok(()) => {
                i.scope.insert(class.clone(), scope);
                if let Some(p) = &folder {
                    // Only what has a `Cargo.toml` becomes a dependency: a package
                    // that is just a manifest (the `engines.toml` of `portadb`)
                    // declares classes and does not provide a crate.
                    if p.join("Cargo.toml").is_file() {
                        i.origin.insert(class, p.clone());
                    }
                }
            }
            Err(e) => i.refusals.push(format!("{name}.{class}: {e}")),
        }
    }
}

#[cfg(test)]
#[path = "test/installed/test.rs"]
mod test;
