//! Pull diagnostics against the REAL rust-analyzer: the server's own
//! `inactive_code` over an item a `#[cfg]` turns off arrives in the same
//! channel as `cargo check`'s, a Hint tagged Unnecessary, so the editor
//! fades it; and the hover over a `#[must_use]` call leads with its warning.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::DiagnosticSeverity;

const MAIN: &str = "#[cfg(feature = \"extra\")]\npub fn only_extra() {}\n\n#[must_use = \"the total is the point\"]\nfn total() -> i32 {\n    1\n}\n\nfn main() {\n    total();\n}\n";

#[test]
fn the_servers_own_diagnostics_arrive_with_cargo_checks() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-pull");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_pull\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\nextra = []\n\n[workspace]\n",
    )
    .unwrap();
    let main = dir.join("src").join("main.rs");
    std::fs::write(&main, MAIN).unwrap();
    let client = Arc::new(LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn"));
    client.did_open(&main, "rust", 1, MAIN).unwrap();
    client.did_save(&main).unwrap();

    // Each list that arrives is the whole of the file: pushed plus pulled.
    let started = Instant::now();
    let (mut inactive, mut must_use) = (None, false);
    while started.elapsed() < Duration::from_secs(120) && (inactive.is_none() || !must_use) {
        for params in client.poll_diagnostics() {
            let code =
                |d: &crawrad_lsp::protocol::Diagnostic, c: &str| d.code.as_deref() == Some(c);
            if let Some(d) = params.diagnostics.iter().find(|d| code(d, "inactive_code")) {
                inactive = Some(d.clone());
            }
            must_use |= params
                .diagnostics
                .iter()
                .any(|d| code(d, "unused_must_use"));
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    let inactive = inactive.expect("rust-analyzer's inactive_code, pulled");
    eprintln!("{inactive:?}");
    assert_eq!(inactive.severity, Some(DiagnosticSeverity::Hint));
    assert_eq!(inactive.tags, vec![1], "Unnecessary: faded");
    assert_eq!(inactive.range.start.line, 0, "from the #[cfg] on");
    assert!(must_use, "cargo check's warning still comes");
    let mark = crawrad_lsp::marks(std::slice::from_ref(&inactive))[0];
    assert!(mark.faded && mark.level() == 0, "faded, no squiggle");
    client.shutdown();
}
