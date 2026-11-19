//! Rename Symbol against the REAL rust-analyzer, through the worker the IDE
//! uses: `prepareRename` with where the name is declared, the edits
//! of `rename`, and the plan read from the editor's (unsaved) text.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crawrad_lsp::LspClient;
use crawrad_lsp::rename::{Asked, Outcome, RenameWorker, plan};

const MAIN: &str = "mod shapes;\n#[path = \"form.designer.rs\"]\nmod form_designer;\nuse form_designer::Form;\n\nfn main() {\n    let a = shapes::area(2);\n    let _ = (a, Form);\n}\n";
/// In the editor, NOT saved: a line more on top.
const MAIN_EDITED: &str = "// unsaved\nmod shapes;\n#[path = \"form.designer.rs\"]\nmod form_designer;\nuse form_designer::Form;\n\nfn main() {\n    let a = shapes::area(2);\n    let _ = (a, Form);\n}\n";
const SHAPES: &str = "// \u{1F980}\npub fn area(x: i32) -> i32 {\n    x * x\n}\n";
const DESIGNER: &str = "pub struct Form;\n";

fn wait(worker: &RenameWorker, ticket: u64) -> Outcome {
    let started = Instant::now();
    loop {
        if let Some(a) = worker.take(ticket) {
            return a.outcome;
        }
        assert!(started.elapsed() < Duration::from_secs(120), "no answer");
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn caret(needle: &str, k: usize) -> (usize, usize) {
    let line = MAIN_EDITED
        .lines()
        .position(|l| l.contains(needle))
        .unwrap();
    (
        line,
        MAIN_EDITED.lines().nth(line).unwrap().find(needle).unwrap() + k,
    )
}

#[test]
fn prepare_declared_rename_and_plan_against_the_real_server() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-rename");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_rename\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let main = dir.join("src").join("main.rs");
    std::fs::write(&main, MAIN).unwrap();
    std::fs::write(dir.join("src").join("shapes.rs"), SHAPES).unwrap();
    std::fs::write(dir.join("src").join("form.designer.rs"), DESIGNER).unwrap();
    let client = Arc::new(LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn"));
    client.did_open(&main, "rust", 2, MAIN_EDITED).unwrap();
    let worker = RenameWorker::new();
    let asked = |caret| Asked {
        file: &main,
        version: 2,
        text: MAIN_EDITED,
        caret,
    };

    // 1. `area` at its call: renamable, its bytes, declared in shapes.rs —
    //    asked until the server has indexed (an empty definition at first).
    let started = Instant::now();
    let mut ticket = 0;
    let (start, end, declared) = loop {
        ticket += 1;
        worker.prepare(Arc::clone(&client), ticket, asked(caret("area(2)", 1)));
        match wait(&worker, ticket) {
            Outcome::Renamable {
                start,
                end,
                declared,
            } if !declared.is_empty() => break (start, end, declared),
            other if started.elapsed() > Duration::from_secs(90) => panic!("{other:?}"),
            _ => std::thread::sleep(Duration::from_millis(500)),
        }
    };
    let line = MAIN_EDITED.lines().nth(caret("area(2)", 1).0).unwrap();
    assert_eq!(&line[start..end], "area");
    assert!(
        declared.iter().all(|p| p.ends_with("shapes.rs")),
        "{declared:?}"
    );

    // 2. `Form` (the generated-like struct): declared in form.designer.rs —
    //    what the window refuses before any dialog.
    ticket += 1;
    worker.prepare(Arc::clone(&client), ticket, asked(caret("(a, Form)", 5)));
    let Outcome::Renamable { declared, .. } = wait(&worker, ticket) else {
        panic!("Form is renamable for the server");
    };
    eprintln!("Form declared in {declared:?}");
    assert!(
        declared.iter().any(|p| p.ends_with("form.designer.rs")),
        "{declared:?}"
    );

    // 3. rename `area` to `size`: edits in both files; planned against the
    //    UNSAVED main.rs (its line on top kept), the disk for shapes.rs.
    ticket += 1;
    worker.rename(
        Arc::clone(&client),
        ticket,
        asked(caret("area(2)", 1)),
        "size",
    );
    let Outcome::Edits(edits) = wait(&worker, ticket) else {
        panic!("no edits");
    };
    let mut read = |f: &Path| match f.ends_with("main.rs") {
        true => Some(MAIN_EDITED.to_string()),
        false => std::fs::read_to_string(f).ok(),
    };
    let planned = plan(&edits, client.position_encoding(), &mut read).unwrap();
    let names: Vec<_> = planned
        .iter()
        .map(|p| p.file.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    eprintln!("planned {names:?}");
    assert_eq!(names, ["main.rs", "shapes.rs"]);
    assert!(
        planned[0].after.starts_with("// unsaved\n")
            && planned[0].after.contains("shapes::size(2)")
    );
    assert!(
        planned[1].after.contains("pub fn size(x: i32)") && planned[1].after.contains('\u{1F980}')
    );
    client.shutdown();
}
