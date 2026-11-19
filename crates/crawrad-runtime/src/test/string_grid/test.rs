use super::*;

#[test]
fn cells_from_text() {
    assert_eq!(Cells::from_text(""), Cells::default());

    let text = "Title 1\tTitle 2\nRow 1 A\t\tRow 1 C\nRow 2 A\tRow 2 B";
    let cells = Cells::from_text(text);
    assert_eq!(cells.get(0, 0), "Title 1");
    assert_eq!(cells.get(1, 0), "Title 2");
    assert_eq!(cells.get(0, 1), "Row 1 A");
    assert_eq!(cells.get(1, 1), "");
    assert!(!cells.data.contains_key(&(1, 1)));
    assert_eq!(cells.get(2, 1), "Row 1 C");
    assert_eq!(cells.get(0, 2), "Row 2 A");
    assert_eq!(cells.get(1, 2), "Row 2 B");

    let crlf = "Col 0\tCol 1\r\nVal 0\tVal 1\r\nVal 2\tVal 3";
    let crlf_cells = Cells::from_text(crlf);
    assert_eq!(crlf_cells.get(0, 0), "Col 0");
    assert_eq!(crlf_cells.get(1, 0), "Col 1");
    assert_eq!(crlf_cells.get(0, 1), "Val 0");
    assert_eq!(crlf_cells.get(1, 1), "Val 1");
    assert_eq!(crlf_cells.get(0, 2), "Val 2");
    assert_eq!(crlf_cells.get(1, 2), "Val 3");
}

#[test]
fn cells_get_set_and_clear() {
    let mut cells = Cells::default();
    assert_eq!(cells.get(0, 0), "");

    cells.set(1, 2, "hello");
    assert_eq!(cells.get(1, 2), "hello");
    assert_eq!(cells.get(0, 0), "");

    cells.set(1, 2, "");
    assert_eq!(cells.get(1, 2), "");
    assert!(cells.data.is_empty());
}

#[test]
fn grid_layout_from_props_clamps_values() {
    let layout = GridLayout::from_props(-5, -2, -1, -3, -100, 0);
    assert_eq!(layout.col_count, 0);
    assert_eq!(layout.row_count, 0);
    assert_eq!(layout.fixed_cols, 0);
    assert_eq!(layout.fixed_rows, 0);
    assert_eq!(layout.col_width, 1.0);
    assert_eq!(layout.row_height, 1.0);

    let normal = GridLayout::from_props(5, 10, 1, 2, 80, 24);
    assert_eq!(normal.col_count, 5);
    assert_eq!(normal.row_count, 10);
    assert_eq!(normal.fixed_cols, 1);
    assert_eq!(normal.fixed_rows, 2);
    assert_eq!(normal.col_width, 80.0);
    assert_eq!(normal.row_height, 24.0);
}

#[test]
fn grid_layout_is_fixed() {
    let layout = GridLayout::from_props(4, 4, 1, 1, 50, 20);
    assert!(layout.is_fixed(0, 0));
    assert!(layout.is_fixed(0, 2));
    assert!(layout.is_fixed(2, 0));
    assert!(!layout.is_fixed(1, 1));
    assert!(!layout.is_fixed(3, 3));
}

#[test]
fn grid_layout_cell_rect() {
    let layout = GridLayout::from_props(4, 4, 0, 0, 60, 25);
    assert_eq!(layout.cell_rect(0, 0), (0.0, 0.0, 60.0, 25.0));
    assert_eq!(layout.cell_rect(2, 3), (120.0, 75.0, 60.0, 25.0));
}

#[test]
fn grid_layout_cell_at_boundaries() {
    let layout = GridLayout::from_props(4, 4, 1, 1, 50, 20);

    // Top-left corner
    assert_eq!(layout.cell_at(0.0, 0.0), Some((0, 0)));

    // Middle of cell (2, 1)
    assert_eq!(layout.cell_at(125.0, 30.0), Some((2, 1)));

    // Edge before transition
    assert_eq!(layout.cell_at(49.9, 19.9), Some((0, 0)));
    assert_eq!(layout.cell_at(50.0, 20.0), Some((1, 1)));

    // Past last column
    assert_eq!(layout.cell_at(200.0, 10.0), None);
    assert_eq!(layout.cell_at(250.0, 10.0), None);

    // Past last row
    assert_eq!(layout.cell_at(10.0, 80.0), None);
    assert_eq!(layout.cell_at(10.0, 90.0), None);

    // Negative coordinates
    assert_eq!(layout.cell_at(-0.1, 10.0), None);
    assert_eq!(layout.cell_at(10.0, -0.1), None);
}

#[test]
fn grid_layout_selectable_at_refuses_fixed_cells() {
    let layout = GridLayout::from_props(4, 4, 1, 1, 50, 20);

    // Fixed column 0
    assert_eq!(layout.selectable_at(10.0, 30.0), None);

    // Fixed row 0
    assert_eq!(layout.selectable_at(60.0, 5.0), None);

    // Both fixed
    assert_eq!(layout.selectable_at(10.0, 5.0), None);

    // Selectable cell (1, 1)
    assert_eq!(layout.selectable_at(60.0, 30.0), Some((1, 1)));

    // Outside bounds
    assert_eq!(layout.selectable_at(250.0, 30.0), None);
}
