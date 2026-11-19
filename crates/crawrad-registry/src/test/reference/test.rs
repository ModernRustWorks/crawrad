//! Golden test for the component catalogue manual page.
//!
//! Generates "Components and properties" from the registry and checks it
//! against `docs/src/reference/components.md`. Bless changes with
//! `CRAWRAD_BLESS=1 cargo test -p crawrad-registry reference`.

use crate::Registry;

const GOLDEN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/src/reference/components.md"
);

#[test]
fn reference_markdown_matches_the_documentation() {
    let now = Registry::builtin().reference_markdown();
    if std::env::var_os("CRAWRAD_BLESS").is_some() {
        std::fs::write(GOLDEN, &now).unwrap();
        return;
    }
    let golden = std::fs::read_to_string(GOLDEN).unwrap_or_default();
    let golden = golden.replace("\r\n", "\n");
    let changed: Vec<_> = (now.lines().zip(golden.lines()))
        .filter(|(n, g)| n != g)
        .map(|(n, g)| format!("  now:    {n}\n  golden: {g}"))
        .collect();
    assert!(
        changed.is_empty() && now.lines().count() == golden.lines().count(),
        "the component reference changed — review, then bless with CRAWRAD_BLESS=1:\n{}",
        changed.join("\n")
    );
}
