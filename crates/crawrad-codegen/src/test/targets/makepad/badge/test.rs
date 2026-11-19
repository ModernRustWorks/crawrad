//! The generator's `badge_text` answers exactly as the runtime's.

use super::badge_text;

#[test]
fn the_generator_reads_a_badge_as_the_runtime_does() {
    for (caption, count, max) in [
        ("", 0, 99),
        ("New", 0, 99),
        ("New", 5, 99),
        ("", 99, 99),
        ("", 150, 99),
        ("", 10, 9),
    ] {
        assert_eq!(
            badge_text(caption, count, max),
            crawrad_runtime_makepad::badge::badge_text(caption, count, max),
            "({caption:?}, {count}, {max})"
        );
    }
}
