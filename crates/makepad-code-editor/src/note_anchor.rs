//! A sidecar note's anchor in the code: `#[note("id")]` (Doc), `[crawrad:note#id]` (Note,
//! including region forms `[crawrad:note:begin#id]` / `[crawrad:note:end#id]`), and
//! `[crawrad:comment#id]` (Comment). Pure text work — the editor draws the icon and the chip from it.

/// The kind of note anchor on a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorKind {
    Doc,
    Note,
    Comment,
}

/// The tag's byte range in `line`, brackets included (for Doc, the whole `#[...]` attribute).
pub fn tag_span(line: &str) -> Option<(usize, usize)> {
    anchor_tag(line).map(|(_, span, _)| span)
}

/// Scans `line` for an anchor tag, returning `(kind, (start, end), id)`.
pub fn anchor_tag(line: &str) -> Option<(AnchorKind, (usize, usize), &str)> {
    if let Some((start, end, id)) = scan_doc_attribute(line) {
        return Some((AnchorKind::Doc, (start, end), id));
    }
    if let Some((start, end, id)) = scan_comment_tag(line) {
        return Some((AnchorKind::Comment, (start, end), id));
    }
    if let Some((start, end, id)) = scan_note_tag(line) {
        return Some((AnchorKind::Note, (start, end), id));
    }
    None
}

/// The anchor kind and note id named on `line`.
pub fn anchor_in(line: &str) -> Option<(AnchorKind, &str)> {
    anchor_tag(line).map(|(kind, _, id)| (kind, id))
}

/// The note id the anchor on `line` names.
pub fn id_in(line: &str) -> Option<&str> {
    anchor_in(line).map(|(_, id)| id)
}

fn scan_comment_tag(line: &str) -> Option<(usize, usize, &str)> {
    let start = line.find("[crawrad:comment#")?;
    let slice = &line[start..];
    let tag_end_rel = slice.find(']')?;
    let end = start + tag_end_rel + 1;
    let tag = &slice[..tag_end_rel];
    let id = tag[tag.find('#')? + 1..].trim();
    if id.is_empty() {
        return None;
    }
    Some((start, end, id))
}

fn scan_note_tag(line: &str) -> Option<(usize, usize, &str)> {
    let start = line.find("[crawrad:note")?;
    let slice = &line[start..];
    let tag_end_rel = slice.find(']')?;
    let end = start + tag_end_rel + 1;
    let tag = &slice[..tag_end_rel];
    if !tag.starts_with("[crawrad:note#")
        && !tag.starts_with("[crawrad:note:begin#")
        && !tag.starts_with("[crawrad:note:end#")
    {
        return None;
    }
    let hash = tag.find('#')?;
    let id = tag[hash + 1..].trim();
    if id.is_empty() {
        return None;
    }
    Some((start, end, id))
}

fn scan_doc_attribute(line: &str) -> Option<(usize, usize, &str)> {
    let mut search_from = 0;
    while let Some(rel) = line[search_from..].find("#[") {
        let start = search_from + rel;
        if start > 0 && line.as_bytes()[start - 1] == b'!' {
            search_from = start + 2;
            continue;
        }
        let rest = &line[start + 2..];
        if let Some(open_paren) = rest.find('(') {
            let path_part = rest[..open_paren].trim();
            let segments: Vec<&str> = path_part.split("::").map(str::trim).collect();
            let is_note = !segments.is_empty()
                && segments.last() == Some(&"note")
                && segments.iter().all(|s| {
                    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                });
            if is_note {
                let after_paren = &rest[open_paren + 1..];
                let trimmed_after = after_paren.trim_start();
                if let Some(after_quote) = trimmed_after.strip_prefix('"')
                    && let Some(quote_end) = after_quote.find('"')
                {
                    let id = &after_quote[..quote_end];
                    let after_closing_quote = after_quote[quote_end + 1..].trim_start();
                    if let Some(after_close_paren) = after_closing_quote.strip_prefix(')') {
                        let trimmed_close = after_close_paren.trim_start();
                        if let Some(bracket_end) = trimmed_close.strip_prefix(']') {
                            let end = line.len() - bracket_end.len();
                            if !id.is_empty() {
                                return Some((start, end, id));
                            }
                        }
                    }
                }
            }
        }
        search_from = start + 2;
    }
    None
}

/// Whether `line` (after trimming) is exactly one doc-note attribute.
pub fn is_link_only_line(line: &str) -> bool {
    let trimmed = line.trim();
    scan_doc_attribute(trimmed).is_some_and(|(start, end, _)| start == 0 && end == trimmed.len())
}

/// The byte range to hide on a code line with a trailing Note/Comment tag;
/// `None` for a link-only line (which collapses instead) and for lines without a tag.
pub fn hidden_span(line: &str) -> Option<(usize, usize)> {
    if is_link_only_line(line) {
        return None;
    }
    let (tag_start, tag_end) = scan_comment_tag(line)
        .map(|(s, e, _)| (s, e))
        .or_else(|| scan_note_tag(line).map(|(s, e, _)| (s, e)))?;

    if line[tag_end..].trim().is_empty()
        && let Some(slash_pos) = line[..tag_start].rfind("//")
        && line[slash_pos + 2..tag_start].trim().is_empty()
    {
        let before_slashes = &line[..slash_pos];
        let hide_start = before_slashes.trim_end().len();
        let hide_end = line.len();
        Some((hide_start, hide_end))
    } else {
        Some((tag_start, tag_end))
    }
}

/// Sorted line indices of link-only lines that have no caret on them.
pub fn collapsed_lines(lines: &[&str], caret_lines: &[usize]) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter(|(idx, line)| is_link_only_line(line) && !caret_lines.contains(idx))
        .map(|(idx, _)| idx)
        .collect()
}

/// The line index that draws the doc icon of `link_line`: the first following
/// line that is not a collapsed link line, an outer attribute `#[...]`, or a `///`
/// doc comment; if none, the next line (`link_line + 1`).
pub fn icon_line(lines: &[&str], link_line: usize) -> usize {
    let next_line = link_line + 1;
    lines
        .iter()
        .enumerate()
        .skip(next_line)
        .find(|(_, line)| {
            !(is_link_only_line(line) || is_outer_attribute(line) || is_doc_comment(line))
        })
        .map_or(next_line, |(i, _)| i)
}

fn is_outer_attribute(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("#[") && trimmed.ends_with(']')
}

fn is_doc_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("///")
}

/// `label` cut to `room` chars with an ellipsis, as the note's chip is.
pub fn fit_label(label: &str, room: usize) -> String {
    if label.chars().count() <= room {
        return label.to_string();
    }
    let cut: String = label.chars().take(room.saturating_sub(1)).collect();
    format!("{}…", cut.trim_end())
}

#[cfg(test)]
#[path = "test/note_anchor/test.rs"]
mod tests;
