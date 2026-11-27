//! **INSTALL A COMPONENT PACKAGE** — `Component ▸ Install Packages…`.
//!
//! Step §9 of ADR 0033,
//! the first of ten requiring UI. Yet it is **headless nonetheless**: the shell
//! dialog asks for the folder; everything that decides lives here, with tests.
//!
//! # What was missing, measured on 18/09/2026
//!
//! The loading gate existed and worked: the IDE scans `components/` on startup
//! and adds whatever it finds. What did not exist was **installing** — the gesture
//! was *"find the installation folder, copy yours inside, and reopen the IDE"*.
//! A third-party suite installed that way simply does not get installed.
//!
//! # Verification comes BEFORE copying, and uses the SAME gate
//!
//! A package that fails is not copied. The difference is not one of diligence:
//! copied, a broken manifest becomes a rejection line that only appears on the
//! NEXT IDE startup, and the folder sits there, half-installed, with nobody to
//! remove it.
//!
//! Checking through the same gate (`Manifest::parse` → `to_components` →
//! `register_external`) is what ensures that *"passed here"* and *"loads on
//! startup"* mean the exact same thing. A second ruler would say yes to a package
//! the first rejects, leaving the user with a component installed that never appears.
//!
//! # The `Cargo.toml` check, and why it belongs to this gate
//!
//! The `crate_module` of a class comes from `[package] name` in the **manifest**,
//! and the user project comes to depend on the component **by path**
//! (`cargo add --path`). If the folder has a `Cargo.toml` with a different package
//! name, generated code writes `gauge_crawrad::Gauge` and Cargo installs
//! `something_else` — the project does not compile, and the error appears in `cargo build`
//! for someone who merely designed a form.
//!
//! It is the kind of error paid for only much later, which is why it is caught here.

use std::path::{Path, PathBuf};

use crawrad_note::note;
use crawrad_package::manifest::{FILE_NAME, Manifest};
use crawrad_registry::Registry;

#[note("installation-verdict")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Accepted. `destination` is where it landed, `classes` what it provides.
    Installed {
        package: String,
        classes: Vec<String>,
        destination: PathBuf,
    },
    /// Did not enter, and the phrase explains why. **Nothing was copied.**
    Refused(String),
}

impl Verdict {
    #[note("installation-verdict-into-parts")]
    pub fn into_parts(self) -> Result<(String, Vec<String>, PathBuf), String> {
        match self {
            Self::Installed {
                package,
                classes,
                destination,
            } => Ok((package, classes, destination)),
            Self::Refused(reason) => Err(reason),
        }
    }

    #[note("installation-verdict-phrase")]
    pub fn phrase(&self) -> String {
        match self {
            Self::Installed {
                package, classes, ..
            } => format!(
                "Installed '{package}' with {} — in the palette now",
                class_list(classes)
            ),
            Self::Refused(reason) => format!("Not installed: {reason}"),
        }
    }
}

fn class_list(classes: &[String]) -> String {
    match classes.len() {
        0 => "no component".to_string(),
        1 => format!("'{}'", classes[0]),
        _ => format!("{} components ({})", classes.len(), classes.join(", ")),
    }
}

#[note("installation-verifiedpackage")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedPackage {
    /// The `[package] name` of the manifest. Gives the installed folder its name.
    pub package: String,
    /// The classes it brings, in file order.
    pub classes: Vec<String>,
}

#[note("installation-verify-package")]
pub fn verify_package(
    origin: &Path,
    already_installed: &Registry,
) -> Result<VerifiedPackage, String> {
    let manifest_at = origin.join(FILE_NAME);
    if !manifest_at.is_file() {
        return Err(format!(
            "'{}' has no {FILE_NAME} — a component package is a folder with one",
            folder_name(origin)
        ));
    }
    let text = std::fs::read_to_string(&manifest_at).map_err(|e| e.to_string())?;
    let manifest = Manifest::parse(&text).map_err(|e| e.to_string())?;
    let components = manifest.to_components().map_err(|e| e.to_string())?;

    // **The registration gate, over a COPY of the catalog.** Checking without
    // actually registering would be a second ruler; actually registering would
    // change this session's catalog for a package that might still be refused
    // on the next line.
    let mut trial = already_installed.clone();
    let mut classes = Vec::new();
    for c in components {
        let class = c.class.clone();
        trial.register_external(c).map_err(|e| e.to_string())?;
        classes.push(class);
    }
    if classes.is_empty() {
        return Err(format!("'{}' declares no component", manifest.package.name));
    }

    // **`crate_module` is what connects both halves**, and whoever has it is whoever
    // generates code. The question comes from the already converted descriptor, not
    // from a second reading of the file: two reads of the same TOML are two chances
    // to disagree.
    let generates_code = trial
        .all()
        .any(|d| classes.iter().any(|c| c == d.class) && !d.crate_module.is_empty());
    if generates_code {
        verify_crate(origin, &manifest)?;
    }

    Ok(VerifiedPackage {
        package: manifest.package.name.clone(),
        classes,
    })
}

#[note("installation-verify-crate")]
fn verify_crate(origin: &Path, manifest: &Manifest) -> Result<(), String> {
    let cargo = origin.join("Cargo.toml");
    if !cargo.is_file() {
        return Err(format!(
            "'{}' generates code and has no Cargo.toml — the generated app \
             would depend on a crate that is not there",
            manifest.package.name
        ));
    }
    let text = std::fs::read_to_string(&cargo).map_err(|e| e.to_string())?;
    let Some(name) = crate_name(&text) else {
        return Err(format!(
            "'{}': its Cargo.toml has no [package] name",
            manifest.package.name
        ));
    };
    if name != manifest.package.name {
        return Err(format!(
            "'{}': the {FILE_NAME} says '{}' and the Cargo.toml says '{name}'. \
             The generated app writes the type with the first and installs the \
             second — it would not compile",
            manifest.package.name, manifest.package.name
        ));
    }
    Ok(())
}

#[note("installation-crate-name")]
fn crate_name(text: &str) -> Option<String> {
    let v: toml::Value = toml::from_str(text).ok()?;
    v.get("package")?.get("name")?.as_str().map(str::to_string)
}

#[note("installation-install-package")]
pub fn install_package(origin: &Path, base: &Path, already_installed: &Registry) -> Verdict {
    let verified = match verify_package(origin, already_installed) {
        Ok(c) => c,
        Err(e) => return Verdict::Refused(e),
    };
    let destination = base.join(&verified.package);
    // **Already there?** Silently overwriting would erase the version the user
    // has in use, and the open IDE would continue with the old one in memory —
    // two truths at the same time, and neither on screen.
    if destination.exists() {
        return Verdict::Refused(format!(
            "'{}' is already installed at {} — remove that folder first",
            verified.package,
            destination.display()
        ));
    }
    if let Err(e) = copy_recursive(origin, &destination) {
        // Partial copy cannot be kept, and it would load a manifest that loads.
        // Deleting is the only honest outcome.
        let _ = std::fs::remove_dir_all(&destination);
        return Verdict::Refused(format!("could not copy: {e}"));
    }
    Verdict::Installed {
        package: verified.package,
        classes: verified.classes,
        destination,
    }
}

#[note("installation-do-not-copy")]
const DO_NOT_COPY: &[&str] = &["target", ".git"];

fn copy_recursive(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if DO_NOT_COPY.iter().any(|x| name == std::ffi::OsStr::new(x)) {
            continue;
        }
        let src = entry.path();
        let dst = to.join(&name);
        if src.is_dir() {
            copy_recursive(&src, &dst)?;
        } else {
            std::fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

fn folder_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| p.display().to_string())
}

#[note("installation-install-now")]
pub fn install_now(origin: &Path) -> Verdict {
    let Some(base) = installation_dir() else {
        return Verdict::Refused("cannot find the CrawRAD folder to install into".into());
    };
    let verdict = install_package(origin, &base, &crate::installed::ide_catalog().catalog);
    if matches!(verdict, Verdict::Installed { .. }) {
        crate::installed::reload_ide_catalog();
    }
    verdict
}

#[note("installation-is-packaged-location")]
pub fn is_packaged_location(exe: &Path) -> bool {
    // The same question the MCP config asks before it writes a command: a path in
    // there is readable by the package and refused to everyone else. One answer,
    // in the crate that holds what this house knows about the operating system.
    crawrad_os::is_inside_store_package(exe)
}

#[note("installation-choose-installation-dir")]
pub fn choose_installation_dir(exe_dir: &Path, config: Option<&Path>) -> Option<PathBuf> {
    if is_packaged_location(exe_dir) {
        config.map(|c| c.join("components"))
    } else {
        Some(exe_dir.join("components"))
    }
}

#[note("installation-installation-dir")]
pub fn installation_dir() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let exe_dir = exe.parent()?;
    let config = crate::workspace::config_folder();
    choose_installation_dir(exe_dir, config.as_deref())
}

#[cfg(test)]
#[path = "test/installation/test.rs"]
mod test;
