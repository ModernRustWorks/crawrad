//! Edits that work on whole lines of the editor's text — pure: lines in,
//! lines out, no window.

#[note("line-edits-toggle-line-comment")]
pub fn toggle_line_comment(lines: &[String], start_line: usize, end_line: usize) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }
    let start = start_line.min(lines.len() - 1);
    let end = end_line.min(lines.len() - 1).max(start);
    let block = &lines[start..=end];
    let has_content = block.iter().any(|l| !l.trim().is_empty());
    let all_commented = has_content
        && block
            .iter()
            .map(|l| l.trim_start())
            .filter(|t| !t.is_empty())
            .all(|t| t.starts_with("//"));
    let indent_of = |l: &str| l.len() - l.trim_start().len();
    let mut out = lines.to_vec();
    if all_commented {
        for l in &mut out[start..=end] {
            let (prefix, rest) = l.split_at(indent_of(l));
            if let Some(after) = rest.strip_prefix("// ").or_else(|| rest.strip_prefix("//")) {
                *l = format!("{prefix}{after}");
            }
        }
    } else {
        let smallest = block
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| indent_of(l))
            .min()
            .unwrap_or(0);
        for l in out[start..=end].iter_mut().filter(|l| !l.trim().is_empty()) {
            let (prefix, rest) = l.split_at(smallest);
            *l = format!("{prefix}// {rest}");
        }
    }
    out
}

#[note("line-edits-move-lines")]
pub fn move_lines(
    lines: &[String],
    start: usize,
    end: usize,
    up: bool,
) -> (Vec<String>, usize, usize) {
    if lines.is_empty() {
        return (Vec::new(), 0, 0);
    }
    let s = start.min(lines.len() - 1);
    let e = end.min(lines.len() - 1).max(s);
    let mut out = lines.to_vec();
    if up && s > 0 {
        let above = out.remove(s - 1);
        out.insert(e, above);
        (out, s - 1, e - 1)
    } else if !up && e + 1 < out.len() {
        let below = out.remove(e + 1);
        out.insert(s, below);
        (out, s + 1, e + 1)
    } else {
        (out, s, e)
    }
}

#[note("line-edits-duplicate-lines")]
pub fn duplicate_lines(
    lines: &[String],
    start: usize,
    end: usize,
    up: bool,
) -> (Vec<String>, usize, usize) {
    if lines.is_empty() {
        return (Vec::new(), 0, 0);
    }
    let s = start.min(lines.len() - 1);
    let e = end.min(lines.len() - 1).max(s);
    let block = lines[s..=e].to_vec();
    let count = block.len();
    let mut out = lines.to_vec();
    let at = if up { s } else { e + 1 };
    out.splice(at..at, block);
    if up {
        (out, s, e)
    } else {
        (out, s + count, e + count)
    }
}

#[note("line-edits-delete-lines")]
pub fn delete_lines(lines: &[String], start: usize, end: usize) -> (Vec<String>, usize) {
    if lines.is_empty() {
        return (vec![String::new()], 0);
    }
    let s = start.min(lines.len() - 1);
    let e = end.min(lines.len() - 1).max(s);
    let mut out = lines.to_vec();
    out.drain(s..=e);
    if out.is_empty() {
        out.push(String::new());
    }
    let caret = s.min(out.len() - 1);
    (out, caret)
}

#[note("line-edits-next-occurrence")]
pub fn next_occurrence(
    lines: &[String],
    term: &str,
    after: (usize, usize),
    before: (usize, usize),
) -> Option<(usize, usize)> {
    if lines.is_empty() || term.is_empty() || after.0 >= lines.len() {
        return None;
    }
    // The first char boundary at or past `byte` in `line`.
    let boundary = |line: &str, byte: usize| {
        let byte = byte.min(line.len());
        (byte..=line.len())
            .find(|&b| line.is_char_boundary(b))
            .unwrap_or(line.len())
    };
    for (l, line) in lines.iter().enumerate().skip(after.0) {
        let from = if l == after.0 {
            boundary(line, after.1)
        } else {
            0
        };
        if let Some(at) = line[from..].find(term) {
            return Some((l, from + at));
        }
    }
    let last = before.0.min(lines.len() - 1);
    for (l, line) in lines.iter().enumerate().take(last + 1) {
        let to = if l == last {
            boundary(line, before.1)
        } else {
            line.len()
        };
        if let Some(at) = line[..to].find(term) {
            return Some((l, at));
        }
    }
    None
}

use crate::editor_chords::{Command, LineDirection};
use crawrad_note::note;

#[note("line-edits-selection-line-range")]
pub fn selection_line_range(start_line: usize, end_line: usize, end_byte: usize) -> (usize, usize) {
    let end = if end_line > start_line && end_byte == 0 {
        end_line.saturating_sub(1)
    } else {
        end_line
    };
    (start_line, end)
}

#[note("line-edits-apply-line-edit")]
pub fn apply_line_edit(
    command: Command,
    lines: &[String],
    start_line: usize,
    end_line: usize,
) -> Option<(Vec<String>, usize)> {
    match command {
        Command::ToggleComment => {
            let res = toggle_line_comment(lines, start_line, end_line);
            Some((res, start_line))
        }
        Command::MoveLines(dir) => {
            let (res, new_s, _) = move_lines(lines, start_line, end_line, dir == LineDirection::Up);
            Some((res, new_s))
        }
        Command::DuplicateLines(dir) => {
            let (res, new_s, _) =
                duplicate_lines(lines, start_line, end_line, dir == LineDirection::Up);
            Some((res, new_s))
        }
        Command::DeleteLine => {
            let (res, new_pos) = delete_lines(lines, start_line, end_line);
            Some((res, new_pos))
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/line_edits/test.rs"]
mod tests;
