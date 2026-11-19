//! Golden test for the "Menus and commands" manual page.
//!
//! Bless a reviewed change with `CRAWRAD_BLESS=1 cargo test -p crawrad-actions reference`.

use super::commands_reference_markdown;

const GOLDEN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/src/reference/commands.md"
);

#[test]
fn the_commands_page_matches_the_catalogue() {
    let now = commands_reference_markdown();
    if std::env::var_os("CRAWRAD_BLESS").is_some() {
        std::fs::write(GOLDEN, &now).unwrap();
        return;
    }
    let golden = std::fs::read_to_string(GOLDEN)
        .unwrap_or_default()
        .replace("\r\n", "\n");
    assert!(
        now == golden,
        "the command catalogue changed: review, then bless with CRAWRAD_BLESS=1"
    );
}

/// The key schemes are named without another product's name: what the user
/// reads in Tools > Options and in the manual is "Modern" and "Classic".
#[test]
fn the_keymap_names_carry_no_other_products_name() {
    let other = ["vs", " code"].concat();
    for scheme in crate::KeymapScheme::ALL {
        for text in [scheme.label(), scheme.description()] {
            assert!(!text.to_lowercase().contains(&other), "{text}");
        }
    }
    assert!(
        !commands_reference_markdown()
            .to_lowercase()
            .contains(&other)
    );
}
