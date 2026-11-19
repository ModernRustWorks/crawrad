//! Pure model and diff calculation for inline code review.

use similar::{ChangeTag, DiffTag, TextDiff};

/// The kind of row shown in an inline diff review.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReviewRowKind {
    Same,
    Added,
    Removed,
}

/// One row of the merged text during a diff review.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReviewRow {
    pub kind: ReviewRowKind,
    /// 1-based line number in the proposed text; `None` for removed rows.
    pub number: Option<usize>,
}

/// The view model representing a pending diff review in the editor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewView {
    /// Merged text shown in the editor while reviewing.
    pub text: String,
    /// Per-line metadata for each line in `text`.
    pub rows: Vec<ReviewRow>,
    /// 0-based line indices in `text` of the start of each change.
    pub changes: Vec<usize>,
}

/// Computes the review view comparing original content to proposed content.
pub fn review_view(original: &str, proposed: &str) -> ReviewView {
    let diff = TextDiff::from_lines(original, proposed);
    let mut rows = Vec::new();
    let mut changes = Vec::new();
    let mut line_strings = Vec::new();
    let mut in_change = false;

    for op in diff.ops() {
        let tag = op.tag();
        if tag == DiffTag::Equal {
            in_change = false;
        } else if !in_change {
            changes.push(rows.len());
            in_change = true;
        }

        for change in diff.iter_changes(op) {
            let (kind, number) = match change.tag() {
                ChangeTag::Equal => (ReviewRowKind::Same, change.new_index().map(|i| i + 1)),
                ChangeTag::Delete => (ReviewRowKind::Removed, None),
                ChangeTag::Insert => (ReviewRowKind::Added, change.new_index().map(|i| i + 1)),
            };
            let val = change.value();
            let clean = val
                .strip_suffix("\r\n")
                .or_else(|| val.strip_suffix('\n'))
                .unwrap_or(val);
            line_strings.push(clean);
            rows.push(ReviewRow { kind, number });
        }
    }

    let mut text = line_strings.join("\n");
    if proposed.ends_with('\n') && !text.is_empty() {
        text.push('\n');
    }

    ReviewView {
        text,
        rows,
        changes,
    }
}

impl ReviewView {
    /// How many rows of `kind` the merged text has.
    pub fn count(&self, kind: ReviewRowKind) -> usize {
        self.rows.iter().filter(|r| r.kind == kind).count()
    }

    /// The change after (`forward`) or before `index`, wrapping around.
    pub fn step(&self, index: usize, forward: bool) -> usize {
        let n = self.changes.len().max(1);
        let index = index % n;
        if forward {
            (index + 1) % n
        } else {
            (index + n - 1) % n
        }
    }

    /// `· 1 change` or `· N changes`, for the review bar.
    pub fn changes_label(&self) -> String {
        match self.changes.len() {
            1 => "· 1 change".to_string(),
            n => format!("· {n} changes"),
        }
    }

    /// `k of N` while there are two or more changes to move between.
    pub fn position_label(&self, index: usize) -> Option<String> {
        let n = self.changes.len();
        (n >= 2).then(|| format!("{} of {n}", index % n + 1))
    }
}

#[cfg(test)]
#[path = "test/review/test.rs"]
mod tests;
