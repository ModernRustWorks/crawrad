use crate::font::{CRAWRAD_FONT_DIALOG_ANSWER, installed_families, test_answer};

#[test]
fn installed_families_is_sorted_deduplicated_and_has_no_at_prefix() {
    let list = installed_families();

    #[cfg(target_os = "windows")]
    assert!(
        !list.is_empty(),
        "on Windows, installed_families() must discover at least one system font family"
    );

    // Sorted and deduplicated check: each element is strictly less than the next
    for pair in list.windows(2) {
        assert!(
            pair[0] < pair[1],
            "families not sorted or deduplicated: {:?} >= {:?}",
            pair[0],
            pair[1]
        );
    }

    // No vertical font families with `@`
    for family in list {
        assert!(
            !family.starts_with('@'),
            "vertical font family starting with '@' was not skipped: {family}"
        );
    }
}

/// The answer a test sets is what the dialog's door reads, and nothing when unset.
#[test]
fn the_test_answer_is_read_where_the_dialog_would_open() {
    // SAFETY: the only test that touches this variable.
    unsafe { std::env::set_var(CRAWRAD_FONT_DIALOG_ANSWER, "Consolas, 10.0pt") };
    assert_eq!(test_answer().as_deref(), Some("Consolas, 10.0pt"));
    unsafe { std::env::remove_var(CRAWRAD_FONT_DIALOG_ANSWER) };
    assert_eq!(test_answer(), None);
}
