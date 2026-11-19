//! Incremental document sync against the REAL rust-analyzer.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crawrad_lsp::sync::SyncKind;
use crawrad_lsp::{LspClient, Position};

const T: Duration = Duration::from_secs(5);

/// A file of ~2,000 lines: what a key cost before was this whole text.
fn big_main() -> String {
    let mut s = String::from("fn main() {\n    let base = 1;\n}\n");
    for i in 0..1000 {
        s.push_str(&format!("fn filler_{i}() -> u32 {{\n    {i}\n}}\n"));
    }
    s
}

fn scratch_crate(text: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-incremental");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_incremental\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(dir.join("src").join("main.rs"), text).unwrap();
    dir
}

fn definition_line(client: &LspClient, file: &Path, at: Position) -> Option<u32> {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(90) {
        if let Ok(locs) = client.request_definition(file, at.line, at.character, T)
            && let Some(l) = locs.first()
        {
            return Some(l.range.start.line);
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    None
}

/// **The server announces Incremental; a typing session sent as deltas
/// keeps it in step; a key costs bytes, not the file.**
#[test]
fn a_typing_session_sent_as_deltas_keeps_the_server_in_step() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let mut text = big_main();
    let root = scratch_crate(&text);
    let file = root.join("src").join("main.rs");
    let client = LspClient::spawn(Some(PathBuf::from(server)), &root).expect("spawn");
    assert_eq!(
        client.sync_kind(),
        SyncKind::Incremental,
        "rust-analyzer offers Incremental"
    );
    client.did_open(&file, "rust", 1, &text).unwrap();
    let mut version = 1;
    let mut send = |text: &str| {
        version += 1;
        client.did_change(&file, version, text).unwrap();
    };

    // Type, key by key, a new function right after `main`...
    let after_main = "fn main() {\n    let base = 1;\n}\n".len();
    let new_fn = "fn eleven() -> u32 {\n    11\n}\n";
    for (i, c) in new_fn.char_indices() {
        text.insert(after_main + i, c);
        send(&text);
    }
    let one_key = client.last_change_bytes();
    // ...a typo typed and erased...
    let at = text.find("let base = 1;").unwrap() + "let base = 1;".len();
    for c in "xyz".chars().rev() {
        text.insert(at, c);
        send(&text);
    }
    for _ in 0..3 {
        text.remove(at);
        send(&text);
    }
    // ...and a call to it inside `main`, key by key.
    // The call is typed with a slip (`elevenzz`) and the slip erased with
    // Backspace: only if the server got the deletions does `eleven` resolve.
    let call = "\n    let e = elevenzz();";
    for (i, c) in call.char_indices() {
        text.insert(at + i, c);
        send(&text);
    }
    for _ in 0..2 {
        let z = text.find("zz(").unwrap_or_else(|| text.find("z(").unwrap());
        text.remove(z);
        send(&text);
    }

    // The server must hold the same text: the call resolves to the line
    // of `fn eleven`, which only exists through the deltas.
    let call_line = text[..text.find("eleven();").unwrap()]
        .matches('\n')
        .count() as u32;
    let col = text
        .lines()
        .nth(call_line as usize)
        .unwrap()
        .find("eleven")
        .unwrap() as u32;
    let def = definition_line(
        &client,
        &file,
        Position {
            line: call_line,
            character: col + 1,
        },
    );
    let whole = serde_json::json!({ "text": text }).to_string().len();
    eprintln!("one key on the wire: {one_key} bytes; the whole file: {whole} bytes");
    let fn_line = text[..text.find("fn eleven").unwrap()]
        .matches('\n')
        .count() as u32;
    assert_eq!(
        def,
        Some(fn_line),
        "the server's text diverged from the editor's"
    );
    assert!(
        one_key < 400,
        "a key must cost bytes, not the file: {one_key}"
    );
    assert!(whole > 30_000, "the file is big enough to matter: {whole}");
    client.shutdown();
}
