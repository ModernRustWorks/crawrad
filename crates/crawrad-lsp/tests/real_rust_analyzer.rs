//! Navigation, rename and format against the REAL rust-analyzer.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING. The fakes in `src/test/` pin the
//! shapes; this one pins that the server really answers in those shapes.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crawrad_lsp::{LspClient, Position, apply_text_edits};

const MAIN: &str = "trait Shape {
    fn area(&self) -> u32;
}

struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

fn helper() -> u32 {
    7
}

fn main() {
    let sq = Square { side: helper() };
    let total = sq.area() + helper();
    println!(\"{total}\");
}
";

const T: Duration = Duration::from_secs(5);

fn at(line: u32, character: u32) -> Position {
    Position { line, character }
}

/// A throwaway crate under the target dir, so the server has a workspace.
fn scratch_crate() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-probe");
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(dir.join("src").join("main.rs"), MAIN).unwrap();
    dir
}

/// Asks until the server has indexed (an empty answer while it loads).
fn until_some<T>(mut ask: impl FnMut() -> Option<T>) -> T {
    let start = Instant::now();
    loop {
        if let Some(v) = ask() {
            return v;
        }
        assert!(
            start.elapsed() < Duration::from_secs(90),
            "rust-analyzer never answered"
        );
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn lines_of(locs: &[crawrad_lsp::Location], file: &Path) -> Vec<u32> {
    let uri = crawrad_lsp::path_to_uri(file);
    let mut lines: Vec<u32> = locs
        .iter()
        .filter(|l| l.uri.eq_ignore_ascii_case(&uri))
        .map(|l| l.range.start.line)
        .collect();
    lines.sort_unstable();
    lines
}

#[test]
fn the_real_server_navigates_renames_and_formats() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let root = scratch_crate();
    let file = root.join("src").join("main.rs");
    let client = LspClient::spawn(Some(PathBuf::from(server)), &root).expect("spawn");
    client.did_open(&file, "rust", 1, MAIN).unwrap();

    // Go to Definition of `helper` in `side: helper()` (line 19): a
    // LocationLink, landing on the name at line 14.
    let def = until_some(|| {
        let v = client.request_definition(&file, 19, 28, T).ok()?;
        (!v.is_empty()).then_some(v)
    });
    assert_eq!(lines_of(&def, &file), vec![14]);
    assert_eq!(def[0].range.start.character, 3, "lands on the name");

    // Type Definition of `sq` (line 19) → `struct Square` (line 4).
    let ty = client.request_type_definition(&file, at(19, 8), T).unwrap();
    assert_eq!(lines_of(&ty, &file), vec![4]);

    // Implementations of `Shape` (line 0) → `impl Shape for Square` (line 8).
    let imps = client.request_implementation(&file, at(0, 7), T).unwrap();
    assert_eq!(lines_of(&imps, &file), vec![8]);

    // References of `helper`: the two calls, plus the declaration if asked.
    let uses = client
        .request_references(&file, at(14, 4), false, T)
        .unwrap();
    assert_eq!(lines_of(&uses, &file), vec![19, 20]);
    let all = client
        .request_references(&file, at(14, 4), true, T)
        .unwrap();
    assert_eq!(lines_of(&all, &file), vec![14, 19, 20]);

    // Rename: prepare gives the name's span; the rename edits all three.
    let span = client
        .request_prepare_rename(&file, at(14, 4), T)
        .unwrap()
        .expect("helper is renameable");
    assert_eq!((span.start.line, span.start.character), (14, 3));
    let edit = client.request_rename(&file, at(14, 4), "seven", T).unwrap();
    let edits = edit
        .iter()
        .find(|(uri, _)| uri.eq_ignore_ascii_case(&crawrad_lsp::path_to_uri(&file)))
        .map(|(_, e)| e.clone())
        .expect("edits for main.rs");
    let renamed = apply_text_edits(MAIN, &edits, client.position_encoding());
    assert_eq!(renamed.matches("seven()").count(), 3, "{renamed}");
    assert!(!renamed.contains("helper"), "{renamed}");

    // Format: a badly spaced line comes back as rustfmt writes it.
    let ugly = MAIN.replace("    7\n", "        7   \n");
    client.did_change(&file, 2, &ugly).unwrap();
    let fmt = client.request_formatting(&file, 4, true, T).unwrap();
    assert_eq!(
        apply_text_edits(&ugly, &fmt, client.position_encoding()),
        MAIN
    );

    client.shutdown();
}
