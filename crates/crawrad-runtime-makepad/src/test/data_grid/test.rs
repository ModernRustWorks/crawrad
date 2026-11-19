//! The Makepad data grid's column layout: own widths kept, the rest split evenly.

use crate::data_grid::{MAX_COLUMNS, MIN_COLUMN_WIDTH, cell_id, column_widths};
use crawrad_runtime::grid::{ColumnAlign, GridColumn};
use makepad_widgets::*;

fn free(n: usize) -> Vec<GridColumn> {
    (0..n)
        .map(|i| GridColumn::new(format!("c{i}"), "", ColumnAlign::Left))
        .collect()
}

#[test]
fn the_columns_split_the_grid_width_evenly() {
    assert_eq!(column_widths(600.0, &free(4)), vec![150.0; 4]);
    assert_eq!(column_widths(600.0, &free(1)), vec![600.0]);
}

#[test]
fn a_column_never_gets_narrower_than_the_minimum() {
    assert_eq!(column_widths(600.0, &free(16)), vec![MIN_COLUMN_WIDTH; 16]);
    // Before the first layout the width is 0.
    assert_eq!(column_widths(0.0, &free(3)), vec![MIN_COLUMN_WIDTH; 3]);
    assert!(column_widths(600.0, &[]).is_empty());
}

/// A column with its own `width` keeps it; the free ones share the rest.
#[test]
fn a_column_width_is_kept_and_the_rest_is_shared() {
    let mut cols = free(3);
    cols[0].width = Some(100.0);
    assert_eq!(column_widths(600.0, &cols), vec![100.0, 250.0, 250.0]);
}

/// A free column with min_width clamps to it and redistributes.
#[test]
fn a_free_column_respects_min_width() {
    let mut cols = free(3);
    cols[0].min_width = 120.0;
    assert_eq!(column_widths(300.0, &cols), vec![120.0, 90.0, 90.0]);
}

/// A free column with max_width clamps to it and redistributes.
#[test]
fn a_free_column_respects_max_width() {
    let mut cols = free(3);
    cols[0].max_width = Some(80.0);
    assert_eq!(column_widths(300.0, &cols), vec![80.0, 110.0, 110.0]);
}

#[test]
fn every_slot_has_its_own_id() {
    assert_eq!(cell_id(0), live_id!(c0));
    assert_eq!(cell_id(MAX_COLUMNS - 1), live_id!(c15));
    let ids: std::collections::HashSet<_> = (0..MAX_COLUMNS).map(cell_id).collect();
    assert_eq!(ids.len(), MAX_COLUMNS);
}

/// The constant and the DSL agree: the header and the row template each have
/// exactly `MAX_COLUMNS` cell slots.
#[test]
fn the_dsl_has_one_slot_per_column_in_the_header_and_the_row() {
    let source = include_str!("../../data_grid.rs");
    // GridCell: the header, the filter bar and the summary bar; GridDataCell: the row template.
    for (template, bars) in [("GridCell", 3), ("GridDataCell", 1)] {
        let last = format!("c{} = <{template}>", MAX_COLUMNS - 1);
        let past = format!("c{} = <{template}>", MAX_COLUMNS);
        assert_eq!(source.matches(&last).count(), bars, "{template}");
        assert_eq!(source.matches(&past).count(), 0, "{template}");
    }
}
