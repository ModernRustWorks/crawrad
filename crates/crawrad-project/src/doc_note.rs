//! The headless model behind the Documentation note.
//!
//! Provides text-based scanning and analysis for Rust documentation notes,
//! mapping cursor positions to documentation targets, suggesting sections,
//! computing kebab-case note IDs, and updating source code on save.

use std::path::{Path, PathBuf};

use crate::sidecar_notes::{NoteKind, SidecarNote};

#[note("doc-note-itemkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Fn,
    Struct,
    Enum,
    Union,
    Trait,
    Const,
    Static,
    TypeAlias,
    Mod,
}

#[note("doc-note-doctarget")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocTarget {
    pub kind: ItemKind,
    pub name: String,
    pub owner: Option<String>,
    pub signature: String,
    pub item_line: usize,
    pub insert_line: usize,
    pub indent: String,
    pub doc_lines: std::ops::Range<usize>,
    pub existing_docs: String,
    pub existing_note: Option<String>,
    pub body: Option<std::ops::Range<usize>>,
    pub is_unsafe: bool,
    pub returns_result: bool,
}

use crawrad_note::note;
pub use crawrad_note_format::rustdoc::{
    ItemFacts, Section, Suggestion, earliest_panic_construct, extract_summary_paragraph,
    first_panic_construct, insert_section, missing_sections, suggestions_for, summary_warning,
};

#[note("doc-note-doc-target-at")]
pub fn doc_target_at(source: &str, cursor_line: usize) -> Option<DocTarget> {
    let lines: Vec<&str> = source.lines().collect();
    if cursor_line >= lines.len() {
        return None;
    }

    let cursor_text = lines[cursor_line].trim();
    if cursor_text.is_empty() {
        return None;
    }

    // A line that is a plain // comment (not /// doc comment) documents nothing.
    let cursor_trimmed_start = lines[cursor_line].trim_start();
    if cursor_trimmed_start.starts_with("//") && !cursor_trimmed_start.starts_with("///") {
        return None;
    }

    let items = scan_items(&lines);

    for target in items {
        let (insert_line, effective_sig_end) = item_documentable_lines(&lines, &target);
        if cursor_line >= insert_line && cursor_line <= effective_sig_end {
            return Some(target);
        }
    }

    None
}

#[note("doc-note-item-documentable-lines")]
fn item_documentable_lines(lines: &[&str], target: &DocTarget) -> (usize, usize) {
    let insert_line = target.insert_line;
    let sig_end_line = find_sig_end_line(lines, target.item_line);

    // If the sig end line has only whitespace before `{`, that line is the start of the body.
    let effective_sig_end = if has_sig_content_before_terminator(lines, sig_end_line) {
        sig_end_line
    } else {
        sig_end_line.saturating_sub(1)
    };

    (insert_line, effective_sig_end)
}

#[note("doc-note-has-sig-content-before-terminator")]
fn has_sig_content_before_terminator(lines: &[&str], line_idx: usize) -> bool {
    if line_idx >= lines.len() {
        return false;
    }
    let line = lines[line_idx];
    let stripped = strip_line_comment(line);
    let term_pos = terminator_pos(stripped, &mut 0);
    match term_pos {
        Some(pos) => !stripped[..pos].trim().is_empty(),
        None => true,
    }
}

#[note("doc-note-find-sig-end-line")]
fn find_sig_end_line(lines: &[&str], item_line: usize) -> usize {
    let mut depth = 0;
    for (idx, line) in lines.iter().enumerate().skip(item_line) {
        if terminator_pos(strip_line_comment(line), &mut depth).is_some() {
            return idx;
        }
    }
    item_line
}

#[note("doc-note-terminator-pos")]
fn terminator_pos(text: &str, depth: &mut i32) -> Option<usize> {
    for (pos, c) in text.char_indices() {
        match c {
            '(' | '[' => *depth += 1,
            ')' | ']' => *depth -= 1,
            '{' => return Some(pos),
            ';' if *depth <= 0 => return Some(pos),
            _ => {}
        }
    }
    None
}

#[derive(Clone, Debug)]
enum Scope {
    Impl(Option<String>),
    Trait(String),
    Fn,
    Other,
}

#[note("doc-note-scan-items")]
fn scan_items(lines: &[&str]) -> Vec<DocTarget> {
    let mut targets = Vec::new();
    let mut scope_stack: Vec<Scope> = Vec::new();

    let mut current_header = String::new();
    let mut in_block_comment = false;
    let mut in_multiline_string = false;

    let mut line_idx = 0;
    while line_idx < lines.len() {
        let raw_line = lines[line_idx];

        // State before processing braces on this line
        let inside_fn_at_start = scope_stack.iter().any(|s| matches!(s, Scope::Fn));
        let current_owner = scope_stack.iter().rev().find_map(|s| match s {
            Scope::Impl(owner) => owner.clone(),
            Scope::Trait(name) => Some(name.clone()),
            _ => None,
        });

        if !inside_fn_at_start && !in_block_comment && !in_multiline_string {
            let indent = leading_whitespace(raw_line);
            let trimmed = raw_line[indent.len()..].trim_end();

            if let Some((kind, name, is_unsafe)) = parse_item_start(trimmed) {
                let (signature, sig_end_line, term_char) =
                    collect_signature(lines, line_idx, indent.len());
                let returns_result = if kind == ItemKind::Fn {
                    check_returns_result(&signature)
                } else {
                    false
                };

                let body = if kind == ItemKind::Fn && term_char == Some('{') {
                    find_body_range(lines, sig_end_line)
                } else {
                    None
                };

                let (insert_line, doc_lines, existing_docs, existing_note) =
                    scan_leading_block(lines, line_idx);

                targets.push(DocTarget {
                    kind,
                    name,
                    owner: current_owner,
                    signature,
                    item_line: line_idx,
                    insert_line,
                    indent: indent.to_string(),
                    doc_lines,
                    existing_docs,
                    existing_note,
                    body,
                    is_unsafe,
                    returns_result,
                });
            }
        }

        let stripped_comment =
            strip_comments_and_strings(raw_line, &mut in_block_comment, &mut in_multiline_string);

        // Check for opening and closing braces to track scopes.
        for c in stripped_comment.chars() {
            if c == '{' {
                let header = current_header.trim();
                if let Some(owner) = extract_impl_owner(header) {
                    scope_stack.push(Scope::Impl(owner));
                } else if let Some(trait_name) = extract_trait_name(header) {
                    scope_stack.push(Scope::Trait(trait_name));
                } else if is_fn_header(header) {
                    scope_stack.push(Scope::Fn);
                } else {
                    scope_stack.push(Scope::Other);
                }
                current_header.clear();
            } else if c == '}' {
                scope_stack.pop();
                current_header.clear();
            } else if c == ';' {
                current_header.clear();
            } else {
                current_header.push(c);
            }
        }

        line_idx += 1;
    }

    targets
}

fn is_fn_header(header: &str) -> bool {
    let mut s = header.trim();
    if s.starts_with("pub(") {
        if let Some(close_paren) = s.find(')') {
            s = s[close_paren + 1..].trim();
        }
    } else if s.starts_with("pub ") || s.starts_with("pub\t") || s == "pub" {
        s = s[3..].trim();
    }
    loop {
        if let Some(rest) = strip_word(s, "default") {
            s = rest;
        } else if let Some(rest) = strip_word(s, "async") {
            s = rest;
        } else if let Some(rest) = strip_word(s, "unsafe") {
            s = rest;
        } else if let Some(rest) = strip_word(s, "extern") {
            s = rest;
            if s.starts_with('"')
                && let Some(end_quote) = s[1..].find('"')
            {
                s = s[end_quote + 2..].trim();
            }
        } else if let Some(rest) = strip_word(s, "const") {
            s = rest;
        } else {
            break;
        }
    }
    s.starts_with("fn ") || s.starts_with("fn\t") || s.starts_with("fn<") || s == "fn"
}

fn leading_whitespace(line: &str) -> &str {
    let end = line
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(line.len());
    &line[..end]
}

#[note("doc-note-parse-item-start")]
fn parse_item_start(line: &str) -> Option<(ItemKind, String, bool)> {
    let mut s = line.trim();
    if s.is_empty()
        || s.starts_with("//")
        || s.starts_with("/*")
        || s.starts_with('*')
        || s.starts_with("#[")
        || s.starts_with("impl ")
        || s.starts_with("impl<")
        || s.starts_with("use ")
    {
        return None;
    }

    // Optional visibility
    if s.starts_with("pub(") {
        let close_paren = s.find(')')?;
        s = s[close_paren + 1..].trim();
    } else if s.starts_with("pub ") || s.starts_with("pub\t") || s == "pub" {
        s = s[3..].trim();
    }

    // Optional qualifiers
    let mut is_unsafe = false;
    loop {
        if let Some(rest) = strip_word(s, "default") {
            s = rest;
        } else if let Some(rest) = strip_word(s, "async") {
            s = rest;
        } else if let Some(rest) = strip_word(s, "unsafe") {
            is_unsafe = true;
            s = rest;
        } else if let Some(rest) = strip_word(s, "extern") {
            s = rest;
            if s.starts_with('"')
                && let Some(end_quote) = s[1..].find('"')
            {
                s = s[end_quote + 2..].trim();
            }
        } else if s.starts_with("const ") || s.starts_with("const\t") {
            let after_const = s[5..].trim();
            // `const fn` is a qualifier, while `const NAME` is an item.
            if after_const.starts_with("fn ")
                || after_const.starts_with("fn\t")
                || after_const.starts_with("async ")
                || after_const.starts_with("unsafe ")
                || after_const.starts_with("extern ")
            {
                s = after_const;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Declaration keyword
    let (kind, rest) = if let Some(rest) = strip_word(s, "fn") {
        (ItemKind::Fn, rest)
    } else if let Some(rest) = strip_word(s, "struct") {
        (ItemKind::Struct, rest)
    } else if let Some(rest) = strip_word(s, "enum") {
        (ItemKind::Enum, rest)
    } else if let Some(rest) = strip_word(s, "union") {
        (ItemKind::Union, rest)
    } else if let Some(rest) = strip_word(s, "trait") {
        (ItemKind::Trait, rest)
    } else if let Some(rest) = strip_word(s, "const") {
        (ItemKind::Const, rest)
    } else if let Some(rest) = strip_word(s, "static") {
        let rest = strip_word(rest, "mut").unwrap_or(rest);
        (ItemKind::Static, rest)
    } else if let Some(rest) = strip_word(s, "type") {
        (ItemKind::TypeAlias, rest)
    } else if let Some(rest) = strip_word(s, "mod") {
        (ItemKind::Mod, rest)
    } else {
        return None;
    };

    // Item name
    let raw_name = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '#')
        .collect::<String>();
    let name = raw_name.strip_prefix("r#").unwrap_or(&raw_name).to_string();
    if name.is_empty() {
        return None;
    }

    let is_unsafe_effective = is_unsafe && (kind == ItemKind::Fn || kind == ItemKind::Trait);
    Some((kind, name, is_unsafe_effective))
}

fn strip_word<'a>(s: &'a str, word: &str) -> Option<&'a str> {
    let rest = s.strip_prefix(word)?;
    if rest.is_empty() {
        Some(rest)
    } else if rest.starts_with(char::is_whitespace) {
        Some(rest.trim_start())
    } else {
        None
    }
}

#[note("doc-note-collect-signature")]
fn collect_signature(
    lines: &[&str],
    start_line: usize,
    indent_len: usize,
) -> (String, usize, Option<char>) {
    let mut sig_parts = Vec::new();
    let mut term_char = None;
    let mut sig_end_line = start_line;
    let mut depth = 0;

    for (idx, line) in lines.iter().enumerate().skip(start_line) {
        let text = if idx == start_line && indent_len < line.len() {
            &line[indent_len..]
        } else {
            line
        };

        let stripped = strip_line_comment(text);
        if let Some(pos) = terminator_pos(stripped, &mut depth) {
            let ch = stripped.as_bytes()[pos] as char;
            term_char = Some(ch);
            sig_parts.push(&stripped[..pos]);
            sig_end_line = idx;
            break;
        } else {
            sig_parts.push(stripped);
            sig_end_line = idx;
        }
    }

    let raw = sig_parts.join(" ");
    let signature = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    (signature, sig_end_line, term_char)
}

#[note("doc-note-check-returns-result")]
fn check_returns_result(signature: &str) -> bool {
    let arrow_pos = match signature.find("->") {
        Some(pos) => pos,
        None => return false,
    };
    let ret = &signature[arrow_pos + 2..].trim();
    let ret_before_where = ret
        .split_whitespace()
        .take_while(|&w| w != "where")
        .collect::<Vec<_>>()
        .join(" ");
    let path = if let Some(bracket) = ret_before_where.find('<') {
        ret_before_where[..bracket].trim()
    } else {
        ret_before_where.trim()
    };

    path == "Result" || path.ends_with("::Result")
}

#[note("doc-note-find-body-range")]
fn find_body_range(lines: &[&str], start_line: usize) -> Option<std::ops::Range<usize>> {
    let mut depth = 0;
    let mut found_open = false;
    let mut in_block_comment = false;
    let mut in_multiline_string = false;

    for (idx, line) in lines.iter().enumerate().skip(start_line) {
        let stripped =
            strip_comments_and_strings(line, &mut in_block_comment, &mut in_multiline_string);
        for c in stripped.chars() {
            if c == '{' {
                depth += 1;
                found_open = true;
            } else if c == '}' && found_open {
                depth -= 1;
                if depth == 0 {
                    return Some(start_line..idx + 1);
                }
            }
        }
    }

    None
}

#[note("doc-note-scan-leading-block")]
fn scan_leading_block(
    lines: &[&str],
    item_line: usize,
) -> (usize, std::ops::Range<usize>, String, Option<String>) {
    if item_line == 0 {
        return (0, 0..0, String::new(), None);
    }

    let mut start_idx = item_line;
    let mut cur = item_line;

    while cur > 0 {
        cur -= 1;
        let line = lines[cur].trim();
        if line.is_empty() {
            break;
        }
        if line.starts_with("///") || line.starts_with("#[") || line.ends_with(']') {
            start_idx = cur;
        } else {
            break;
        }
    }

    let insert_line = start_idx;
    let mut doc_start = None;
    let mut doc_end = None;
    let mut doc_lines_vec = Vec::new();
    let mut existing_note = None;

    for (i, &raw) in lines.iter().enumerate().take(item_line).skip(insert_line) {
        let trimmed_start = raw.trim_start();
        if trimmed_start.starts_with("///") {
            if doc_start.is_none() {
                doc_start = Some(i);
            }
            doc_end = Some(i + 1);

            let stripped = if let Some(s) = trimmed_start.strip_prefix("/// ") {
                s
            } else if let Some(s) = trimmed_start.strip_prefix("///") {
                s
            } else {
                trimmed_start
            };
            doc_lines_vec.push(stripped);
        }

        // Check for existing note attribute
        let trimmed = raw.trim();
        if let Some(pos) = trimmed.find("note(") {
            let before = &trimmed[..pos];
            if before.ends_with("#[")
                || before.ends_with("#[crawrad_note::")
                || before.ends_with("::")
                || before.ends_with(' ')
            {
                let rest = trimmed[pos + 5..].trim_start();
                if let Some(inner) = rest.strip_prefix('"')
                    && let Some(quote_end) = inner.find('"')
                {
                    let id = &inner[..quote_end];
                    existing_note = Some(id.to_string());
                }
            }
        }
    }

    let doc_lines = match (doc_start, doc_end) {
        (Some(start), Some(end)) => start..end,
        _ => insert_line..insert_line,
    };
    let existing_docs = doc_lines_vec.join("\n");

    (insert_line, doc_lines, existing_docs, existing_note)
}

#[note("doc-note-strip-comments-and-strings")]
fn strip_comments_and_strings(
    line: &str,
    in_block_comment: &mut bool,
    in_multiline_string: &mut bool,
) -> String {
    let mut out = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let mut i = 0;
    let mut in_char = false;
    let mut escaped = false;

    while i < bytes.len() {
        if *in_block_comment {
            if bytes[i] == b'*' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                *in_block_comment = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }

        if *in_multiline_string {
            if escaped {
                escaped = false;
            } else if bytes[i] == b'\\' {
                escaped = true;
            } else if bytes[i] == b'"' {
                *in_multiline_string = false;
            }
            i += 1;
            continue;
        }

        let b = bytes[i];

        if in_char {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'\'' {
                in_char = false;
            }
            i += 1;
            continue;
        }

        if b == b'/' && i + 1 < bytes.len() {
            if bytes[i + 1] == b'/' {
                // Line comment: rest of line is ignored
                break;
            }
            if bytes[i + 1] == b'*' {
                *in_block_comment = true;
                i += 2;
                continue;
            }
        }

        if b == b'"' {
            *in_multiline_string = true;
            i += 1;
            continue;
        }

        if b == b'\'' {
            in_char = true;
            i += 1;
            continue;
        }

        out.push(b as char);
        i += 1;
    }

    out
}

#[note("doc-note-strip-line-comment")]
fn strip_line_comment(line: &str) -> &str {
    let mut in_string = false;
    let mut escaped = false;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
        } else if b == b'"' {
            in_string = true;
        } else if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            return &line[..i];
        }
        i += 1;
    }
    line
}

#[note("doc-note-extract-impl-owner")]
fn extract_impl_owner(header: &str) -> Option<Option<String>> {
    let s = header.trim();
    let impl_idx = s.find("impl")?;
    if impl_idx > 0 && s.as_bytes()[impl_idx - 1].is_ascii_alphanumeric() {
        return None;
    }
    let after_impl = s[impl_idx + 4..].trim_start();
    let after_generics = if after_impl.starts_with('<') {
        skip_balanced_angles(after_impl)?
    } else {
        after_impl
    };

    let trimmed = after_generics.trim_start();
    if let Some(after_for) = find_for_target(trimmed) {
        Some(extract_type_name(after_for))
    } else {
        Some(extract_type_name(trimmed))
    }
}

fn skip_balanced_angles(s: &str) -> Option<&str> {
    if !s.starts_with('<') {
        return Some(s);
    }
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        if c == '<' {
            depth += 1;
        } else if c == '>' {
            depth -= 1;
            if depth == 0 {
                return Some(&s[i + 1..]);
            }
        }
    }
    None
}

fn find_for_target(s: &str) -> Option<&str> {
    let mut angle_depth = 0;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'<' => angle_depth += 1,
            b'>' if angle_depth > 0 => {
                angle_depth -= 1;
            }
            b'f' if angle_depth == 0 && s[i..].starts_with("for") => {
                let prev_ok = i == 0 || bytes[i - 1].is_ascii_whitespace();
                let next_idx = i + 3;
                let next_ok = next_idx < bytes.len() && bytes[next_idx].is_ascii_whitespace();
                if prev_ok && next_ok {
                    let rest = s[next_idx..].trim_start();
                    if !rest.starts_with('<') {
                        return Some(rest);
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn extract_type_name(s: &str) -> Option<String> {
    let mut trimmed = s.trim_start();
    while trimmed.starts_with('&') {
        trimmed = trimmed[1..].trim_start();
        if trimmed.starts_with("mut") && trimmed[3..].starts_with(char::is_whitespace) {
            trimmed = trimmed[3..].trim_start();
        }
    }

    let mut end = trimmed.len();
    for (idx, c) in trimmed.char_indices() {
        if c == '<' || c == '{' || c == ';' || c.is_whitespace() {
            end = idx;
            break;
        }
    }
    let path = &trimmed[..end];
    let last_seg = path.split("::").last()?.trim();
    let ident: String = last_seg
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    if ident.is_empty() { None } else { Some(ident) }
}

#[note("doc-note-extract-trait-name")]
fn extract_trait_name(header: &str) -> Option<String> {
    let s = header.trim();
    let trait_idx = s.find("trait")?;
    if trait_idx > 0 && s.as_bytes()[trait_idx - 1].is_ascii_alphanumeric() {
        return None;
    }
    let after_trait = s[trait_idx + 5..].trim_start();
    let ident: String = after_trait
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    if ident.is_empty() { None } else { Some(ident) }
}

#[note("doc-note-item-facts-for-target")]
pub fn item_facts_for_target(source: &str, target: &DocTarget) -> ItemFacts {
    let mut first_panic = None;
    if let Some(ref body_range) = target.body {
        let lines: Vec<&str> = source.lines().collect();
        for line_idx in body_range.clone() {
            if line_idx >= lines.len() {
                break;
            }
            let raw_line = lines[line_idx];
            let stripped = strip_line_comment(raw_line);

            if let Some((_, construct)) = earliest_panic_construct(stripped) {
                let line_1based = line_idx + 1;
                first_panic = Some(format!("calls {construct} on line {line_1based}"));
                break;
            }
        }
    }

    let is_public = if is_pub_exact(&target.signature) {
        true
    } else if let Some(ref owner) = target.owner {
        is_owner_trait(source, owner)
    } else {
        false
    };

    ItemFacts {
        is_public,
        returns_result: target.returns_result,
        is_unsafe: target.is_unsafe,
        is_trait: target.kind == ItemKind::Trait,
        first_panic,
    }
}

fn is_pub_exact(sig: &str) -> bool {
    let s = sig.trim_start();
    (s.starts_with("pub ") || s.starts_with("pub\t") || s == "pub") && !s.starts_with("pub(")
}

fn is_owner_trait(source: &str, owner: &str) -> bool {
    let mut in_block_comment = false;
    let mut in_multiline_string = false;
    for line in source.lines() {
        let stripped =
            strip_comments_and_strings(line, &mut in_block_comment, &mut in_multiline_string);
        if let Some(trait_name) = extract_trait_name(&stripped)
            && trait_name == owner
        {
            return true;
        }
    }
    false
}

#[note("doc-note-suggested-sections")]
pub fn suggested_sections(source: &str, target: &DocTarget) -> Vec<Suggestion> {
    let facts = item_facts_for_target(source, target);
    suggestions_for(&facts)
}

#[note("doc-note-note-alert")]
pub fn note_alert(source: &str, line: usize, note: &SidecarNote) -> bool {
    if note.kind != NoteKind::Doc {
        return false;
    }
    let Some(target) = doc_target_at(source, line) else {
        return false;
    };
    if crawrad_note_format::stale_signature(&note.id, &note.signature, &target.signature).is_some()
    {
        return true;
    }
    let facts = item_facts_for_target(source, &target);
    !crawrad_note_format::rustdoc::doc_problems(&note.content, &facts).is_empty()
}

#[note("doc-note-note-mark")]
pub fn note_mark(root: &Path, id: &str, source: &str, line: usize) -> (String, bool, bool) {
    match crate::sidecar_notes::load_note(root, id) {
        Ok(n) => {
            let hidden = n.kind == NoteKind::Comment && n.resolved;
            (
                format!("{}\n{}", n.title, n.content.trim()),
                note_alert(source, line, &n),
                hidden,
            )
        }
        Err(_) => (String::new(), false, false),
    }
}

#[note("doc-note-suggested-note-id")]
pub fn suggested_note_id(target: &DocTarget) -> String {
    let name_kebab = to_kebab_case(&target.name);
    let id = match &target.owner {
        Some(owner) if !owner.trim().is_empty() => {
            let owner_kebab = to_kebab_case(owner);
            format!("{owner_kebab}-{name_kebab}")
        }
        _ => name_kebab,
    };
    debug_assert!(crawrad_note_format::validate_id(&id).is_ok());
    id
}

fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        let c = chars[i];
        if c == '_' || c == '-' {
            if !result.ends_with('-') && !result.is_empty() {
                result.push('-');
            }
        } else if c.is_ascii_uppercase() {
            if i > 0 {
                let prev = chars[i - 1];
                let next = chars.get(i + 1).copied();
                let boundary = prev.is_ascii_lowercase()
                    || prev.is_ascii_digit()
                    || (prev.is_ascii_uppercase() && next.is_some_and(|n| n.is_ascii_lowercase()));
                if boundary && !result.ends_with('-') && !result.is_empty() {
                    result.push('-');
                }
            }
            result.push(c.to_ascii_lowercase());
        } else if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
        }
    }
    let trimmed = result.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "note".to_string()
    } else {
        trimmed
    }
}

#[note("doc-note-apply-note-attribute")]
pub fn apply_note_attribute(source: &str, target: &DocTarget, id: &str) -> String {
    if target.existing_note.is_some() {
        return source.to_string();
    }

    let is_crlf = source.contains("\r\n");
    let line_ending = if is_crlf { "\r\n" } else { "\n" };
    let trailing_newline = source.ends_with('\n');

    let mut lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

    // Only the `///` lines go: an attribute between two doc lines stays.
    // They all sit at or after `insert_line`, so it does not move.
    let end = target.doc_lines.end.min(lines.len());
    for idx in (target.doc_lines.start..end).rev() {
        if lines[idx].trim_start().starts_with("///") {
            lines.remove(idx);
        }
    }
    let insert_idx = target.insert_line.min(lines.len());

    let note_line = format!("{}#[crawrad_note::note(\"{id}\")]", target.indent);
    lines.insert(insert_idx, note_line);

    let mut result = lines.join(line_ending);
    if trailing_newline {
        result.push_str(line_ending);
    }
    result
}

#[note("doc-note-crate-dir-of")]
pub fn crate_dir_of(file: &Path) -> Option<PathBuf> {
    let mut current = if file.is_dir() {
        Some(file)
    } else {
        file.parent()
    };

    while let Some(dir) = current {
        let cargo_toml_path = dir.join("Cargo.toml");
        if cargo_toml_path.is_file()
            && let Ok(contents) = std::fs::read_to_string(&cargo_toml_path)
            && let Ok(table) = toml::from_str::<toml::Table>(&contents)
            && table.contains_key("package")
        {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}

#[note("doc-note-depends-on-crawrad-note")]
pub fn depends_on_crawrad_note(cargo_toml: &str) -> bool {
    if let Ok(val) = toml::from_str::<toml::Value>(cargo_toml)
        && let Some(deps) = val.get("dependencies").and_then(|d| d.as_table())
        && (deps.contains_key("crawrad-note") || deps.contains_key("crawrad_note"))
    {
        return true;
    }
    false
}

/// `cargo_toml` with `crawrad-note` added to `[dependencies]` as a path to
/// `note_crate`, written the way the project generator writes the runtime
/// crates (relative when one exists). `None` when it is already a dependency
/// or the manifest does not parse.
pub fn with_crawrad_note_dependency(
    cargo_toml: &str,
    manifest_dir: &Path,
    note_crate: &Path,
) -> Option<String> {
    if depends_on_crawrad_note(cargo_toml) {
        return None;
    }
    let mut doc = cargo_toml.parse::<toml_edit::DocumentMut>().ok()?;
    let path = crawrad_codegen::dependencies::manifest_path(manifest_dir, note_crate)
        .to_string_lossy()
        .replace('\\', "/");
    let mut dep = toml_edit::InlineTable::new();
    dep.insert("path", path.into());
    let deps = doc
        .entry("dependencies")
        .or_insert_with(toml_edit::table)
        .as_table_like_mut()?;
    deps.insert("crawrad-note", toml_edit::value(dep));
    Some(doc.to_string())
}

#[note("doc-note-crate-name-of")]
pub fn crate_name_of(crate_dir: &Path) -> Option<String> {
    let cargo_toml_path = crate_dir.join("Cargo.toml");
    let contents = std::fs::read_to_string(&cargo_toml_path).ok()?;
    let val = toml::from_str::<toml::Value>(&contents).ok()?;
    val.get("package")?
        .get("name")?
        .as_str()
        .map(|s| s.to_string())
}

#[derive(Clone, Copy)]
enum PrefixKind {
    Whether,
    WhetherItHas,
    WhetherItCan,
    ConvertsTo,
    CreatesFrom,
    Sets,
    ReturnsThe,
}

const SPECIAL_PREFIXES: &[(&str, PrefixKind)] = &[
    ("is", PrefixKind::Whether),
    ("has", PrefixKind::WhetherItHas),
    ("can", PrefixKind::WhetherItCan),
    ("to", PrefixKind::ConvertsTo),
    ("from", PrefixKind::CreatesFrom),
    ("with", PrefixKind::Sets),
    ("get", PrefixKind::ReturnsThe),
];

const KNOWN_GETTERS: &[&str] = &[
    "name",
    "id",
    "kind",
    "len",
    "size",
    "count",
    "capacity",
    "status",
    "value",
    "title",
    "text",
    "description",
    "header",
    "footer",
    "content",
    "target",
    "path",
    "file",
    "line",
    "column",
    "width",
    "height",
    "index",
    "tag",
    "tags",
    "author",
    "version",
    "data",
    "result",
];

#[note("doc-note-doc-skeleton")]
pub fn doc_skeleton(source: &str, target: &DocTarget, crate_name: Option<&str>) -> String {
    let mut sections = Vec::new();
    sections.push(summary_for_target(target));

    if target.kind == ItemKind::Fn {
        let example = build_example_section(source, target, crate_name);
        sections.push(example);
    }

    let facts = item_facts_for_target(source, target);
    if facts.returns_result {
        sections.push("# Errors\n\nReturns an error when \u{2026}".to_string());
    }
    if facts.first_panic.is_some() {
        sections.push("# Panics\n\nPanics when \u{2026}".to_string());
    }
    if facts.is_unsafe {
        sections.push("# Safety\n\nThe caller must ensure \u{2026}".to_string());
    }

    sections.join("\n\n")
}

fn summary_for_target(target: &DocTarget) -> String {
    match target.kind {
        ItemKind::Fn => summary_for_fn(target),
        ItemKind::Struct
        | ItemKind::Enum
        | ItemKind::Union
        | ItemKind::Trait
        | ItemKind::TypeAlias => {
            let words = split_camel_case(&target.name);
            let joined = words.join(" ");
            let article = article_for(&joined);
            format!("{article} {joined}.")
        }
        ItemKind::Const | ItemKind::Static => format!("The {}.", snake_words(&target.name)),
        ItemKind::Mod => format!("The {} module.", snake_words(&target.name)),
    }
}

fn summary_for_fn(target: &DocTarget) -> String {
    if target.name == "new" {
        return match &target.owner {
            Some(owner) if !owner.trim().is_empty() => format!("Creates a new `{owner}`."),
            _ => "Creates a new value.".to_string(),
        };
    }

    if let Some(rest) = target.name.strip_prefix("try_") {
        let dummy = DocTarget {
            name: rest.to_string(),
            ..target.clone()
        };
        let inner = summary_for_fn(&dummy);
        let trimmed = inner.trim_end_matches('.');
        return format!("{trimmed} (fails instead of panicking).");
    }

    let words: Vec<&str> = target.name.split('_').filter(|w| !w.is_empty()).collect();
    if words.is_empty() {
        return "Returns the value.".to_string();
    }

    for (prefix, kind) in SPECIAL_PREFIXES {
        if words[0] == *prefix && words.len() > 1 {
            let rest = words[1..].join(" ");
            return match kind {
                PrefixKind::Whether => format!("Returns whether {rest}."),
                PrefixKind::WhetherItHas => format!("Returns whether it has {rest}."),
                PrefixKind::WhetherItCan => format!("Returns whether it can {rest}."),
                PrefixKind::ConvertsTo => format!("Converts to {rest}."),
                PrefixKind::CreatesFrom => format!("Creates the value from {rest}."),
                PrefixKind::Sets => format!("Sets {rest}."),
                PrefixKind::ReturnsThe => format!("Returns the {rest}."),
            };
        }
    }

    if KNOWN_GETTERS.contains(&target.name.as_str()) {
        let rest = words.join(" ");
        return format!("Returns the {rest}.");
    }

    let first_conjugated = to_third_person(words[0]);
    if words.len() > 1 {
        let rest = words[1..].join(" ");
        format!("{first_conjugated} {rest}.")
    } else {
        format!("{first_conjugated}.")
    }
}

#[note("doc-note-snake-words")]
fn snake_words(name: &str) -> String {
    let words: Vec<String> = name
        .split('_')
        .filter(|w| !w.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    words.join(" ")
}

fn to_third_person(verb: &str) -> String {
    let lower = verb.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let conjugated = if lower.ends_with("sh")
        || lower.ends_with("ch")
        || lower.ends_with('s')
        || lower.ends_with('x')
        || lower.ends_with('z')
        || lower.ends_with('o')
    {
        format!("{lower}es")
    } else if lower.ends_with('y') && bytes.len() >= 2 && is_consonant(bytes[bytes.len() - 2]) {
        format!("{}ies", &lower[..lower.len() - 1])
    } else {
        format!("{lower}s")
    };
    capitalize(&conjugated)
}

fn is_consonant(b: u8) -> bool {
    b.is_ascii_alphabetic() && !matches!(b.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u')
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

fn article_for(word: &str) -> &'static str {
    match word.chars().next() {
        Some(c) if matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') => "An",
        _ => "A",
    }
}

fn split_camel_case(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut words = Vec::new();
    let mut current = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            let prev = chars[i - 1];
            let next = chars.get(i + 1).copied();
            let boundary = prev.is_ascii_lowercase()
                || prev.is_ascii_digit()
                || (prev.is_ascii_uppercase() && next.is_some_and(|n| n.is_ascii_lowercase()));
            if boundary && !current.is_empty() {
                words.push(current.to_ascii_lowercase());
                current.clear();
            }
        }
        current.push(c);
    }
    if !current.is_empty() {
        words.push(current.to_ascii_lowercase());
    }
    words
}

fn build_example_section(source: &str, target: &DocTarget, crate_name: Option<&str>) -> String {
    let mut lines = Vec::new();

    if let Some(crate_str) = crate_name {
        let crate_ident = crate_str.replace('-', "_");
        if target.owner.is_none() && is_pub_signature(&target.signature) {
            lines.push(format!("use {crate_ident}::{};", target.name));
        } else if let Some(ref owner) = target.owner {
            lines.push(format!("use {crate_ident}::{owner};"));
        }
    }

    let mut generic_params = extract_fn_generic_params(&target.signature, &target.name);
    generic_params.extend(extract_impl_generic_params(source, target.item_line));

    let raw_params = extract_raw_params(&target.signature).unwrap_or("");
    let param_parts = split_top_level_commas(raw_params);

    let mut has_self = false;
    let mut is_mut_self = false;
    let mut non_self_params = Vec::new();

    for param in param_parts {
        let p = param.trim();
        if p.is_empty() {
            continue;
        }
        if let Some(is_mut) = self_receiver(p) {
            has_self = true;
            is_mut_self = is_mut_self || is_mut;
        } else if let Some((pat, ty)) = split_param_pattern_and_type(p) {
            non_self_params.push((pat, ty));
        }
    }

    let mut call_args = Vec::new();
    for (idx, (pat, ty)) in non_self_params.iter().enumerate() {
        let param_num = idx + 1;
        let stripped_pat = pat.strip_prefix("mut ").unwrap_or(pat).trim();
        let is_plain_ident = !stripped_pat.is_empty()
            && stripped_pat != "_"
            && stripped_pat.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
            && stripped_pat
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_');
        let param_name = if is_plain_ident {
            stripped_pat.to_string()
        } else {
            format!("arg{param_num}")
        };
        let clean_ty = clean_type(ty, &generic_params);
        lines.push(format!("let {param_name}: {clean_ty} = todo!();"));
        call_args.push(param_name);
    }

    let args_str = call_args.join(", ");
    let is_unit = check_is_unit_return(&target.signature);

    if let Some(ref owner) = target.owner {
        if has_self {
            let decl = if is_mut_self {
                format!("let mut value: {owner} = todo!();")
            } else {
                format!("let value: {owner} = todo!();")
            };
            lines.push(decl);
            let call = if is_unit {
                format!("value.{}({args_str});", target.name)
            } else {
                format!("let result = value.{}({args_str});", target.name)
            };
            lines.push(call);
        } else {
            let call = if is_unit {
                format!("{owner}::{}({args_str});", target.name)
            } else {
                format!("let result = {owner}::{}({args_str});", target.name)
            };
            lines.push(call);
        }
    } else {
        let call = if is_unit {
            format!("{}({args_str});", target.name)
        } else {
            format!("let result = {}({args_str});", target.name)
        };
        lines.push(call);
    }

    format!("# Examples\n\n```no_run\n{}\n```", lines.join("\n"))
}

fn is_pub_signature(sig: &str) -> bool {
    let s = sig.trim_start();
    s.starts_with("pub ") || s.starts_with("pub(") || s.starts_with("pub\t")
}

fn check_is_unit_return(sig: &str) -> bool {
    let Some(pos) = sig.find("->") else {
        return true;
    };
    let ret = sig[pos + 2..].trim();
    let ret_clean = ret
        .split_whitespace()
        .take_while(|&w| w != "where")
        .collect::<Vec<_>>()
        .join(" ");
    ret_clean == "()" || ret_clean.is_empty()
}

#[note("doc-note-with-depth")]
fn with_depth(s: &str) -> impl Iterator<Item = (usize, char, usize, usize)> + '_ {
    let mut depth = 0usize;
    let mut prev = ' ';
    s.char_indices().map(move |(i, c)| {
        let before = depth;
        match c {
            '(' | '<' | '[' => depth += 1,
            ')' | ']' => depth = depth.saturating_sub(1),
            '>' if prev != '-' => depth = depth.saturating_sub(1),
            _ => {}
        }
        prev = c;
        (i, c, before, depth)
    })
}

#[note("doc-note-closing")]
fn closing(s: &str, open: usize) -> Option<usize> {
    with_depth(&s[open..])
        .skip(1)
        .find(|&(_, _, before, after)| before == 1 && after == 0)
        .map(|(i, ..)| open + i)
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut last = 0;
    for (i, c, depth, _) in with_depth(s) {
        if c == ',' && depth == 0 {
            parts.push(s[last..i].trim());
            last = i + 1;
        }
    }
    parts.push(s[last..].trim());
    parts.retain(|p| !p.is_empty());
    parts
}

fn extract_raw_params(sig: &str) -> Option<&str> {
    let (open, ..) = with_depth(sig).find(|&(_, c, depth, _)| c == '(' && depth == 0)?;
    let close = closing(sig, open)?;
    Some(&sig[open + 1..close])
}

#[note("doc-note-self-receiver")]
fn self_receiver(param: &str) -> Option<bool> {
    match strip_named_lifetimes(param.trim()).as_str() {
        "self" | "&self" | "mut self" => Some(false),
        "&mut self" => Some(true),
        _ => None,
    }
}

fn split_param_pattern_and_type(param: &str) -> Option<(&str, &str)> {
    let (i, ..) = with_depth(param).find(|&(_, c, depth, _)| c == ':' && depth == 0)?;
    Some((param[..i].trim(), param[i + 1..].trim()))
}

#[note("doc-note-clean-type")]
fn clean_type(ty: &str, generic_params: &[String]) -> String {
    let ty = ty.trim();
    let generic = ty
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .any(|w| w == "impl" || generic_params.iter().any(|g| g == w));
    if generic {
        "_".to_string()
    } else {
        strip_named_lifetimes(ty)
    }
}

#[note("doc-note-strip-named-lifetimes")]
fn strip_named_lifetimes(ty: &str) -> String {
    let mut out = String::with_capacity(ty.len());
    let mut rest = ty;
    while let Some(c) = rest.chars().next() {
        if let Some(after) = rest.strip_prefix("&'") {
            let name_len = after
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(after.len());
            out.push('&');
            rest = after[name_len..].trim_start();
        } else if c == '<'
            && let Some(close) = closing(rest, 0)
        {
            let kept: Vec<String> = split_top_level_commas(&rest[1..close])
                .into_iter()
                .filter(|a| !a.starts_with('\''))
                .map(strip_named_lifetimes)
                .collect();
            if !kept.is_empty() {
                out.push('<');
                out.push_str(&kept.join(", "));
                out.push('>');
            }
            rest = &rest[close + 1..];
        } else {
            out.push(c);
            rest = &rest[c.len_utf8()..];
        }
    }
    out
}

#[note("doc-note-leading-generics")]
fn leading_generics(text: &str) -> Vec<String> {
    let text = text.trim_start();
    if !text.starts_with('<') {
        return Vec::new();
    }
    let Some(close) = closing(text, 0) else {
        return Vec::new();
    };
    split_top_level_commas(&text[1..close])
        .into_iter()
        .filter(|p| !p.starts_with('\''))
        .map(|p| p.strip_prefix("const ").unwrap_or(p).trim_start())
        .map(|p| {
            p.chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect::<String>()
        })
        .filter(|n| !n.is_empty())
        .collect()
}

fn extract_fn_generic_params(sig: &str, name: &str) -> Vec<String> {
    let needle = format!("fn {name}");
    sig.find(&needle)
        .map(|pos| leading_generics(&sig[pos + needle.len()..]))
        .unwrap_or_default()
}

#[note("doc-note-extract-impl-generic-params")]
fn extract_impl_generic_params(source: &str, item_line: usize) -> Vec<String> {
    let above: Vec<&str> = source.lines().take(item_line).collect();
    above
        .into_iter()
        .rev()
        .map(str::trim_start)
        .find(|l| l.starts_with("impl<") || l.starts_with("impl "))
        .and_then(|l| l.strip_prefix("impl"))
        .map(leading_generics)
        .unwrap_or_default()
}

#[cfg(test)]
#[path = "test/doc_note/test.rs"]
mod test;
