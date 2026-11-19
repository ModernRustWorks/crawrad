//! `$/cancelRequest` against the REAL rust-analyzer: cancelling many
//! requests leaves the server answering the next ones correctly.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use crawrad_lsp::{LspClient, LspError};

const MAIN: &str = "fn helper() -> u32 {\n    7\n}\n\nfn main() {\n    let s = String::new();\n    let n = helper();\n}\n";

#[test]
fn cancelled_requests_leave_the_server_answering() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-cancel");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_cancel\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let file = dir.join("src").join("main.rs");
    std::fs::write(&file, MAIN).unwrap();
    let client = LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn");
    client.did_open(&file, "rust", 1, MAIN).unwrap();
    let params = serde_json::json!({
        "textDocument": { "uri": crawrad_lsp::path_to_uri(&file) },
        "position": { "line": 5, "character": 14 }
    });

    // Twenty completions, each cancelled as soon as it is on the wire.
    let mut cancelled = 0;
    for _ in 0..20 {
        let r = client.send_request_until(
            "textDocument/completion",
            params.clone(),
            Duration::from_secs(10),
            &mut || true,
        );
        if matches!(r, Err(LspError::Cancelled)) {
            cancelled += 1;
        }
    }
    assert!(cancelled > 0, "nothing was cancelled");

    // The server still answers, and answers right: Go to Definition of the
    // call to `helper` (line 6) lands on `fn helper` (line 0).
    let started = Instant::now();
    let def = loop {
        if let Ok(locs) = client.request_definition(&file, 6, 13, Duration::from_secs(5))
            && let Some(l) = locs.first()
        {
            break l.range.start.line;
        }
        assert!(
            started.elapsed() < Duration::from_secs(90),
            "the server stopped answering"
        );
        std::thread::sleep(Duration::from_millis(500));
    };
    eprintln!("{cancelled}/20 cancelled; definition after them: line {def}");
    assert_eq!(def, 0);
    client.shutdown();
}
