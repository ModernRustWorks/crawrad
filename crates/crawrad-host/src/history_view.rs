//! Pure model and diff algorithms for the local history view.
//!
//! Provides side-by-side line diffing, change block navigation, single block
//! restoration, and deterministic metadata rows for history entries.

use std::ops::Range;

use similar::{ChangeTag, TextDiff};

use crate::history::Entry;
use crawrad_note::note;

#[note("history-view-rowkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowKind {
    /// Line identical on both sides.
    Same,
    /// Line exists only in the copy (removed from current buffer).
    Removed,
    /// Line exists only in the current buffer (added since copy).
    Added,
    /// Line exists on both sides but has changed content.
    Changed,
}

#[note("history-view-diffrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffRow {
    /// What kind of difference or match this row represents.
    pub kind: RowKind,
    /// The 1-based line number and content on the copy (left) side, if present.
    pub left: Option<(usize, String)>,
    /// The 1-based line number and content on the buffer (right) side, if present.
    pub right: Option<(usize, String)>,
}

#[note("history-view-sidebyside")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideBySide {
    /// The aligned side-by-side rows.
    pub rows: Vec<DiffRow>,
    /// Row ranges of consecutive non-Same rows.
    pub blocks: Vec<Range<usize>>,
}

#[note("history-view-side-by-side")]
pub fn side_by_side(copy: &str, buffer: &str) -> SideBySide {
    let clean_copy = normalize_line_endings(copy);
    let clean_buffer = normalize_line_endings(buffer);

    let diff = TextDiff::from_lines(&clean_copy, &clean_buffer);

    let mut rows = Vec::new();
    let mut blocks = Vec::new();

    let mut pending_removed: Vec<(usize, String)> = Vec::new();
    let mut pending_added: Vec<(usize, String)> = Vec::new();

    let mut left_line: usize = 1;
    let mut right_line: usize = 1;

    let flush_group = |rows: &mut Vec<DiffRow>,
                       blocks: &mut Vec<Range<usize>>,
                       removed: &mut Vec<(usize, String)>,
                       added: &mut Vec<(usize, String)>| {
        if removed.is_empty() && added.is_empty() {
            return;
        }
        let start_row = rows.len();
        let common = usize::min(removed.len(), added.len());

        for (rem, add) in removed.iter().take(common).zip(added.iter().take(common)) {
            rows.push(DiffRow {
                kind: RowKind::Changed,
                left: Some(rem.clone()),
                right: Some(add.clone()),
            });
        }
        for rem in removed.iter().skip(common) {
            rows.push(DiffRow {
                kind: RowKind::Removed,
                left: Some(rem.clone()),
                right: None,
            });
        }
        for add in added.iter().skip(common) {
            rows.push(DiffRow {
                kind: RowKind::Added,
                left: None,
                right: Some(add.clone()),
            });
        }

        let end_row = rows.len();
        if start_row < end_row {
            blocks.push(start_row..end_row);
        }
        removed.clear();
        added.clear();
    };

    for change in diff.iter_all_changes() {
        let text = strip_newline(change.value()).to_string();
        match change.tag() {
            ChangeTag::Equal => {
                flush_group(
                    &mut rows,
                    &mut blocks,
                    &mut pending_removed,
                    &mut pending_added,
                );
                rows.push(DiffRow {
                    kind: RowKind::Same,
                    left: Some((left_line, text.clone())),
                    right: Some((right_line, text)),
                });
                left_line += 1;
                right_line += 1;
            }
            ChangeTag::Delete => {
                pending_removed.push((left_line, text));
                left_line += 1;
            }
            ChangeTag::Insert => {
                pending_added.push((right_line, text));
                right_line += 1;
            }
        }
    }

    flush_group(
        &mut rows,
        &mut blocks,
        &mut pending_removed,
        &mut pending_added,
    );

    SideBySide { rows, blocks }
}

#[note("history-view-restore-block")]
pub fn restore_block(copy: &str, buffer: &str, block: usize) -> Option<String> {
    let view = side_by_side(copy, buffer);
    let target_range = view.blocks.get(block)?;

    let mut restored_lines: Vec<&str> = Vec::new();
    for (idx, row) in view.rows.iter().enumerate() {
        if target_range.contains(&idx) {
            if let Some((_, text)) = &row.left {
                restored_lines.push(text.as_str());
            }
        } else if let Some((_, text)) = &row.right {
            restored_lines.push(text.as_str());
        }
    }

    let uses_crlf = buffer.contains("\r\n");
    let eol = if uses_crlf { "\r\n" } else { "\n" };
    let trailing_newline = buffer.ends_with('\n');

    if restored_lines.is_empty() {
        return Some(String::new());
    }

    let mut result = restored_lines.join(eol);
    if trailing_newline {
        result.push_str(eol);
    }
    Some(result)
}

#[note("history-view-next-block")]
pub fn next_block(view: &SideBySide, current: Option<usize>) -> Option<usize> {
    let len = view.blocks.len();
    if len == 0 {
        return None;
    }
    match current {
        None => Some(0),
        Some(c) => Some((c + 1) % len),
    }
}

#[note("history-view-previous-block")]
pub fn previous_block(view: &SideBySide, current: Option<usize>) -> Option<usize> {
    let len = view.blocks.len();
    if len == 0 {
        return None;
    }
    match current {
        None => Some(len - 1),
        Some(0) => Some(len - 1),
        Some(c) if c >= len => Some(len - 1),
        Some(c) => Some(c - 1),
    }
}

#[note("history-view-entryrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryRow {
    /// Formatted local time: `"HH:MM:SS"`.
    pub time: String,
    /// Relative calendar descriptor: `"today"`, `"yesterday"`, `"dd/mm"`, or `"dd/mm/yyyy"`.
    pub when: String,
    /// Origin label: `"Save"` or `"External"`.
    pub origin: &'static str,
    /// Human-readable byte size: `"812 B"`, `"1.8 KB"`, or `"2.4 MB"`.
    pub size: String,
    /// Line count of the content.
    pub lines: usize,
}

#[note("history-view-when-and-time")]
pub fn when_and_time(time: u64, now: u64, offset: i64) -> (String, String) {
    let now_local_secs = (now as i64) + offset;
    let now_days = now_local_secs.div_euclid(86_400);
    let (now_year, _, _) = crawrad_os::clock::civil_from_days(now_days);

    let entry_local_secs = (time as i64) + offset;
    let sec_of_day = entry_local_secs.rem_euclid(86_400);
    let hour = sec_of_day / 3600;
    let minute = (sec_of_day % 3600) / 60;
    let second = sec_of_day % 60;
    let time_str = format!("{hour:02}:{minute:02}:{second:02}");

    let entry_days = entry_local_secs.div_euclid(86_400);
    let (entry_year, entry_month, entry_day) = crawrad_os::clock::civil_from_days(entry_days);

    let when = if entry_days == now_days {
        "today".to_string()
    } else if entry_days == now_days - 1 {
        "yesterday".to_string()
    } else if entry_year == now_year {
        format!("{entry_day:02}/{entry_month:02}")
    } else {
        format!("{entry_day:02}/{entry_month:02}/{entry_year}")
    };

    (time_str, when)
}

#[note("history-view-entry-rows")]
pub fn entry_rows(entries: &[(Entry, String)], now: u64, offset: i64) -> Vec<EntryRow> {
    let mut indexed: Vec<(usize, &(Entry, String))> = entries.iter().enumerate().collect();
    indexed.sort_by(|(idx_a, (entry_a, _)), (idx_b, (entry_b, _))| {
        entry_b
            .time
            .cmp(&entry_a.time)
            .then_with(|| idx_b.cmp(idx_a))
    });

    let mut rows = Vec::with_capacity(indexed.len());
    for (_, (entry, content)) in indexed {
        let (time, when) = when_and_time(entry.time, now, offset);

        let origin = match entry.origin {
            crate::history::Origin::Save => "Save",
            crate::history::Origin::External => "External",
        };

        let size = format_size(content.len());
        let lines = content.lines().count();

        rows.push(EntryRow {
            time,
            when,
            origin,
            size,
            lines,
        });
    }

    rows
}

#[note("history-view-timeline-lines")]
pub fn timeline_lines(entries: &[Entry], now: u64, offset: i64) -> Vec<String> {
    let mut indexed: Vec<(usize, &Entry)> = entries.iter().enumerate().collect();
    indexed.sort_by(|(idx_a, entry_a), (idx_b, entry_b)| {
        entry_b
            .time
            .cmp(&entry_a.time)
            .then_with(|| idx_b.cmp(idx_a))
    });

    let mut lines = Vec::with_capacity(indexed.len());
    for (_, entry) in indexed {
        let (time, when) = when_and_time(entry.time, now, offset);
        let origin = match entry.origin {
            crate::history::Origin::Save => "Save",
            crate::history::Origin::External => "External",
        };
        lines.push(format!("{origin} · {time} · {when}"));
    }
    lines
}

#[note("history-view-historysession")]
#[derive(Debug, Clone, Default)]
pub struct HistorySession {
    /// The file whose copies are shown.
    pub file: std::path::PathBuf,
    /// Oldest first, as the engine keeps them.
    copies: Vec<(Entry, String)>,
    /// The copies as the list shows them, newest first.
    pub rows: Vec<EntryRow>,
    /// The row compared with the buffer.
    pub selected: Option<usize>,
    /// The change block in focus.
    pub block: Option<usize>,
}

impl HistorySession {
    #[note("history-view-historysession-open")]
    pub fn open(
        history: Option<&crate::history::LocalHistory>,
        file: std::path::PathBuf,
        now: u64,
        offset: i64,
    ) -> Self {
        let copies: Vec<(Entry, String)> = history
            .map(|h| {
                let with_text = |e: Entry| h.content(&file, &e).map(|text| (e, text));
                h.entries(&file).into_iter().filter_map(with_text).collect()
            })
            .unwrap_or_default();
        let rows = entry_rows(&copies, now, offset);
        Self {
            file,
            selected: (!copies.is_empty()).then_some(0),
            copies,
            rows,
            block: None,
        }
    }

    #[note("history-view-historysession-copy")]
    pub fn copy(&self) -> &str {
        let index = |row: usize| self.copies.len().checked_sub(row + 1);
        self.selected
            .and_then(index)
            .and_then(|i| self.copies.get(i))
            .map_or("", |(_, text)| text)
    }

    #[note("history-view-historysession-diff")]
    pub fn diff(&self, buffer: &str) -> SideBySide {
        side_by_side(self.copy(), buffer)
    }

    #[note("history-view-historysession-select")]
    pub fn select(&mut self, row: usize) {
        if row < self.copies.len() {
            self.selected = Some(row);
            self.block = None;
        }
    }

    #[note("history-view-historysession-step")]
    pub fn step(&mut self, buffer: &str, forward: bool) -> Option<usize> {
        let view = self.diff(buffer);
        self.block = if forward {
            next_block(&view, self.block)
        } else {
            previous_block(&view, self.block)
        };
        Some(view.blocks.get(self.block?)?.start)
    }

    #[note("history-view-historysession-restored")]
    pub fn restored(&self, buffer: &str) -> Option<String> {
        restore_block(self.copy(), buffer, self.block.unwrap_or(0))
    }

    #[note("history-view-historysession-json")]
    pub fn json(&self, buffer: &str) -> serde_json::Value {
        let entries: Vec<_> = self
            .rows
            .iter()
            .map(|r| {
                serde_json::json!({ "time": r.time, "when": r.when, "origin": r.origin,
                    "size": r.size, "lines": r.lines })
            })
            .collect();
        serde_json::json!({
            "file": self.file.display().to_string(),
            "entries": entries,
            "selected": self.selected,
            "blocks": self.diff(buffer).blocks.len(),
            "block": self.block,
        })
    }
}

fn format_size(bytes: usize) -> String {
    const KIB: usize = 1024;
    const MIB: usize = 1024 * 1024;

    if bytes < KIB {
        format!("{bytes} B")
    } else if bytes < MIB {
        let kb = bytes as f64 / KIB as f64;
        format!("{kb:.1} KB")
    } else {
        let mb = bytes as f64 / MIB as f64;
        format!("{mb:.1} MB")
    }
}

fn normalize_line_endings(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 1);
    for line in text.lines() {
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn strip_newline(s: &str) -> &str {
    s.trim_end_matches(['\r', '\n'])
}

#[cfg(test)]
#[path = "test/history_view/test.rs"]
mod tests;
