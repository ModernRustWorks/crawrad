//! Find All References against the REAL rust-analyzer, through the same
//! worker the IDE uses, and the places turned into the editor's spans.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crawrad_lsp::LspClient;
use crawrad_lsp::references::{Outcome, ReferencesWorker, spans};

/// On disk.
const MAIN: &str = "mod shapes;\n\nfn main() {\n    let a = shapes::area(2);\n    let s = \"\u{e9}\u{1F980}\"; let b = shapes::area(3);\n    let _ = (a, b, s);\n}\n";
/// In the editor, NOT saved: two lines more on top. The server answers
/// about this text, so the spans must be read from it, not from the disk.
const MAIN_EDITED: &str = "// edited\n// not saved\nmod shapes;\n\nfn main() {\n    let a = shapes::area(2);\n    let s = \"\u{e9}\u{1F980}\"; let b = shapes::area(3);\n    let _ = (a, b, s);\n}\n";
const SHAPES: &str = "// \u{1F980} shapes\npub fn area(x: i32) -> i32 {\n    x * x\n}\n";

/// Asks until rust-analyzer has indexed (fewer than `want` while it loads).
fn ask(
    worker: &ReferencesWorker,
    client: &Arc<LspClient>,
    file: &Path,
    text: &str,
    at: (usize, usize),
    want: usize,
) -> Vec<crawrad_lsp::protocol::Location> {
    let started = Instant::now();
    let mut ticket = 0;
    loop {
        ticket += 1;
        worker.request(Arc::clone(client), ticket, file, 2, text, at);
        let answer = loop {
            if let Some(a) = worker.take(ticket) {
                break a;
            }
            assert!(started.elapsed() < Duration::from_secs(120), "no answer");
            std::thread::sleep(Duration::from_millis(50));
        };
        assert_eq!((answer.file.as_path(), answer.version), (file, 2));
        match answer.outcome {
            Outcome::Found(p) if p.len() >= want => return p,
            other if started.elapsed() > Duration::from_secs(90) => panic!("{other:?}"),
            _ => std::thread::sleep(Duration::from_millis(500)),
        }
    }
}

#[test]
fn references_across_files_read_from_the_unsaved_buffer() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-references");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_references\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let main = dir.join("src").join("main.rs");
    let shapes = dir.join("src").join("shapes.rs");
    std::fs::write(&main, MAIN).unwrap();
    std::fs::write(&shapes, SHAPES).unwrap();

    let client = Arc::new(LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn"));
    client.did_open(&main, "rust", 2, MAIN_EDITED).unwrap();
    let worker = ReferencesWorker::new();

    // `area` on the line after the emoji: the declaration and both calls.
    let line = 6;
    let at = (
        line,
        MAIN_EDITED.lines().nth(line).unwrap().find("area").unwrap() + 1,
    );
    let places = ask(&worker, &client, &main, MAIN_EDITED, at, 3);
    eprintln!("{} places: {places:?}", places.len());

    // The editor's reader: the unsaved buffer for main.rs, the disk otherwise.
    let enc = client.position_encoding();
    let mut read = |f: &Path| match f.ends_with("main.rs") {
        true => Some(MAIN_EDITED.to_string()),
        false => std::fs::read_to_string(f).ok(),
    };
    let got = spans(places.clone(), enc, &mut read);
    let seen: Vec<(String, usize, &str)> = got
        .iter()
        .map(|s| {
            let name = s.file.file_name().unwrap().to_string_lossy().to_string();
            (name, s.line, &s.text[s.start..s.end])
        })
        .collect();
    eprintln!("spans: {seen:?}");
    let mut lines: Vec<(String, usize)> = seen.iter().map(|(f, l, _)| (f.clone(), *l)).collect();
    lines.sort();
    assert_eq!(
        lines,
        vec![
            ("main.rs".into(), 5),
            ("main.rs".into(), 6),
            ("shapes.rs".into(), 1),
        ]
    );
    assert!(seen.iter().all(|(_, _, word)| *word == "area"), "{seen:?}");

    // The same places read from the DISK miss: the lines moved by two.
    let mut disk = |f: &Path| std::fs::read_to_string(f).ok();
    let stale = spans(places, enc, &mut disk);
    assert!(
        stale
            .iter()
            .any(|s| s.file.ends_with("main.rs") && s.text.get(s.start..s.end) != Some("area")),
        "the disk must not agree with the unsaved buffer"
    );
    client.shutdown();
}
