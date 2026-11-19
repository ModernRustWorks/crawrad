//! The column unit negotiated with the REAL rust-analyzer.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.
//!
//! Every position here sits AFTER `é` and `🦀` on its line, where UTF-8,
//! UTF-16 and char columns all differ: a client and a server that disagree
//! on the unit point at different characters, and the answers below miss.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crawrad_lsp::complete::Trigger;
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::{LspClient, Position};

const MAIN: &str =
    "fn main() {\n    let s = \"é🦀\"; let n = s.len(); let m = n;\n    let t = \"🦀\"; s.\n}\n";

/// The position of the `nth` `needle` on `line` of `text`, in the client's
/// negotiated unit.
fn at(client: &LspClient, text: &str, line: usize, needle: &str, nth: usize) -> Position {
    let code = text.lines().nth(line).unwrap();
    let byte = code.match_indices(needle).nth(nth).unwrap().0;
    Position {
        line: line as u32,
        character: client.position_encoding().column(code, byte),
    }
}

/// Where `n` in `let m = n` is defined, retried while the server indexes.
fn definition(client: &LspClient, file: &Path, from: Position) -> Option<Position> {
    let started = Instant::now();
    loop {
        let r = client.request_definition(file, from.line, from.character, Duration::from_secs(10));
        if let Some(l) = r.ok().and_then(|v| v.into_iter().next()) {
            return Some(l.range.start);
        }
        if started.elapsed() > Duration::from_secs(60) {
            return None;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn labels(client: &LspClient, file: &Path, at: Position) -> Vec<String> {
    let started = Instant::now();
    loop {
        let items: Vec<String> = client
            .request_completion_list(
                file,
                at,
                Trigger::Character('.').context(),
                Duration::from_secs(10),
                &mut || false,
            )
            .map(|(i, _)| i.into_iter().map(|c| c.label).collect())
            .unwrap_or_default();
        if !items.is_empty() || started.elapsed() > Duration::from_secs(60) {
            return items;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

#[test]
fn utf8_is_negotiated_and_every_position_after_an_emoji_lands() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-encoding");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_encoding\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let file = dir.join("src").join("main.rs");
    std::fs::write(&file, MAIN).unwrap();
    let client = LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn");
    client.did_open(&file, "rust", 1, MAIN).unwrap();

    // 1. The server took the offer.
    assert_eq!(client.position_encoding(), PositionEncoding::Utf8);

    // 2. F12 on `n` in `let m = n` goes to `let n`, both after the emoji.
    let found = definition(&client, &file, at(&client, MAIN, 1, "n", 2));
    assert_eq!(found, Some(at(&client, MAIN, 1, "n", 0)), "definition");

    // 3. `s.` after an emoji on its line lists the methods of `&str`.
    let after_dot = {
        let mut p = at(&client, MAIN, 2, "s.", 0);
        p.character += 2;
        p
    };
    let items = labels(&client, &file, after_dot);
    assert!(items.iter().any(|l| l == "len"), "{items:?}");

    // 4. Incremental edits that add and remove multi-byte text keep the
    //    server's copy equal to ours: F12 still lands after each one.
    for (version, text) in [
        (2, MAIN.replace("\"é🦀\"", "\"é🦀🦀é\"")),
        (
            3,
            MAIN.replace("let s = \"é🦀\";", "let s = \"🦀\"; let é = 1;"),
        ),
        (4, MAIN.replace("let t = \"🦀\";", "let t = \"\";")),
    ] {
        client.did_change(&file, version, &text).unwrap();
        let found = definition(&client, &file, at(&client, &text, 1, "n", 2));
        assert_eq!(
            found,
            Some(at(&client, &text, 1, "n", 0)),
            "after edit {version}: {text}"
        );
    }
    client.shutdown();
}
