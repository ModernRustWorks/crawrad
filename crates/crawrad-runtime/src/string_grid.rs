//! Pure headless data and geometry models for an unbound string grid.

use crawrad_note::note;
use std::collections::BTreeMap;

#[note("string-grid-cells")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cells {
    data: BTreeMap<(usize, usize), String>,
}

impl Cells {
    #[note("string-grid-cells-from-text")]
    pub fn from_text(text: &str) -> Self {
        let mut cells = Self::default();
        for (row, line) in text.split('\n').enumerate() {
            let line = line.strip_suffix('\r').unwrap_or(line);
            for (col, piece) in line.split('\t').enumerate() {
                if !piece.is_empty() {
                    cells.set(col, row, piece);
                }
            }
        }
        cells
    }

    #[note("string-grid-cells-get")]
    pub fn get(&self, col: usize, row: usize) -> &str {
        self.data.get(&(col, row)).map_or("", String::as_str)
    }

    #[note("string-grid-cells-set")]
    pub fn set(&mut self, col: usize, row: usize, text: impl Into<String>) {
        let text = text.into();
        if text.is_empty() {
            self.data.remove(&(col, row));
        } else {
            self.data.insert((col, row), text);
        }
    }
}

#[note("string-grid-gridlayout")]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GridLayout {
    pub col_count: usize,
    pub row_count: usize,
    pub fixed_cols: usize,
    pub fixed_rows: usize,
    pub col_width: f64,
    pub row_height: f64,
}

impl GridLayout {
    #[note("string-grid-gridlayout-from-props")]
    pub fn from_props(
        col_count: i64,
        row_count: i64,
        fixed_cols: i64,
        fixed_rows: i64,
        col_width: i64,
        row_height: i64,
    ) -> Self {
        Self {
            col_count: usize::try_from(col_count.max(0)).unwrap_or(0),
            row_count: usize::try_from(row_count.max(0)).unwrap_or(0),
            fixed_cols: usize::try_from(fixed_cols.max(0)).unwrap_or(0),
            fixed_rows: usize::try_from(fixed_rows.max(0)).unwrap_or(0),
            col_width: col_width.max(1) as f64,
            row_height: row_height.max(1) as f64,
        }
    }

    #[note("string-grid-gridlayout-is-fixed")]
    pub fn is_fixed(&self, col: usize, row: usize) -> bool {
        col < self.fixed_cols || row < self.fixed_rows
    }

    #[note("string-grid-gridlayout-cell-rect")]
    pub fn cell_rect(&self, col: usize, row: usize) -> (f64, f64, f64, f64) {
        (
            col as f64 * self.col_width,
            row as f64 * self.row_height,
            self.col_width,
            self.row_height,
        )
    }

    #[note("string-grid-gridlayout-cell-at")]
    pub fn cell_at(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let col = (x / self.col_width) as usize;
        let row = (y / self.row_height) as usize;
        (col < self.col_count && row < self.row_count).then_some((col, row))
    }

    #[note("string-grid-gridlayout-selectable-at")]
    pub fn selectable_at(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        self.cell_at(x, y)
            .filter(|&(col, row)| !self.is_fixed(col, row))
    }
}

#[cfg(test)]
#[path = "test/string_grid/test.rs"]
mod tests;
