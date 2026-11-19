//! Go to Definition against the REAL rust-analyzer, through the same worker
//! the IDE uses.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING. Needs the `rust-src` component for
//! the `std` case.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crawrad_lsp::LspClient;
use crawrad_lsp::definition::{DefinitionWorker, Outcome};

const MAIN: &str = "mod shapes;\n\nfn main() {\n    let a = shapes::area(2);\n    let s = String::new();\n    let _ = (a, s);\n    let p = P { a };\n    let _ = p.a;\n}\n\nstruct P {\n    a: i32,\n}\n";
const SHAPES: &str = "// \u{1F980} shapes\npub fn area(x: i32) -> i32 {\n    x * x\n}\n";

/// The caret at the `k`-th byte of `needle` on `line` of `text`.
fn caret(text: &str, line: usize, needle: &str, k: usize) -> (usize, usize) {
    (
        line,
        text.lines().nth(line).unwrap().find(needle).unwrap() + k,
    )
}

/// Asks until rust-analyzer has indexed (an empty answer while it loads).
fn ask(
    worker: &DefinitionWorker,
    client: &Arc<LspClient>,
    ticket: &mut u64,
    file: &Path,
    at: (usize, usize),
) -> Outcome {
    let started = Instant::now();
    loop {
        *ticket += 1;
        worker.request(Arc::clone(client), *ticket, file, 1, MAIN, at);
        let answer = loop {
            if let Some(a) = worker.take(*ticket) {
                break a;
            }
            assert!(started.elapsed() < Duration::from_secs(90), "no answer");
            std::thread::sleep(Duration::from_millis(50));
        };
        assert_eq!((answer.file.as_path(), answer.version), (file, 1));
        if !matches!(answer.outcome, Outcome::Nothing)
            || started.elapsed() > Duration::from_secs(60)
        {
            return answer.outcome;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

#[test]
fn definitions_in_the_project_and_in_std() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-definition");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_definition\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let main = dir.join("src").join("main.rs");
    let shapes = dir.join("src").join("shapes.rs");
    std::fs::write(&main, MAIN).unwrap();
    std::fs::write(&shapes, SHAPES).unwrap();

    let client = Arc::new(LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn"));
    client.did_open(&main, "rust", 1, MAIN).unwrap();
    let worker = DefinitionWorker::new();
    let mut ticket = 0;

    // 1. `area` in main.rs -> `pub fn area` in shapes.rs, on the name
    //    (a `LocationLink`'s `targetSelectionRange`).
    let found = ask(
        &worker,
        &client,
        &mut ticket,
        &main,
        caret(MAIN, 3, "area", 2),
    );
    let Outcome::Found(places) = found else {
        panic!("area: {found:?}")
    };
    eprintln!("area -> {places:?}");
    let want_byte = SHAPES.lines().nth(1).unwrap().find("area").unwrap();
    assert_eq!(places.len(), 1);
    assert_eq!(places[0].file.file_name().unwrap(), "shapes.rs");
    assert_eq!((places[0].line, places[0].byte), (1, want_byte));

    // 2. `new` in `String::new()` -> std's string.rs, under the toolchain.
    let found = ask(
        &worker,
        &client,
        &mut ticket,
        &main,
        caret(MAIN, 4, "new", 1),
    );
    let Outcome::Found(places) = found else {
        panic!("String::new: {found:?}")
    };
    eprintln!("String::new -> {places:?}");
    let path = places[0].file.to_string_lossy().replace('\\', "/");
    assert!(
        path.contains("/rustlib/") && path.ends_with("/string.rs"),
        "{path}"
    );

    // 2b. A shorthand field `P { a }` is two definitions: the local `a`
    //     and the field `a` — the case the IDE offers a list for.
    let found = ask(
        &worker,
        &client,
        &mut ticket,
        &main,
        caret(MAIN, 6, "{ a }", 2),
    );
    let Outcome::Found(places) = found else {
        panic!("P {{ a }}: {found:?}")
    };
    eprintln!("P {{ a }} -> {places:?}");
    let lines: Vec<usize> = places.iter().map(|p| p.line).collect();
    assert_eq!(places.len(), 2, "{places:?}");
    assert!(
        lines.contains(&3) && lines.contains(&11),
        "the local (line 3) and the field (line 11): {lines:?}"
    );

    // 3. A blank spot has no definition: one answer, at once, not a hang
    //    (the index is loaded by now).
    worker.request(Arc::clone(&client), 900, &main, 1, MAIN, (1, 0));
    let started = Instant::now();
    let blank = loop {
        if let Some(a) = worker.take(900) {
            break a.outcome;
        }
        assert!(started.elapsed() < Duration::from_secs(10), "no answer");
        std::thread::sleep(Duration::from_millis(20));
    };
    eprintln!("blank -> {blank:?} in {:?}", started.elapsed());
    assert_eq!(blank, Outcome::Nothing);

    // 4. Only the newest ticket answers: an older one is dropped.
    worker.request(
        Arc::clone(&client),
        1001,
        &main,
        1,
        MAIN,
        caret(MAIN, 3, "area", 0),
    );
    worker.request(
        Arc::clone(&client),
        1002,
        &main,
        1,
        MAIN,
        caret(MAIN, 4, "String", 0),
    );
    let started = Instant::now();
    while worker.take(1002).is_none() {
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "1002 never answered"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
    assert!(worker.take(1001).is_none(), "the older ticket was dropped");
    client.shutdown();
}
