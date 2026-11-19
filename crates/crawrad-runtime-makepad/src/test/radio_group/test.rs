//! Where a RadioGroup lays out its items and what it draws.

use super::{GroupState, cells, commands};
use crate::paint::Command;

#[test]
fn the_items_fill_each_column_top_down() {
    let c = cells(200.0, 100.0, 5, 2);
    // three rows: items 0..2 in the first column, 3..4 in the second
    assert_eq!(c[0].0, c[2].0);
    assert!(c[3].0 > c[0].0);
    assert_eq!(c[3].1, c[0].1);
    assert!(c[1].1 > c[0].1);
}

#[test]
fn only_the_chosen_item_gets_its_dot() {
    let state = GroupState {
        caption: "Pick".into(),
        items: vec!["a".into(), "b".into(), "c".into()],
        index: 1,
        ..GroupState::default()
    };
    let circles = commands(&state, 160.0, 90.0, 24.0)
        .into_iter()
        .filter(|c| matches!(c, Command::Circle { .. }))
        .count();
    // a ring per item, and one dot
    assert_eq!(circles, 4);
}

#[test]
fn none_chosen_draws_no_dot() {
    let state = GroupState {
        items: vec!["a".into(), "b".into()],
        ..GroupState::default()
    };
    let circles = commands(&state, 160.0, 90.0, 0.0)
        .into_iter()
        .filter(|c| matches!(c, Command::Circle { .. }))
        .count();
    assert_eq!(circles, 2);
}
