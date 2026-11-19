//! The runtime stays permissive: a crate licensed `MIT OR Apache-2.0` goes inside the
//! user's app, so it must never depend on one of our GPL-3.0 IDE crates — that would
//! carry the GPL into every app built with CrawRAD (see `ADDITIONAL-PERMISSIONS.md`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Every workspace crate: its package name, licence, and the names of its path
/// dependencies (normal ones only — dev and build dependencies do not ship).
pub fn workspace_crates(root: &Path) -> BTreeMap<String, (String, Vec<String>)> {
    let mut out = BTreeMap::new();
    for dir in ["crates", "components"] {
        let Ok(entries) = std::fs::read_dir(root.join(dir)) else {
            continue;
        };
        for entry in entries.flatten() {
            let manifest: PathBuf = entry.path().join("Cargo.toml");
            let Ok(text) = std::fs::read_to_string(&manifest) else {
                continue;
            };
            let Ok(doc) = text.parse::<toml::Table>() else {
                continue;
            };
            let Some(package) = doc.get("package").and_then(|p| p.as_table()) else {
                continue;
            };
            let name = package
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or_default()
                .to_string();
            let license = match package.get("license") {
                Some(toml::Value::String(s)) => s.clone(),
                _ => "GPL-3.0-only".to_string(),
            };
            let deps = doc
                .get("dependencies")
                .and_then(|d| d.as_table())
                .map(|t| {
                    t.iter()
                        .filter(|(_, v)| v.get("path").is_some() || v.get("workspace").is_some())
                        .map(|(k, v)| {
                            v.get("package")
                                .and_then(|p| p.as_str())
                                .unwrap_or(k)
                                .to_string()
                        })
                        .collect()
                })
                .unwrap_or_default();
            out.insert(name, (license, deps));
        }
    }
    out
}

/// The violations: `(runtime crate, GPL crate it depends on)`.
pub fn violations(crates: &BTreeMap<String, (String, Vec<String>)>) -> Vec<(String, String)> {
    let mut bad = Vec::new();
    for (name, (license, deps)) in crates {
        if license.contains("GPL") {
            continue;
        }
        for dep in deps {
            if let Some((dep_license, _)) = crates.get(dep)
                && dep_license.contains("GPL")
            {
                bad.push((name.clone(), dep.clone()));
            }
        }
    }
    bad
}

#[cfg(test)]
#[path = "test/runtime_license_gate/test.rs"]
mod tests;
