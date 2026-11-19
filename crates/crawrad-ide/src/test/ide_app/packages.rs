//! The gate of the PACKAGE MANAGER panel of the Makepad shell.
//!
//! Everything that talks to the network, to `cargo` or to the manifest is
//! measured next to the model (`crawrad-project/src/test/package_manager/test.rs`), and
//! those tests belong to both shells because both compile that file. What is
//! measured here is what this panel adds: which row a state turns into, which
//! template it asks for, and the sentences it says when there is nothing to
//! list.

use super::packages::{self, Row};
use crawrad_project::package_manager::{InstalledPackage, PackageItem, PackageManagerState, Tab};

/// The Makepad shell — the whole file, as text.
use super::casca::CASCA;

/// The THEME, where the row templates live.
const TEMA: &str = super::tests::TEMA_COMPLETO;

fn compacto() -> String {
    CASCA.chars().filter(|c| !c.is_whitespace()).collect()
}

fn pacote(nome: &str) -> PackageItem {
    PackageItem {
        name: nome.to_string(),
        version: "1.0.0".to_string(),
        description: "a crate".to_string(),
        downloads: Some(1_234_567),
        recent_downloads: None,
        updated_at: Some("8 months ago".to_string()),
        documentation: None,
        repository: None,
        homepage: None,
    }
}

/// Without a project, the panel explains what happened **and what to do about it**.
///
/// Two rows, and the second is not decorative: `No project is currently open.`
/// alone leaves the viewer without the next step, which is precisely what
/// separates an alert from a dead end.
#[test]
fn with_no_project_the_panel_says_what_to_do() {
    let estado = PackageManagerState::default();
    let linhas = packages::rows(&estado, false);
    assert_eq!(linhas.len(), 2);
    assert!(matches!(linhas[0], Row::Note(_)));
    assert!(matches!(linhas[1], Row::Hint(_)));
}

/// With a project and no search, the panel guides the user — it does not remain blank.
#[test]
fn an_empty_search_teaches_the_gesture() {
    let estado = PackageManagerState::default();
    let linhas = packages::rows(&estado, true);
    assert_eq!(
        linhas,
        vec![Row::Note(
            crawrad_project::package_manager::SEARCH_HINT.to_string()
        )]
    );
}

/// **A crate already present in the project does not offer `Add`.**
///
/// A button that accepts interaction and changes nothing is the dead control this
/// shell already eliminated from the status bar, and it will not return via Package Manager.
/// The template is different, and the state chooses it — not the painter.
#[test]
fn a_crate_already_installed_offers_no_add_button() {
    let mut estado = PackageManagerState::default();
    estado.results = vec![pacote("serde"), pacote("rfd")];
    estado.installed = vec![InstalledPackage {
        name: "serde".to_string(),
        req: "^1.0".to_string(),
    }];
    let linhas = packages::rows(&estado, true);
    assert_eq!(linhas[0].template(), "FoundInstalled");
    assert_eq!(linhas[1].template(), "Found");
}

#[test]
fn a_crate_with_newer_version_offers_update_button() {
    let mut estado = PackageManagerState::default();
    estado.results = vec![PackageItem {
        name: "eframe".to_string(),
        version: "0.36.2".to_string(),
        description: "egui framework".to_string(),
        downloads: Some(19_000_000),
        recent_downloads: None,
        updated_at: Some("just now".to_string()),
        documentation: None,
        repository: None,
        homepage: None,
    }];
    estado.installed = vec![InstalledPackage {
        name: "eframe".to_string(),
        req: "^0.36".to_string(),
    }];
    let linhas = packages::rows(&estado, true);
    assert_eq!(linhas[0].template(), "FoundUpdatable");
}

/// The `Installed` tab displays what the manifest declares, along with the requirement.
#[test]
fn the_installed_tab_shows_what_the_manifest_declares() {
    let mut estado = PackageManagerState::default();
    estado.tab = Tab::Installed;
    estado.installed = vec![InstalledPackage {
        name: "eframe".to_string(),
        req: "^0.36".to_string(),
    }];
    assert_eq!(
        packages::rows(&estado, true),
        vec![Row::Installed {
            name: "eframe".to_string(),
            req: "^0.36".to_string(),
        }]
    );
}

/// **API failure to respond does not become an EMPTY ROW.**
///
/// The right column is a stack of fixed-height labels: a blank line
/// in the middle opens an unexplained gap, and a card without any statistics
/// would have its button pushed three rows down.
///
/// This test replaced `the_stats_line_never_ends_in_a_dangling_separator`,
/// which guarded the same concern in the previous design — a single line with ` · `
/// between elements. The intent remains; what changed was the layout when the owner
/// requested the two columns back.
#[test]
fn api_unresponsive_does_not_become_empty_row() {
    let mut p = pacote("rfd");
    p.recent_downloads = Some(8_910);
    assert_eq!(
        packages::package_stats(&p),
        vec![
            "All-time: 1,234,567".to_string(),
            "Recent: 8,910".to_string(),
            "Updated: 8 months ago".to_string(),
        ]
    );

    p.recent_downloads = None;
    assert_eq!(
        packages::package_stats(&p),
        vec![
            "All-time: 1,234,567".to_string(),
            "Updated: 8 months ago".to_string(),
        ],
        "absent `Recent` must DISAPPEAR, not turn into a blank line"
    );

    p.updated_at = None;
    p.downloads = None;
    assert!(
        packages::package_stats(&p).is_empty(),
        "with no facts the column disappears entirely, and the button moves up"
    );
}

/// **The column never requests more boxes than the DSL provides.**
///
/// `live_design!` is static and there is no public API to inject DSL at runtime
/// (ADR 0027): a fourth statistic would not appear in a fourth
/// row — it would vanish **silently**, which is the kind of defect this codebase
/// strives hardest to avoid.
#[test]
fn column_never_requests_more_boxes_than_dsl_has() {
    let mut p = pacote("rfd");
    p.recent_downloads = Some(8_910);
    assert!(packages::package_stats(&p).len() <= packages::MAX_STAT_LINES);
    // And the DSL declares EXACTLY that quantity of boxes.
    let c: String = TEMA.chars().filter(|c| !c.is_whitespace()).collect();
    for slot in 1..=packages::MAX_STAT_LINES {
        assert!(
            c.contains(&format!("stat_{slot}=<CrawradPacoteStat>")),
            "box `stat_{slot}` disappeared from DSL, and its statistic has \
             nowhere to be written"
        );
    }
    assert!(
        !c.contains(&format!("stat_{}=<", packages::MAX_STAT_LINES + 1)),
        "the DSL gained one more box than the model can fill, and it \
         stays empty on screen forever"
    );
}

/// Error takes precedence over state, and state takes precedence over `Ready`.
///
/// A panel that falls silent after a failed `cargo add` loses the only
/// message explaining why nothing happened.
#[test]
fn the_status_line_puts_the_failure_first() {
    let mut estado = PackageManagerState::default();
    assert_eq!(
        packages::status(&estado),
        crawrad_project::package_manager::READY
    );
    estado.status = Some("Adding package 'rfd'...".to_string());
    assert_eq!(packages::status(&estado), "Adding package 'rfd'...");
    estado.error = Some("HTTP request failed".to_string());
    assert_eq!(packages::status(&estado), "HTTP request failed");
}

/// **Every template requested by a row is declared in the DSL.**
///
/// Same rule as the neighbouring Explorer, and same trap: a mismatched name
/// returns an empty widget and the row renders NOTHING, leaving one log line.
#[test]
fn every_package_template_is_declared_in_the_dsl() {
    let c = compacto();
    for linha in packages::sample_rows() {
        let nome = linha.template();
        assert!(
            c.contains(&format!("{nome}=<")),
            "template `{nome}`, requested by a Package Manager row, is not \
             declared in the panel DSL"
        );
    }
}

/// The panel is populated by code, and both tab labels come from the MODEL.
#[test]
fn the_panel_is_fed_from_rust_and_the_tabs_come_from_the_model() {
    let c = compacto();
    assert!(
        c.contains("packages_list=<PortalList>"),
        "Package Manager is no longer a code-fed list"
    );
    assert!(
        c.contains("self.linhas_dos_pacotes()"),
        "Package Manager rows stopped being built from state"
    );
    // The labels are NOT hardcoded in the DSL: both tabs start blank and the
    // text comes from `Tab::label`, which is the list BOTH shells read.
    //
    // The check looks for the DECLARATION, not a stray word in the file: the
    // first version of this test looked for `Installed (0)` anywhere
    // and failed on a COMMENT explaining a defect citing that text.
    // Comments do not define any label.
    for nome in ["tab_browse", "tab_installed"] {
        assert!(
            c.contains(&format!("{nome}=<CrawradInspectorTab>{{text:\"\"}}")),
            "tab `{nome}` of Package Manager either disappeared from DSL or was \
             born with a label written on it — creating the tab list in two \
             places, with the second born outdated"
        );
    }
    assert!(
        c.contains("aba.label(instalados)"),
        "tab labels stopped coming from `Tab::label`: the two shells \
         would stop reading the same list"
    );
    // And the count in `Installed (N)` is from state, not a hardcoded number.
    assert_eq!(Tab::Installed.label(Some(3)), "Installed (3)");
    // Before the first read the tab says no number, never a false zero.
    assert_eq!(Tab::Installed.label(None), "Installed");
    assert_eq!(Tab::ALL.len(), 2);
}
