//! Completion trigger characters against the REAL rust-analyzer.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crawrad_lsp::complete::Trigger;
use crawrad_lsp::{LspClient, Position};

const MAIN: &str = "#[derive(Debug)]\nstruct P;\n\nfn main() {\n    let a = std::\n    let v = Vec::\n    let n = String::from(\n}\n";

fn labels(client: &LspClient, file: &Path, at: Position, trigger: Trigger) -> Vec<String> {
    let started = Instant::now();
    loop {
        let r = client.request_completion_list(
            file,
            at,
            trigger.context(),
            Duration::from_secs(10),
            &mut || false,
        );
        let items: Vec<String> = r
            .map(|(i, _)| i.into_iter().map(|c| c.label).collect())
            .unwrap_or_default();
        if !items.is_empty() || started.elapsed() > Duration::from_secs(60) {
            return items;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

#[test]
fn the_servers_trigger_characters_answer_what_they_should() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-triggers");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_triggers\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let file = dir.join("src").join("main.rs");
    std::fs::write(&file, MAIN).unwrap();
    let client = LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn");
    client.did_open(&file, "rust", 1, MAIN).unwrap();

    let triggers = client.completion_triggers();
    eprintln!("triggerCharacters: {triggers:?}");
    assert!(
        triggers.iter().any(|t| t == ":") && triggers.iter().any(|t| t == "."),
        "{triggers:?}"
    );

    // `std::` (line 4, after the second ':') -> modules
    let std_items = labels(
        &client,
        &file,
        Position {
            line: 4,
            character: 17,
        },
        Trigger::Character(':'),
    );
    // `Vec::` (line 5) -> associated functions
    let vec_items = labels(
        &client,
        &file,
        Position {
            line: 5,
            character: 17,
        },
        Trigger::Character(':'),
    );
    eprintln!(
        "std:: -> {} items, Vec:: -> {} items",
        std_items.len(),
        vec_items.len()
    );
    assert!(
        std_items.iter().any(|l| l.starts_with("collections")),
        "{std_items:?}"
    );
    assert!(vec_items.iter().any(|l| l == "new"), "{vec_items:?}");

    // A call's `(` (line 6): what does the server answer for the trigger?
    let started = Instant::now();
    let call = client
        .request_completion_list(
            &file,
            Position {
                line: 6,
                character: 25,
            },
            Trigger::Character('(').context(),
            Duration::from_secs(10),
            &mut || false,
        )
        .map(|(i, _)| i.len())
        .unwrap_or(0);
    eprintln!(
        "a call's '(' answers {call} items in {:?}",
        started.elapsed()
    );
    client.shutdown();
}
