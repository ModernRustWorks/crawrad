//! Golden test for the "Keyboard shortcuts" manual page.
//!
//! Bless a reviewed change with `CRAWRAD_BLESS=1 cargo test -p crawrad-host shortcuts`.

use super::shortcuts_reference_markdown;

const GOLDEN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/src/reference/shortcuts.md"
);

#[test]
fn the_shortcuts_page_matches_the_chord_table() {
    let now = shortcuts_reference_markdown();
    if std::env::var_os("CRAWRAD_BLESS").is_some() {
        std::fs::write(GOLDEN, &now).unwrap();
        return;
    }
    let golden = std::fs::read_to_string(GOLDEN)
        .unwrap_or_default()
        .replace("\r\n", "\n");
    assert!(
        now == golden,
        "the editor chords changed: review, then bless with CRAWRAD_BLESS=1"
    );
}
