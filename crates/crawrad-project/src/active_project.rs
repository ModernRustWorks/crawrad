//! Pure logic for the active workspace project, title formatting, and selector rows.

use std::path::{Path, PathBuf};

use crawrad_note::note;

use crate::workspace::{MANIFEST, package_name_from_manifest, workspace_list};

#[note("active-project-is-workspace")]
pub fn is_workspace(root: &Path) -> bool {
    let Ok(manifest) = std::fs::read_to_string(root.join(MANIFEST)) else {
        return false;
    };
    manifest.contains("[workspace]") && !workspace_list(&manifest, "members").is_empty()
}

#[note("active-project-active-member-folder")]
pub fn active_member_folder(root: &Path) -> Option<PathBuf> {
    if !root.exists() {
        return None;
    }
    if is_workspace(root) {
        let manifest = std::fs::read_to_string(root.join(MANIFEST)).unwrap_or_default();
        let defaults = workspace_list(&manifest, "default-members");
        if let Some(first_def) = defaults.into_iter().next() {
            let folder = root.join(&first_def);
            if folder.join(MANIFEST).is_file() {
                return Some(folder);
            }
        }
        crate::workspace::members(root).into_iter().next()
    } else if root.join(MANIFEST).is_file() {
        Some(root.to_path_buf())
    } else {
        None
    }
}

#[note("active-project-project-name")]
pub fn project_name(folder: &Path) -> String {
    let manifest_path = folder.join(MANIFEST);
    if let Ok(manifest) = std::fs::read_to_string(&manifest_path)
        && let Some(name) = package_name_from_manifest(&manifest)
    {
        return name;
    }
    folder
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("app")
        .to_string()
}

#[note("active-project-active-project-name")]
pub fn active_project_name(root: &Path) -> Option<String> {
    active_member_folder(root).map(|f| project_name(&f))
}

#[note("active-project-window-title")]
pub fn window_title(project: Option<&str>, form_file: Option<&str>) -> String {
    match (project, form_file) {
        (Some(p), Some(f)) => format!("CrawRAD — {p} · {f}"),
        (Some(p), None) => format!("CrawRAD — {p} · no form"),
        (None, Some(f)) => format!("CrawRAD — {f}"),
        (None, None) => "CrawRAD — no form".to_string(),
    }
}

#[note("active-project-status-project-text")]
pub fn status_project_text(project: Option<&str>) -> String {
    match project {
        Some(p) => format!("Project: {p}"),
        None => String::new(),
    }
}

#[cfg(test)]
#[path = "test/active_project/test.rs"]
mod tests;
