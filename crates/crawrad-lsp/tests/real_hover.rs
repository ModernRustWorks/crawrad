//! Hover against the REAL rust-analyzer, through the worker the IDE uses
//!
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crawrad_lsp::LspClient;
use crawrad_lsp::hover::{HoverWorker, Outcome};

const MAIN: &str = "/// Squares \u{1F980} `x`.\n///\n/// See [`main`].\nfn area(x: i32) -> i32 {\n    x * x\n}\n\nfn main() {\n    let a = area(2);\n    let s = String::new();\n    // a comment\n    let _ = (a, s);\n}\n";

fn caret(line: usize, needle: &str, k: usize) -> (usize, usize) {
    (
        line,
        MAIN.lines().nth(line).unwrap().find(needle).unwrap() + k,
    )
}

/// One answer for one ticket.
fn answer(
    worker: &HoverWorker,
    client: &Arc<LspClient>,
    ticket: u64,
    file: &Path,
    at: (usize, usize),
) -> Outcome {
    worker.request(Arc::clone(client), ticket, file, 1, MAIN, at);
    let started = Instant::now();
    loop {
        if let Some(a) = worker.take(ticket) {
            return a.outcome;
        }
        assert!(started.elapsed() < Duration::from_secs(30), "no answer");
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Asks until the server has indexed (it answers nothing while loading).
fn text(
    worker: &HoverWorker,
    client: &Arc<LspClient>,
    ticket: &mut u64,
    file: &Path,
    at: (usize, usize),
) -> String {
    let started = Instant::now();
    loop {
        *ticket += 1;
        match answer(worker, client, *ticket, file, at) {
            Outcome::Text(t) => return t,
            other if started.elapsed() > Duration::from_secs(60) => panic!("{other:?}"),
            _ => std::thread::sleep(Duration::from_millis(500)),
        }
    }
}

#[test]
fn hover_shows_signature_and_docs_plain() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-hover");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_hover\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let main = dir.join("src").join("main.rs");
    std::fs::write(&main, MAIN).unwrap();
    let client = Arc::new(LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn"));
    client.did_open(&main, "rust", 1, MAIN).unwrap();
    let worker = HoverWorker::new();
    let mut ticket = 0;

    // 1. Our function: its signature and its doc, made plain (no fences,
    //    no backticks, the link's text kept).
    let t = text(&worker, &client, &mut ticket, &main, caret(8, "area", 1));
    eprintln!("area:\n{t}\n---");
    assert!(t.contains("fn area(x: i32) -> i32"), "{t}");
    assert!(t.contains("Squares \u{1F980} x."), "{t}");
    assert!(t.contains("See main."), "{t}");
    assert!(!t.contains("```") && !t.contains('`'), "{t}");

    // 2. std: String::new's signature and the start of its docs.
    let t = text(&worker, &client, &mut ticket, &main, caret(9, "new", 1));
    eprintln!(
        "String::new: {}",
        t.lines().take(4).collect::<Vec<_>>().join(" | ")
    );
    assert!(t.contains("pub const fn new() -> String"), "{t}");

    // 3. A comment and a blank: nothing, so no card.
    ticket += 1;
    assert_eq!(
        answer(&worker, &client, ticket, &main, caret(10, "comment", 2)),
        Outcome::Nothing
    );
    ticket += 1;
    assert_eq!(
        answer(&worker, &client, ticket, &main, (6, 0)),
        Outcome::Nothing
    );
    client.shutdown();
}
