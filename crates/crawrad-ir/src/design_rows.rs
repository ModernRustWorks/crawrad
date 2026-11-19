//! The first rows of a bound `Grid`'s table, as the Designer shows them.
//!
//! Only the vocabulary lives here: the IDE's host reads the rows (through
//! `crawrad-dbprobe`, off the UI thread) and the canvas draws them. Nothing of
//! this is ever written into the form — the app reads its own data at run time.

use crawrad_note::note;

#[note("design-rows-key")]
pub const DESIGN_ROWS_KEY: &str = "design_rows";

#[note("design-rows-default-limit")]
pub const DEFAULT_LIMIT: usize = 8;

#[note("design-rows-choices")]
pub const CHOICES: [&str; 4] = ["Off", "8", "20", "50"];

#[note("design-rows-sample")]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sample {
    /// The columns, in the table's order.
    pub columns: Vec<String>,
    /// The declared data types for each column ("INTEGER", "REAL", etc.).
    pub kinds: Vec<String>,
    /// The first rows, each cell already turned into text (None for NULL).
    pub rows: Vec<Vec<Option<String>>>,
    /// How many rows the whole table has.
    pub total: u64,
}

#[note("design-rows-gridrows")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridRows {
    /// The probe is reading; `columns` are from the last successful read.
    Reading {
        columns: Vec<String>,
        limit: usize,
        table: String,
    },
    /// The rows are here. `file` is the database's file name, when it has one.
    Ready {
        sample: Sample,
        table: String,
        file: Option<String>,
    },
    /// No rows: `reason` is the probe's own words, `columns` the last read.
    Offline {
        columns: Vec<String>,
        reason: String,
        table: String,
    },
}

impl GridRows {
    #[note("design-rows-gridrows-columns")]
    pub fn columns(&self) -> &[String] {
        match self {
            Self::Reading { columns, .. } | Self::Offline { columns, .. } => columns,
            Self::Ready { sample, .. } => &sample.columns,
        }
    }

    #[note("design-rows-gridrows-kinds")]
    pub fn kinds(&self) -> &[String] {
        match self {
            Self::Ready { sample, .. } => &sample.kinds,
            _ => &[],
        }
    }

    #[note("design-rows-gridrows-footer")]
    pub fn footer(&self) -> String {
        match self {
            Self::Reading { limit, .. } => format!("reading {limit} rows\u{2026}"),
            Self::Ready {
                sample,
                table,
                file,
            } => {
                let source = match file {
                    Some(f) => format!("{table} ({f})"),
                    None => table.clone(),
                };
                format!(
                    "design-time rows \u{b7} {} of {} \u{b7} {source}",
                    sample.rows.len(),
                    thousands(sample.total)
                )
            }
            Self::Offline { table, .. } => {
                format!("design-time rows \u{b7} offline \u{b7} {table}")
            }
        }
    }
}

#[note("design-rows-limit-of")]
pub fn limit_of(value: Option<&str>) -> Option<usize> {
    match value.map(str::trim) {
        None | Some("") => Some(DEFAULT_LIMIT),
        Some(v) if v.eq_ignore_ascii_case("off") => None,
        Some(v) => Some(
            v.parse::<usize>()
                .ok()
                .filter(|n| *n > 0)
                .unwrap_or(DEFAULT_LIMIT),
        ),
    }
}

#[note("design-rows-thousands")]
pub fn thousands(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[note("design-rows-gridbinding")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridBinding<'a> {
    /// The node that holds the database `url`.
    pub connection: &'a crate::Node,
    /// The dataset in between, when the Grid reads its `sql` instead of a table.
    pub dataset: Option<&'a crate::Node>,
}

/// Where a `Grid`'s `data_source` leads (Provider -> Connection -> Dataset -> Grid).
///
/// A referenced node that itself points onward through `connection` is a
/// dataset; any other is the connection. Read from the props, never from a
/// class name. `None` when unbound or when a link is missing.
#[note("design-rows-grid-binding")]
pub fn grid_binding<'a>(doc: &'a crate::Document, grid: &crate::Node) -> Option<GridBinding<'a>> {
    let reference = |node: &crate::Node, key: &str| match node.props.get(key) {
        Some(crate::Value::Ref(id)) => Some(id.clone()),
        _ => None,
    };
    let target = doc.node(&reference(grid, "data_source").filter(|id| !id.is_empty())?)?;
    match reference(target, "connection") {
        Some(id) => Some(GridBinding {
            connection: doc.node(&id).filter(|_| !id.is_empty())?,
            dataset: Some(target),
        }),
        None => Some(GridBinding {
            connection: target,
            dataset: None,
        }),
    }
}

#[cfg(test)]
#[path = "test/design_rows/test.rs"]
mod tests;
