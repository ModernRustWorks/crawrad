//! # Package Manager (Crates.io)
//!
//! Allows searching, installing, and removing packages (crates) in the project opened by
//! CrawRAD, integrating directly with `cargo` and the `crates.io` API without
//! locking the IDE UI.
//!
//! Package search uses the public crates.io REST API (`GET /api/v1/crates?q=…`),
//! which returns rich data (downloads, update date, documentation and repository links).
//! Installation and removal continue via `cargo` (ADR 0018: `cargo add`, `cargo remove`,
//! `cargo metadata`).

use crawrad_note::note;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, channel};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Browse,
    Installed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PackageItem {
    pub name: String,
    pub version: String,
    pub description: String,
    pub downloads: Option<u64>,
    pub recent_downloads: Option<u64>,
    pub updated_at: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPackage {
    pub name: String,
    pub req: String,
}

#[derive(serde::Deserialize)]
struct ResponseMetadata {
    packages: Vec<MetadataPackage>,
}

#[derive(serde::Deserialize)]
struct MetadataPackage {
    name: String,
    dependencies: Vec<MetadataDependency>,
}

#[derive(serde::Deserialize)]
struct MetadataDependency {
    name: String,
    req: String,
    kind: Option<String>,
}

/// Extracts the list of packages from `cargo search` text output.
///
/// Kept for existing tests. Primary search now uses [`search_via_api`].
#[cfg(test)]
pub fn parse_cargo_search(saida: &str) -> Vec<PackageItem> {
    let mut itens = Vec::new();
    for linha in saida.lines() {
        let linha = linha.trim();
        if linha.is_empty() || linha.starts_with("...") || linha.starts_with("note:") {
            continue;
        }
        if let Some((declaracao, desc)) = linha.split_once('#') {
            let declaracao = declaracao.trim();
            if let Some((nome, versao)) = declaracao.split_once('=') {
                let nome = nome.trim().to_string();
                let versao = versao.trim().trim_matches('"').to_string();
                let desc = desc.trim().to_string();
                if !nome.is_empty() && !versao.is_empty() {
                    itens.push(PackageItem {
                        name: nome,
                        version: versao,
                        description: desc,
                        downloads: None,
                        recent_downloads: None,
                        updated_at: None,
                        documentation: None,
                        repository: None,
                        homepage: None,
                    });
                }
            }
        }
    }
    itens
}

// ── Crates.io REST API ──────────────────────────────────────────────

const USER_AGENT: &str = "CrawRAD/0.1.0 (https://github.com/ModernRustWorks/crawrad)";

#[note("package-manager-cratesioresponse")]
#[derive(serde::Deserialize)]
struct CratesIoResponse {
    crates: Vec<CrateApiItem>,
}

#[derive(serde::Deserialize)]
struct CrateApiItem {
    name: String,
    #[serde(default)]
    max_version: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    downloads: u64,
    #[serde(default)]
    recent_downloads: Option<u64>,
    #[serde(default)]
    updated_at: Option<String>,
    #[serde(default)]
    documentation: Option<String>,
    #[serde(default)]
    repository: Option<String>,
    #[serde(default)]
    homepage: Option<String>,
}

#[note("package-manager-search-via-api")]
pub fn search_via_api(query: &str, limite: u32) -> Result<Vec<PackageItem>, String> {
    // Simple query string encoding (space → +, special chars → %XX)
    let query_codificada: String = query
        .chars()
        .map(|c| match c {
            ' ' => "+".to_string(),
            c if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' => c.to_string(),
            c => format!("%{:02X}", c as u32),
        })
        .collect();

    let url = format!("https://crates.io/api/v1/crates?q={query_codificada}&per_page={limite}",);

    let agente = ureq::Agent::config_builder()
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .provider(ureq::tls::TlsProvider::Rustls)
                .unversioned_rustls_crypto_provider(Arc::new(rustls_graviola::default_provider()))
                .build(),
        )
        .build()
        .new_agent();

    let body = agente
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| format!("HTTP request failed: {e}"))?
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    let resp: CratesIoResponse =
        serde_json::from_str(&body).map_err(|e| format!("Failed to parse API response: {e}"))?;

    Ok(resp
        .crates
        .into_iter()
        .map(|c| PackageItem {
            name: c.name,
            version: c.max_version,
            description: c.description.unwrap_or_default(),
            downloads: Some(c.downloads),
            recent_downloads: c.recent_downloads,
            updated_at: c.updated_at.as_deref().map(humanize_date),
            documentation: c.documentation,
            repository: c.repository,
            homepage: c.homepage,
        })
        .collect())
}

// ── Formatting helper functions ───────────────────────────────────

#[note("package-manager-format-number")]
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut resultado = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            resultado.push(',');
        }
        resultado.push(c);
    }
    resultado.chars().rev().collect()
}

#[note("package-manager-humanize-date")]
pub fn humanize_date(iso: &str) -> String {
    // Minimal manual parsing — avoids pulling the `chrono` crate for a single function.
    // Expected format: "YYYY-MM-DDT…"
    let partes: Vec<&str> = iso.split('T').collect();
    let Some(data) = partes.first() else {
        return iso.to_string();
    };
    let campos: Vec<&str> = data.split('-').collect();
    if campos.len() < 3 {
        return iso.to_string();
    }
    let Ok(ano) = campos[0].parse::<i32>() else {
        return iso.to_string();
    };
    let Ok(mes) = campos[1].parse::<u32>() else {
        return iso.to_string();
    };

    // Approximate current date (via SystemTime)
    let agora = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Approximation: 1970 + now/31_557_600 (seconds per average year)
    let ano_atual = 1970 + (agora / 31_557_600) as i32;
    let mes_atual = ((agora % 31_557_600) / 2_629_800) as u32 + 1;

    let diff_meses = (ano_atual - ano) * 12 + mes_atual as i32 - mes as i32;

    if diff_meses < 1 {
        "just now".to_string()
    } else if diff_meses == 1 {
        "1 month ago".to_string()
    } else if diff_meses < 12 {
        format!("{diff_meses} months ago")
    } else {
        let anos = diff_meses / 12;
        if anos == 1 {
            "about 1 year ago".to_string()
        } else {
            format!("about {anos} years ago")
        }
    }
}

#[note("package-manager-parse-dependencies-json")]
pub fn parse_dependencies_json(json: &str, pacote_alvo: Option<&str>) -> Vec<InstalledPackage> {
    let Ok(m) = serde_json::from_str::<ResponseMetadata>(json) else {
        return Vec::new();
    };

    let pacote = if let Some(alvo) = pacote_alvo {
        m.packages
            .iter()
            .find(|p| p.name == alvo)
            .or_else(|| m.packages.first())
    } else {
        m.packages.first()
    };

    let Some(p) = pacote else {
        return Vec::new();
    };

    let mut deps = Vec::new();
    for d in &p.dependencies {
        // Only normal runtime dependencies (kind == null)
        if d.kind.is_none() {
            deps.push(InstalledPackage {
                name: d.name.clone(),
                req: d.req.clone(),
            });
        }
    }
    deps.sort_by(|a, b| a.name.cmp(&b.name));
    deps.dedup_by(|a, b| a.name == b.name);
    deps
}

pub struct PackageManagerState {
    pub tab: Tab,
    pub query: String,
    pub results: Vec<PackageItem>,
    pub installed: Vec<InstalledPackage>,
    pub searching: bool,
    pub executing_action: bool,
    pub status: Option<String>,
    pub error: Option<String>,
    /// `true` while the `cargo metadata` thread is out.
    pub loading_installed: bool,
    search_rx: Option<Receiver<Result<Vec<PackageItem>, String>>>,
    action_rx: Option<Receiver<Result<String, String>>>,
    installed_rx: Option<Receiver<(InstalledKey, Vec<InstalledPackage>)>>,
    /// What [`Self::installed`] was read from; `None` before the first read.
    installed_from: Option<InstalledKey>,
    /// What the thread that is out is reading.
    reading: Option<InstalledKey>,
}

#[note("package-manager-installedkey")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledKey {
    root: PathBuf,
    target: Option<String>,
    manifests: Vec<(PathBuf, Option<std::time::SystemTime>)>,
}

impl InstalledKey {
    pub fn of(root: &Path, target: Option<&str>) -> Self {
        let manifests = root
            .ancestors()
            .map(|dir| dir.join("Cargo.toml"))
            .filter_map(|m| {
                let meta = std::fs::metadata(&m).ok()?;
                Some((m, meta.modified().ok()))
            })
            .collect();
        Self {
            root: root.to_path_buf(),
            target: target.map(str::to_string),
            manifests,
        }
    }
}

impl Default for PackageManagerState {
    fn default() -> Self {
        Self {
            tab: Tab::Browse,
            query: String::new(),
            results: Vec::new(),
            installed: Vec::new(),
            searching: false,
            executing_action: false,
            status: None,
            error: None,
            loading_installed: false,
            search_rx: None,
            action_rx: None,
            installed_rx: None,
            installed_from: None,
            reading: None,
        }
    }
}

impl PackageManagerState {
    pub fn open(&mut self, root: Option<&Path>, target_package: Option<&str>) {
        self.error = None;
        self.status = None;
        self.reload_installed(root, target_package);
    }

    #[note("package-manager-packagemanagerstate-reload-installed")]
    pub fn reload_installed(&mut self, root: Option<&Path>, target_package: Option<&str>) {
        let Some(root) = root else {
            self.installed.clear();
            self.installed_from = None;
            return;
        };
        let key = InstalledKey::of(root, target_package);
        if self.installed_from.as_ref() == Some(&key) || self.reading.as_ref() == Some(&key) {
            return;
        }
        let (tx, rx) = channel();
        self.installed_rx = Some(rx);
        self.reading = Some(key.clone());
        self.loading_installed = true;
        let root = root.to_path_buf();
        let target = target_package.map(str::to_string);
        thread::spawn(move || {
            // Same question as [`crate::workspace::group_executables`]:
            // two handwritten lists diverge, which is how the bug started.
            let list = match crawrad_os::process::wait(
                &mut crawrad_os::process::CargoCall::metadata(&root).command(),
            ) {
                Ok(out) if out.success => parse_dependencies_json(&out.stdout, target.as_deref()),
                _ => Vec::new(),
            };
            let _ = tx.send((key, list));
            crawrad_os::tasks::wake();
        });
    }

    #[note("package-manager-packagemanagerstate-installed-count")]
    pub fn installed_count(&self) -> Option<usize> {
        if self.installed_from.is_none() && self.loading_installed {
            None
        } else {
            Some(self.installed.len())
        }
    }

    #[note("package-manager-packagemanagerstate-is-busy")]
    pub fn is_busy(&self) -> bool {
        self.searching || self.executing_action || self.loading_installed
    }

    pub fn start_search(&mut self, query: &str) {
        let q = query.trim().to_string();
        if q.is_empty() {
            return;
        }

        self.searching = true;
        self.error = None;
        self.status = Some(format!("Searching crates.io for '{q}'..."));

        let (tx, rx) = channel();
        self.search_rx = Some(rx);

        thread::spawn(move || {
            let _ = tx.send(search_via_api(&q, 25));
            crawrad_os::tasks::wake();
        });
    }

    pub fn start_addition(
        &mut self,
        package: &str,
        root: Option<&Path>,
        target_package: Option<&str>,
    ) {
        let Some(root) = root else {
            self.error = Some("No active project to add dependency to.".to_string());
            return;
        };

        self.executing_action = true;
        self.error = None;
        self.status = Some(format!("Adding package '{package}' to project..."));

        let pkg = package.to_string();
        let root_path: PathBuf = root.to_path_buf();
        let target_pkg = target_package.map(|s| s.to_string());

        let (tx, rx) = channel();
        self.action_rx = Some(rx);

        thread::spawn(move || {
            let mut cmd = crawrad_os::process::no_console("cargo");
            cmd.arg("add").arg(&pkg);
            if let Some(ref target) = target_pkg {
                cmd.args(["-p", target]);
            }
            cmd.current_dir(&root_path);

            let res = cmd.output();
            let outcome = match res {
                Ok(out) => {
                    if out.status.success() {
                        Ok(format!("Package '{pkg}' added successfully"))
                    } else {
                        let err = String::from_utf8_lossy(&out.stderr).to_string();
                        Err(if err.trim().is_empty() {
                            format!("Failed to add package '{pkg}'")
                        } else {
                            err
                        })
                    }
                }
                Err(e) => Err(format!("Failed to run cargo add: {e}")),
            };

            let _ = tx.send(outcome);
            crawrad_os::tasks::wake();
        });
    }

    pub fn start_removal(
        &mut self,
        package: &str,
        root: Option<&Path>,
        target_package: Option<&str>,
    ) {
        let Some(root) = root else {
            self.error = Some("No active project to remove package from.".to_string());
            return;
        };

        self.executing_action = true;
        self.error = None;
        self.status = Some(format!("Removing package '{package}' from project..."));

        let pkg = package.to_string();
        let root_path: PathBuf = root.to_path_buf();
        let target_pkg = target_package.map(|s| s.to_string());

        let (tx, rx) = channel();
        self.action_rx = Some(rx);

        thread::spawn(move || {
            let mut cmd = crawrad_os::process::no_console("cargo");
            cmd.arg("remove").arg(&pkg);
            if let Some(ref target) = target_pkg {
                cmd.args(["-p", target]);
            }
            cmd.current_dir(&root_path);

            let res = cmd.output();
            let outcome = match res {
                Ok(out) => {
                    if out.status.success() {
                        Ok(format!("Package '{pkg}' removed successfully"))
                    } else {
                        let err = String::from_utf8_lossy(&out.stderr).to_string();
                        Err(if err.trim().is_empty() {
                            format!("Failed to remove package '{pkg}'")
                        } else {
                            err
                        })
                    }
                }
                Err(e) => Err(format!("Failed to run cargo remove: {e}")),
            };

            let _ = tx.send(outcome);
            crawrad_os::tasks::wake();
        });
    }

    pub fn update_channels(&mut self, root: Option<&Path>, target_package: Option<&str>) {
        if let Some(ref rx) = self.installed_rx
            && let Ok((key, list)) = rx.try_recv()
        {
            self.installed_rx = None;
            self.reading = None;
            self.loading_installed = false;
            self.installed = list;
            self.installed_from = Some(key);
        }
        if let Some(ref rx) = self.search_rx
            && let Ok(res) = rx.try_recv()
        {
            self.searching = false;
            self.search_rx = None;
            match res {
                Ok(items) => {
                    self.status = Some(format!("Found {} packages", items.len()));
                    self.results = items;
                }
                Err(e) => {
                    self.error = Some(e);
                    self.status = None;
                }
            }
        }

        if let Some(ref rx) = self.action_rx
            && let Ok(res) = rx.try_recv()
        {
            self.executing_action = false;
            self.action_rx = None;
            match res {
                Ok(msg) => {
                    self.status = Some(msg);
                    self.error = None;
                    self.reload_installed(root, target_package);
                }
                Err(e) => {
                    self.error = Some(e);
                    self.status = None;
                }
            }
        }
    }
}

// ------------------------------------------------- words for the two screens

/// The two tabs, in the order both shells show them.
///
/// It is a list of the MODEL because the egui window and the Makepad dock
/// panel draw the same two, and a tab written twice is how one shell ends up
/// with a tab the other does not have.
impl Tab {
    pub const ALL: &'static [Self] = &[Self::Browse, Self::Installed];

    #[note("package-manager-tab-label")]
    pub fn label(self, installed: Option<usize>) -> String {
        match (self, installed) {
            (Self::Browse, _) => "Browse (crates.io)".to_string(),
            (Self::Installed, Some(n)) => format!("Installed ({n})"),
            (Self::Installed, None) => "Installed".to_string(),
        }
    }
}

#[note("package-manager-searching")]
pub const SEARCHING: &str = "Querying crates.io registry...";

#[note("package-manager-search-hint")]
pub const SEARCH_HINT: &str = "Type a crate name above and press Enter to search.";

#[note("package-manager-no-results")]
pub fn no_results(query: &str) -> String {
    format!("No crates found matching '{query}'")
}

#[note("package-manager-no-project")]
pub const NO_PROJECT: &str = "No project is currently open.";

#[note("package-manager-no-project-hint")]
pub const NO_PROJECT_HINT: &str = "Open a project or workspace to view and manage its packages.";

#[note("package-manager-no-packages")]
pub const NO_PACKAGES: &str = "No packages installed in this project.";

#[note("package-manager-no-packages-hint")]
pub const NO_PACKAGES_HINT: &str =
    "Go to the 'Browse (crates.io)' tab to search and add packages to this project.";

#[note("package-manager-ready")]
pub const READY: &str = "Ready";

#[note("package-manager-already")]
pub const ALREADY: &str = "Installed";

impl PackageManagerState {
    #[note("package-manager-packagemanagerstate-is-installed")]
    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.iter().any(|i| i.name == name)
    }

    #[note("package-manager-packagemanagerstate-installed-req")]
    pub fn installed_req(&self, name: &str) -> Option<&str> {
        self.installed
            .iter()
            .find(|i| i.name == name)
            .map(|i| i.req.as_str())
    }
}

// The tests live WITH the model, and not with the painter: what they measure —
// the API response parser, number formatting, reading `cargo metadata` —
// doesn't have a line of `egui`, and declared on the other side would not reach
// private types of this file.
//
// `#[cfg(test)]` BEFORE `#[path]`, and the order is not a preference: the guard
// `shell_still_mutates_document_outside_catalogue`, over in `crawrad-actions`,
// looks for `#[path]` INSIDE the test region of the file. With `#[path]` on top
// it sees a bare `mod tests;` and accuses the entire test file of being production.
#[cfg(test)]
#[path = "test/package_manager/test.rs"]
mod tests;
