//! The Tab sequence's rule (`tab_order.rs`), with no window.

use super::*;

#[test]
fn tab_goes_to_the_next_stop_and_wraps_round() {
    let all = [true, true, true];
    assert_eq!(next(None, &all, true), Some(0), "no focus: the first");
    assert_eq!(next(Some(0), &all, true), Some(1));
    assert_eq!(next(Some(2), &all, true), Some(0), "round from the last");
}

#[test]
fn shift_tab_goes_backwards() {
    let all = [true, true, true];
    assert_eq!(next(None, &all, false), Some(2), "no focus: the last");
    assert_eq!(next(Some(2), &all, false), Some(1));
    assert_eq!(next(Some(0), &all, false), Some(2), "round from the first");
}

#[test]
fn a_hidden_or_disabled_stop_is_skipped() {
    let middle_off = [true, false, true];
    assert_eq!(next(Some(0), &middle_off, true), Some(2));
    assert_eq!(next(Some(2), &middle_off, false), Some(0));
    assert_eq!(next(None, &[false, true], true), Some(1));
}

#[test]
fn with_no_stop_on_nothing_moves() {
    assert_eq!(next(None, &[], true), None);
    assert_eq!(next(None, &[false, false], true), None);
    assert_eq!(next(Some(0), &[true], true), Some(0), "alone, it keeps it");
}
