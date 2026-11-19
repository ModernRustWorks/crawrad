//! Golden test for the "Project templates" manual page.
//!
//! Bless a reviewed change with `CRAWRAD_BLESS=1 cargo test -p crawrad-project reference`.

use super::templates_reference_markdown;

const GOLDEN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/src/reference/templates.md"
);

#[test]
fn the_templates_page_matches_the_catalogues() {
    let now = templates_reference_markdown();
    if std::env::var_os("CRAWRAD_BLESS").is_some() {
        std::fs::write(GOLDEN, &now).unwrap();
        return;
    }
    let golden = std::fs::read_to_string(GOLDEN)
        .unwrap_or_default()
        .replace("\r\n", "\n");
    assert!(
        now == golden,
        "the template catalogue changed: review, then bless with CRAWRAD_BLESS=1"
    );
}
