//! Format Document against the REAL rust-analyzer (rustfmt), through the
//! worker the IDE uses.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>`; without it
//! the test passes and proves NOTHING.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crawrad_lsp::LspClient;
use crawrad_lsp::format::{FormatWorker, Verdict, verdict};

const UGLY: &str = "fn main( ){let x=1;let s = \"é🦀\";println!(\"{x} {s}\")}\n";
const PRETTY: &str =
    "fn main() {\n    let x = 1;\n    let s = \"é🦀\";\n    println!(\"{x} {s}\")\n}\n";

fn judge(
    worker: &FormatWorker,
    client: &Arc<LspClient>,
    ticket: u64,
    file: &Path,
    text: &str,
) -> Verdict {
    worker.request(Arc::clone(client), ticket, file, 1, 4);
    let started = Instant::now();
    loop {
        if let Some(a) = worker.take(ticket) {
            return verdict(a.outcome, true, text, client.position_encoding());
        }
        assert!(started.elapsed() < Duration::from_secs(60), "no answer");
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn rustfmt_through_the_worker() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ra-format");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_format\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    let file = dir.join("src").join("main.rs");
    std::fs::write(&file, UGLY).unwrap();
    let client = Arc::new(LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn"));
    client.did_open(&file, "rust", 1, UGLY).unwrap();
    let worker = FormatWorker::new();

    // 1. The ugly file comes back as rustfmt writes it (accent and emoji kept).
    let got = judge(&worker, &client, 1, &file, UGLY);
    eprintln!("ugly -> {got:?}");
    assert_eq!(got, Verdict::Apply(PRETTY.to_string()));

    // 2. Once formatted, the answer changes nothing: "Already formatted".
    client.did_change(&file, 2, PRETTY).unwrap();
    let again = judge(&worker, &client, 2, &file, PRETTY);
    eprintln!("pretty -> {again:?}");
    assert_eq!(again, Verdict::Say("Already formatted"));
    client.shutdown();
}
