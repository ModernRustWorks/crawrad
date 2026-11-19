//! Code lenses against the REAL rust-analyzer, through the same worker the
//! IDE will use: a trait with four implementations reads
//! "4 implementations", each struct "1 implementation", and the lens carries
//! the places to list.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crawrad_lsp::{CodeLens, CodeLensWorker, LensKind, LspClient};

const MAIN: &str = "trait Animal {\n    fn name(&self) -> &str;\n}\n\nstruct Dog {}\nstruct Cat {}\nstruct Cow {}\nstruct Owl {}\n\nimpl Animal for Dog {\n    fn name(&self) -> &str { \"dog\" }\n}\nimpl Animal for Cat {\n    fn name(&self) -> &str { \"cat\" }\n}\nimpl Animal for Cow {\n    fn name(&self) -> &str { \"cow\" }\n}\nimpl Animal for Owl {\n    fn name(&self) -> &str { \"owl\" }\n}\n\nfn main() {\n    let all: [&dyn Animal; 4] = [&Dog {}, &Cat {}, &Cow {}, &Owl {}];\n    for a in all {\n        println!(\"{}\", a.name());\n    }\n}\n";

/// The title of the resolved lens on `line`, if any.
fn title_on(lenses: &[CodeLens], line: u32) -> Option<&str> {
    lenses
        .iter()
        .filter(|l| l.range.start.line == line)
        .find_map(|l| l.command.as_ref().map(|c| c.title.as_str()))
}

/// Asks until rust-analyzer has indexed (the trait's count settles at 4).
fn ask(worker: &CodeLensWorker, client: &Arc<LspClient>, file: &Path) -> Vec<CodeLens> {
    let started = Instant::now();
    loop {
        worker.request(Arc::clone(client), file, 1);
        let answer = loop {
            if let Some(a) = worker.take() {
                break a;
            }
            assert!(started.elapsed() < Duration::from_secs(120), "no answer");
            std::thread::sleep(Duration::from_millis(50));
        };
        assert_eq!((answer.file.as_path(), answer.version), (file, 1));
        if title_on(&answer.lenses, 0) == Some("4 implementations") {
            return answer.lenses;
        }
        assert!(
            started.elapsed() < Duration::from_secs(90),
            "{:?}",
            answer.lenses
        );
        std::thread::sleep(Duration::from_millis(500));
    }
}

#[test]
fn a_trait_with_four_impls_reads_four_implementations() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-code-lens");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_code_lens\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n",
    )
    .unwrap();
    let main = dir.join("src").join("main.rs");
    std::fs::write(&main, MAIN).unwrap();

    let client = Arc::new(LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn"));
    client.did_open(&main, "rust", 1, MAIN).unwrap();
    let worker = CodeLensWorker::new();
    let lenses = ask(&worker, &client, &main);
    eprintln!("{} lenses: {lenses:?}", lenses.len());

    // the trait: its four impls, as places to list
    let trait_lens = lenses
        .iter()
        .find(|l| l.range.start.line == 0 && l.command.is_some())
        .expect("a lens on the trait");
    let Some(LensKind::ShowLocations(places)) = trait_lens.command.as_ref().map(|c| &c.kind) else {
        panic!("the trait's lens lists no places: {trait_lens:?}");
    };
    let mut lines: Vec<u32> = places.iter().map(|p| p.range.start.line).collect();
    lines.sort_unstable();
    assert_eq!(lines, vec![9, 12, 15, 18]);

    // each struct: one implementation
    for line in 4..8 {
        assert_eq!(
            title_on(&lenses, line),
            Some("1 implementation"),
            "line {line}: {lenses:?}"
        );
    }
    // `fn main` gets Run/Debug, told apart from the places
    assert!(
        lenses.iter().any(|l| l.range.start.line == 22
            && l.command.as_ref().is_some_and(|c| c.kind == LensKind::Run)),
        "{lenses:?}"
    );
    client.shutdown();
}
