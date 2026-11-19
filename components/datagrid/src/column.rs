//! Column storage variants and dynamic cell values.

use crate::bitmap::Bitmap;

/// A borrowed scalar value from a column cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<'a> {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(&'a str),
}

/// In-memory columnar storage representation.
#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Int {
        values: Vec<i64>,
        valid: Option<Bitmap>,
    },
    Float {
        values: Vec<f64>,
        valid: Option<Bitmap>,
    },
    Bool {
        values: Bitmap,
        valid: Option<Bitmap>,
    },
    /// Plain text: one byte buffer + offsets (`offsets.len() == rows + 1`).
    Text {
        bytes: Vec<u8>,
        offsets: Vec<u32>,
        valid: Option<Bitmap>,
    },
    /// Dictionary text: `codes[row]` indexes `dict`; `dict` is kept SORTED,
    /// so code order == text order (sort and group compare integers).
    Dict {
        codes: Vec<u32>,
        dict: Vec<String>,
        valid: Option<Bitmap>,
    },
}

impl Column {
    /// Returns the number of rows in the column.
    pub fn len(&self) -> usize {
        match self {
            Column::Int { values, .. } => values.len(),
            Column::Float { values, .. } => values.len(),
            Column::Bool { values, .. } => values.len(),
            Column::Text { offsets, .. } => {
                if offsets.is_empty() {
                    0
                } else {
                    offsets.len() - 1
                }
            }
            Column::Dict { codes, .. } => codes.len(),
        }
    }

    /// Returns `true` if the column contains zero rows.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the value at row `row` is null.
    #[inline]
    pub fn is_null(&self, row: usize) -> bool {
        match self {
            Column::Int { valid, .. }
            | Column::Float { valid, .. }
            | Column::Bool { valid, .. }
            | Column::Text { valid, .. }
            | Column::Dict { valid, .. } => {
                if let Some(v) = valid {
                    !v.get(row)
                } else {
                    false
                }
            }
        }
    }

    /// Returns the borrowed cell value at row `row`.
    #[inline]
    pub fn get(&self, row: usize) -> Value<'_> {
        assert!(
            row < self.len(),
            "row index out of bounds: {} >= {}",
            row,
            self.len()
        );
        if self.is_null(row) {
            return Value::Null;
        }
        match self {
            Column::Int { values, .. } => Value::Int(values[row]),
            Column::Float { values, .. } => Value::Float(values[row]),
            Column::Bool { values, .. } => Value::Bool(values.get(row)),
            Column::Text { bytes, offsets, .. } => {
                let start = offsets[row] as usize;
                let end = offsets[row + 1] as usize;
                let slice = &bytes[start..end];
                let s = std::str::from_utf8(slice).expect("valid utf-8 in text column");
                Value::Text(s)
            }
            Column::Dict { codes, dict, .. } => {
                let code = codes[row] as usize;
                Value::Text(&dict[code])
            }
        }
    }

    /// Returns the validity mask if any nulls exist in this column.
    #[inline]
    pub fn validity(&self) -> Option<&Bitmap> {
        match self {
            Column::Int { valid, .. }
            | Column::Float { valid, .. }
            | Column::Bool { valid, .. }
            | Column::Text { valid, .. }
            | Column::Dict { valid, .. } => valid.as_ref(),
        }
    }

    /// Constructs an `Int` column from optional values.
    pub fn ints(items: Vec<Option<i64>>) -> Self {
        let len = items.len();
        let has_null = items.iter().any(|v| v.is_none());
        let mut values = Vec::with_capacity(len);

        if has_null {
            let mut valid = Bitmap::new(len, true);
            for (i, item) in items.into_iter().enumerate() {
                match item {
                    Some(v) => values.push(v),
                    None => {
                        valid.set(i, false);
                        values.push(0);
                    }
                }
            }
            Column::Int {
                values,
                valid: Some(valid),
            }
        } else {
            for item in items {
                values.push(item.expect("null checked"));
            }
            Column::Int {
                values,
                valid: None,
            }
        }
    }

    /// Constructs a `Float` column from optional values.
    pub fn floats(items: Vec<Option<f64>>) -> Self {
        let len = items.len();
        let has_null = items.iter().any(|v| v.is_none());
        let mut values = Vec::with_capacity(len);

        if has_null {
            let mut valid = Bitmap::new(len, true);
            for (i, item) in items.into_iter().enumerate() {
                match item {
                    Some(v) => values.push(v),
                    None => {
                        valid.set(i, false);
                        values.push(0.0);
                    }
                }
            }
            Column::Float {
                values,
                valid: Some(valid),
            }
        } else {
            for item in items {
                values.push(item.expect("null checked"));
            }
            Column::Float {
                values,
                valid: None,
            }
        }
    }

    /// Constructs a `Bool` column from optional values.
    pub fn bools(items: Vec<Option<bool>>) -> Self {
        let len = items.len();
        let has_null = items.iter().any(|v| v.is_none());
        let mut values = Bitmap::new(len, false);

        if has_null {
            let mut valid = Bitmap::new(len, true);
            for (i, item) in items.into_iter().enumerate() {
                match item {
                    Some(b) => {
                        if b {
                            values.set(i, true);
                        }
                    }
                    None => {
                        valid.set(i, false);
                    }
                }
            }
            Column::Bool {
                values,
                valid: Some(valid),
            }
        } else {
            for (i, item) in items.into_iter().enumerate() {
                if item.expect("null checked") {
                    values.set(i, true);
                }
            }
            Column::Bool {
                values,
                valid: None,
            }
        }
    }

    /// Constructs a plain `Text` column from optional values.
    pub fn texts(items: Vec<Option<String>>) -> Self {
        let len = items.len();
        let has_null = items.iter().any(|v| v.is_none());
        let mut bytes = Vec::new();
        let mut offsets = Vec::with_capacity(len + 1);
        offsets.push(0);

        if has_null {
            let mut valid = Bitmap::new(len, true);
            for (i, item) in items.into_iter().enumerate() {
                match item {
                    Some(s) => {
                        bytes.extend_from_slice(s.as_bytes());
                        offsets.push(bytes.len() as u32);
                    }
                    None => {
                        valid.set(i, false);
                        offsets.push(bytes.len() as u32);
                    }
                }
            }
            Column::Text {
                bytes,
                offsets,
                valid: Some(valid),
            }
        } else {
            for item in items {
                let s = item.expect("null checked");
                bytes.extend_from_slice(s.as_bytes());
                offsets.push(bytes.len() as u32);
            }
            Column::Text {
                bytes,
                offsets,
                valid: None,
            }
        }
    }

    /// Constructs a dictionary-encoded `Dict` column with a sorted dictionary.
    pub fn dict_from(items: Vec<Option<String>>) -> Self {
        let len = items.len();
        let mut unique_refs: Vec<&str> = items.iter().filter_map(|s| s.as_deref()).collect();
        unique_refs.sort_unstable();
        unique_refs.dedup();
        let mut unique: Vec<String> = unique_refs.iter().map(|&s| s.to_string()).collect();
        unique.shrink_to_fit();

        let has_null = items.iter().any(|v| v.is_none());
        let mut codes = Vec::with_capacity(len);

        if has_null {
            let mut valid = Bitmap::new(len, true);
            for (i, item) in items.into_iter().enumerate() {
                match item {
                    Some(s) => {
                        let code = unique
                            .binary_search(&s)
                            .expect("item present in dictionary")
                            as u32;
                        codes.push(code);
                    }
                    None => {
                        valid.set(i, false);
                        codes.push(0);
                    }
                }
            }
            Column::Dict {
                codes,
                dict: unique,
                valid: Some(valid),
            }
        } else {
            for item in items {
                let s = item.expect("null checked");
                let code = unique
                    .binary_search(&s)
                    .expect("item present in dictionary") as u32;
                codes.push(code);
            }
            Column::Dict {
                codes,
                dict: unique,
                valid: None,
            }
        }
    }

    /// Chooses between `Dict` and `Text` based on distinct string ratio.
    /// Picks `Dict` when distinct values <= rows / 4, else plain `Text`.
    pub fn texts_auto(items: Vec<Option<String>>) -> Self {
        let rows = items.len();
        let mut unique: Vec<&str> = items.iter().filter_map(|s| s.as_deref()).collect();
        unique.sort_unstable();
        unique.dedup();
        let distinct = unique.len();

        if distinct <= rows / 4 {
            Column::dict_from(items)
        } else {
            Column::texts(items)
        }
    }

    /// Extracts a new column containing only the cells at indices `rows`.
    ///
    /// Preserves the storage kind, dictionary tables, and validity bitmaps.
    /// Allocates contiguous buffers once per column rather than per cell.
    pub fn gather(&self, rows: &[u32]) -> Column {
        let valid = self.validity().map(|v| {
            let mut bm = Bitmap::new(rows.len(), true);
            for (i, &r) in rows.iter().enumerate() {
                if !v.get(r as usize) {
                    bm.set(i, false);
                }
            }
            bm
        });

        match self {
            Column::Int { values, .. } => {
                let mut new_values = Vec::with_capacity(rows.len());
                for &r in rows {
                    new_values.push(values[r as usize]);
                }
                Column::Int {
                    values: new_values,
                    valid,
                }
            }
            Column::Float { values, .. } => {
                let mut new_values = Vec::with_capacity(rows.len());
                for &r in rows {
                    new_values.push(values[r as usize]);
                }
                Column::Float {
                    values: new_values,
                    valid,
                }
            }
            Column::Bool { values, .. } => {
                let mut new_values = Bitmap::new(rows.len(), false);
                for (i, &r) in rows.iter().enumerate() {
                    if values.get(r as usize) {
                        new_values.set(i, true);
                    }
                }
                Column::Bool {
                    values: new_values,
                    valid,
                }
            }
            Column::Text { bytes, offsets, .. } => {
                let mut total_bytes = 0usize;
                for &r in rows {
                    let r = r as usize;
                    let start = offsets[r] as usize;
                    let end = offsets[r + 1] as usize;
                    total_bytes += end - start;
                }
                let mut new_bytes = Vec::with_capacity(total_bytes);
                let mut new_offsets = Vec::with_capacity(rows.len() + 1);
                new_offsets.push(0);
                for &r in rows {
                    let r = r as usize;
                    let start = offsets[r] as usize;
                    let end = offsets[r + 1] as usize;
                    new_bytes.extend_from_slice(&bytes[start..end]);
                    new_offsets.push(new_bytes.len() as u32);
                }
                Column::Text {
                    bytes: new_bytes,
                    offsets: new_offsets,
                    valid,
                }
            }
            Column::Dict { codes, dict, .. } => {
                // Only the entries the window uses are copied (the full dictionary can
                // hold one string per row); old codes are sorted, so the remap keeps
                // the dictionary sorted and code order == text order.
                let mut used: Vec<u32> = rows.iter().map(|&r| codes[r as usize]).collect();
                used.sort_unstable();
                used.dedup();
                let new_codes = rows
                    .iter()
                    .map(|&r| {
                        used.binary_search(&codes[r as usize])
                            .expect("code collected from the same rows")
                            as u32
                    })
                    .collect();
                let new_dict = used.iter().map(|&c| dict[c as usize].clone()).collect();
                Column::Dict {
                    codes: new_codes,
                    dict: new_dict,
                    valid,
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "test/column/test.rs"]
mod test;
