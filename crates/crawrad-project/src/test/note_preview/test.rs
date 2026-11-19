use super::*;

#[test]
fn the_preview_puts_the_signature_on_top_of_the_body() {
    let md = preview_markdown("pub fn fetch(&self, id: &str)", "Downloads one package.");
    assert_eq!(
        md,
        "```rust\npub fn fetch(&self, id: &str)\n```\n\n---\n\nDownloads one package."
    );
    assert_eq!(preview_markdown("  ", "Only body."), "Only body.");
}

#[test]
fn an_intra_doc_link_shows_as_code_and_a_real_link_stays() {
    let md = preview_markdown("", "Returns [`RegistryError::NotFound`] when missing.");
    assert_eq!(md, "Returns `RegistryError::NotFound` when missing.");
    let md = preview_markdown("", "See [`Vec`](https://doc.rust-lang.org/std/vec/).");
    assert_eq!(md, "See [`Vec`](https://doc.rust-lang.org/std/vec/).");
    let md = preview_markdown("", "Broken [`open and more");
    assert_eq!(md, "Broken [`open and more");
}

#[test]
fn only_a_rust_block_is_an_example() {
    assert!(has_example("Text\n\n```\nlet x = 1;\n```\n"));
    assert!(has_example("```rust,no_run\nmain();\n```"));
    assert!(!has_example("```text\nnot code\n```"));
    assert!(!has_example("```ignore\nskip();\n```"));
    assert!(!has_example("No code here."));
}

#[test]
fn the_package_name_comes_from_the_package_table() {
    let toml = "[workspace]\nname = \"not-this\"\n\n[package]\nname = \"crawrad-registry\"\nversion = \"0.1.0\"\n";
    assert_eq!(package_name(toml).as_deref(), Some("crawrad-registry"));
    assert_eq!(package_name("[dependencies]\nname = \"x\"\n"), None);
}

#[test]
fn the_run_filters_by_the_item_path() {
    let run = DoctestRun::new("crawrad-registry", Some("RegistryClient"), "fetch");
    assert_eq!(
        run.args(),
        [
            "test",
            "--doc",
            "-p",
            "crawrad-registry",
            "--",
            "RegistryClient::fetch"
        ]
    );
    assert_eq!(DoctestRun::new("c", None, "parse").path, "parse");
}

fn fetch() -> DoctestRun {
    DoctestRun::new("dtprobe", Some("Client"), "fetch")
}

/// Real cargo output (Windows, `cargo test --doc -- Client::fetch`): the
/// filter also ran `Client::fetch_all`, which fails. Only `fetch` counts.
const REAL_MIXED: &str = "   Doc-tests dtprobe\n\nrunning 2 tests\ntest src\\lib.rs - Client::fetch_all (line 11) ... FAILED\ntest src\\lib.rs - Client::fetch (line 5) ... ok\n\nfailures:\n\n---- src\\lib.rs - Client::fetch_all (line 11) stdout ----\nerror[E0425]: cannot find value `y` in this scope\n  --> src\\lib.rs:13:41\n\nerror: aborting due to 1 previous error\n\nCouldn't compile the test.\n\nfailures:\n    src\\lib.rs - Client::fetch_all (line 11)\n\ntest result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.63s\n\nerror: doctest failed, to rerun pass `-p dtprobe --doc`\n";

#[test]
fn a_neighbour_that_fails_does_not_fail_the_item() {
    let o = parse_doctest_output(REAL_MIXED, &fetch());
    assert_eq!(
        o,
        DoctestOutcome::Passed {
            count: 1,
            secs: 0.63
        }
    );
    assert!(o.passed());
    assert_eq!(o.label(), "Doctest passed · 0.6 s");
}

#[test]
fn the_failing_item_names_its_own_error() {
    let run = DoctestRun::new("dtprobe", Some("Client"), "fetch_all");
    assert_eq!(
        parse_doctest_output(REAL_MIXED, &run),
        DoctestOutcome::Failed {
            failed: 1,
            detail: "error[E0425]: cannot find value `y` in this scope".into()
        }
    );
}

#[test]
fn no_doctest_of_the_item_is_its_own_answer() {
    let out = "running 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s\n";
    assert_eq!(parse_doctest_output(out, &fetch()), DoctestOutcome::NoTest);
    let other = DoctestRun::new("dtprobe", Some("Other"), "fetch");
    assert_eq!(
        parse_doctest_output(REAL_MIXED, &other),
        DoctestOutcome::NoTest
    );
}

#[test]
fn a_crate_that_does_not_build_is_an_error_not_a_pass() {
    let out =
        "   Compiling x v0.1.0\nerror[E0308]: mismatched types\nerror: could not compile `x`\n";
    assert_eq!(
        parse_doctest_output(out, &fetch()),
        DoctestOutcome::Error("error[E0308]: mismatched types".into())
    );
}

#[test]
fn section_headings_drop_three_levels_outside_code() {
    let body = "Sum.\n\n# Examples\n\n```\n# use demo::add;\n```\n\n## Notes\n#tag stays\n";
    let md = preview_markdown("", body);
    assert!(md.contains("\n#### Examples\n"), "{md}");
    assert!(md.contains("\n##### Notes\n"), "{md}");
    assert!(
        md.contains("\n# use demo::add;\n"),
        "a hidden doctest line is code: {md}"
    );
    assert!(md.contains("\n#tag stays\n"), "not a heading: {md}");
}

#[test]
fn the_note_on_a_definition_is_found_from_its_name_line() {
    let src = "use crawrad_note::note;\n\n#[note(\"add-two\")]\npub fn add_two(n: i32) -> i32 {\n    n + 2\n}\n\npub fn add_four(n: i32) -> i32 {\n    add_two(add_two(n))\n}\n";
    assert_eq!(note_at_definition(src, 3).as_deref(), Some("add-two"));
    assert_eq!(note_at_definition(src, 7), None, "an item with no note");
    assert_eq!(note_at_definition(src, 0), None, "not an item");
}
