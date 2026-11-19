//! **Installing and uninstalling an extension while the IDE runs**:
//! a folder with a manifest is copied into the user extensions store at
//! `<user_dir>/<id>@<version>/` (if not already present), registered in the project's
//! `.crawrad/extensions.json`, and started at once. Uninstalling stops its
//! process, takes back everything it contributed — actions, cards, line
//! marks, views — and removes it from the project list, preserving the user
//! store folder for other projects (while deleting legacy project-local folders).
//! No restart, and no trace.

use std::path::{Path, PathBuf};
use std::time::Duration;

use crawrad_extension_api::error::ExtensionError;
use crawrad_extension_api::manifest::ExtensionManifest;
use serde::{Deserialize, Serialize};

use crate::host::ShellExtensionHost;
use crawrad_note::note;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionRef {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectExtensions {
    #[serde(default)]
    pub extensions: Vec<ExtensionRef>,
}

#[note("install-project-extensions-file")]
pub fn project_extensions_file(root: &Path) -> PathBuf {
    root.join(".crawrad").join("extensions.json")
}

#[note("install-read-project-extensions")]
pub fn read_project_extensions(root: &Path) -> Result<ProjectExtensions, ExtensionError> {
    let file = project_extensions_file(root);
    if !file.is_file() {
        return Ok(ProjectExtensions::default());
    }
    let text = std::fs::read_to_string(&file).map_err(|e| ExtensionError::Io(e.to_string()))?;
    serde_json::from_str(&text).map_err(|e| ExtensionError::Io(e.to_string()))
}

#[note("install-write-project-extensions")]
pub fn write_project_extensions(
    root: &Path,
    mut exts: ProjectExtensions,
) -> Result<(), ExtensionError> {
    exts.extensions.sort_by(|a, b| a.id.cmp(&b.id));
    let parent = root.join(".crawrad");
    std::fs::create_dir_all(&parent).map_err(|e| ExtensionError::Io(e.to_string()))?;
    let text =
        serde_json::to_string_pretty(&exts).map_err(|e| ExtensionError::Io(e.to_string()))?;
    std::fs::write(project_extensions_file(root), format!("{text}\n"))
        .map_err(|e| ExtensionError::Io(e.to_string()))
}

#[note("install-user-extensions-dir")]
pub fn user_extensions_dir() -> PathBuf {
    if let Some(app_data) = std::env::var_os("APPDATA") {
        PathBuf::from(app_data).join("CrawRAD").join("extensions")
    } else if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))
    {
        PathBuf::from(home).join(".crawrad").join("extensions")
    } else {
        PathBuf::from("extensions")
    }
}

#[note("install-extensions-dir")]
pub fn extensions_dir(root: &Path) -> PathBuf {
    root.join(".crawrad").join("extensions")
}

#[note("install-read-manifest")]
pub fn read_manifest(dir: &Path) -> Result<ExtensionManifest, ExtensionError> {
    let found = ["manifest.json", "extension.json"]
        .iter()
        .map(|n| dir.join(n))
        .find(|p| p.is_file())
        .ok_or_else(|| {
            let shown = dir.display();
            ExtensionError::InvalidManifest(format!("no manifest.json in '{shown}'"))
        })?;
    let text = std::fs::read_to_string(&found).map_err(|e| ExtensionError::Io(e.to_string()))?;
    ExtensionManifest::from_json_str(&text)
}

#[note("install-legacy-installed-dir")]
pub fn legacy_installed_dir(root: &Path, id: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(extensions_dir(root)).ok()?;
    entries
        .flatten()
        .map(|e| e.path())
        .find(|p| p.is_dir() && read_manifest(p).is_ok_and(|m| m.id == id))
}

#[note("install-installed-dir")]
pub fn installed_dir(root: &Path, user_dir: &Path, id: &str) -> Option<PathBuf> {
    if let Ok(project_exts) = read_project_extensions(root)
        && let Some(ext_ref) = project_exts.extensions.iter().find(|e| e.id == id)
    {
        let candidate = user_dir.join(format!("{}@{}", ext_ref.id, ext_ref.version));
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    legacy_installed_dir(root, id)
}

#[note("install-copy-dir")]
fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)?.flatten() {
        let (src, dst) = (entry.path(), to.join(entry.file_name()));
        if src.is_dir() {
            copy_dir(&src, &dst)?;
        } else {
            std::fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

#[note("install-remove-dir")]
fn remove_dir(dir: &Path) -> std::io::Result<()> {
    let mut last = Ok(());
    for _ in 0..20 {
        last = std::fs::remove_dir_all(dir);
        if last.is_ok() || !dir.exists() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    last
}

impl ShellExtensionHost {
    #[note("install-shellextensionhost-install")]
    pub fn install(
        &mut self,
        root: &Path,
        source: &Path,
    ) -> Result<ExtensionManifest, ExtensionError> {
        self.install_into(root, &user_extensions_dir(), source)
    }

    /// [`Self::install`] into a given user store (tests pass a scratch folder).
    pub fn install_into(
        &mut self,
        root: &Path,
        user_dir: &Path,
        source: &Path,
    ) -> Result<ExtensionManifest, ExtensionError> {
        let manifest = read_manifest(source)?;
        let mut project_exts = read_project_extensions(root)?;
        if self.manifests.iter().any(|m| m.id == manifest.id)
            || project_exts.extensions.iter().any(|e| e.id == manifest.id)
            || legacy_installed_dir(root, &manifest.id).is_some()
        {
            return Err(ExtensionError::Duplicate(manifest.id));
        }

        let target = user_dir.join(format!("{}@{}", manifest.id, manifest.version));
        if !target.exists() {
            copy_dir(source, &target).map_err(|e| ExtensionError::Io(e.to_string()))?;
        }

        project_exts.extensions.retain(|e| e.id != manifest.id);
        project_exts.extensions.push(ExtensionRef {
            id: manifest.id.clone(),
            version: manifest.version.clone(),
        });
        write_project_extensions(root, project_exts)?;

        if let Ok(mut ph) = self.process_host.lock() {
            ph.workspace_root.get_or_insert_with(|| root.to_path_buf());
            ph.user_dir.get_or_insert_with(|| user_dir.to_path_buf());
        }
        self.register_manifest(manifest.clone())?;
        Ok(manifest)
    }

    #[note("install-shellextensionhost-uninstall")]
    pub fn uninstall(&mut self, root: &Path, id: &str) -> Result<(), ExtensionError> {
        let before_manifests = self.manifests.len();
        self.manifests.retain(|m| m.id != id);
        let removed_from_manifests = self.manifests.len() < before_manifests;

        let mut project_exts = read_project_extensions(root)?;
        let before_project = project_exts.extensions.len();
        project_exts.extensions.retain(|e| e.id != id);
        let removed_from_project = project_exts.extensions.len() < before_project;
        if removed_from_project {
            write_project_extensions(root, project_exts)?;
        }

        let legacy = legacy_installed_dir(root, id);
        if !removed_from_manifests && !removed_from_project && legacy.is_none() {
            return Err(ExtensionError::NotFound(id.to_string()));
        }

        if let Ok(mut cards) = self.card_manager.lock() {
            cards.forget(id);
        }
        if let Ok(mut ph) = self.process_host.lock() {
            ph.unregister(id);
        }
        if let Some(dir) = legacy {
            remove_dir(&dir).map_err(|e| ExtensionError::Io(e.to_string()))?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/install/test.rs"]
mod tests;
