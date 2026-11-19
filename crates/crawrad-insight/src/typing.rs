//! **The completion list while the user types** — pure, no window.
//!
//! The owner, on 01/10/2026, comparing with a modern editor: *"if I start typing
//! 'tab' it doesn't do any search, much less filter which would be the TOP feature"*.
//! A modern editor narrows the open list with every key: `sp` keeps `split`,
//! `splitlines` (start with it) and then `swapcase`, `strip` (the letters in
//! order). This module decides that, and where a list starts and ends.

use crawrad_note::note;
#[note("typing-word-prefix")]
pub fn word_prefix(line: &str, byte: usize) -> &str {
    let up_to = line.get(..byte).unwrap_or(line);
    let start = up_to.rfind(|c: char| !is_name_char(c)).map_or(0, |i| i + 1);
    &up_to[start..]
}

#[note("typing-dot-context")]
pub fn dot_context(line: &str, byte: usize) -> (Option<String>, String, usize) {
    let up_to = line.get(..byte).unwrap_or(line);
    let prefix = word_prefix(up_to, up_to.len()).to_string();
    let before_prefix = &up_to[..up_to.len() - prefix.len()];
    if let Some(before_dot) = before_prefix.trim_end().strip_suffix('.') {
        let start = before_dot
            .rfind(|c: char| !is_name_char(c))
            .map_or(0, |i| i + 1);
        let base = before_dot[start..].trim();
        // `"Cancel".`, `name().`, `list[0].`, `value?.`: the dot follows an
        // expression, not a name — still a member access, and only the
        // server knows its type. The base is the closing character.
        let base = match before_dot.trim_end().chars().last() {
            Some(c) if base.is_empty() && CLOSES_AN_EXPRESSION.contains(&c) => c.to_string(),
            _ => base.to_string(),
        };
        if !base.is_empty() {
            let replace_from = before_prefix
                .rfind('.')
                .map_or(up_to.len() - prefix.len(), |i| i + 1);
            return (Some(base), prefix, replace_from);
        }
    }
    let replace_from = up_to.len() - prefix.len();
    (None, prefix, replace_from)
}

#[note("typing-typed-since")]
pub fn typed_since(before_caret: &str, start: usize) -> Option<&str> {
    let typed = before_caret.get(start..)?;
    typed.chars().all(is_name_char).then_some(typed)
}

#[note("typing-opens-after-dot")]
pub fn opens_after_dot(before_caret: &str) -> bool {
    let Some(before) = before_caret.strip_suffix('.') else {
        return false;
    };
    // A dot in a comment or inside a string is prose: `// fim.`, `"Olá. "`.
    if !in_code(before) {
        return false;
    }
    if before.ends_with(CLOSES_AN_EXPRESSION) {
        return true;
    }
    let name = word_prefix(before, before.len());
    name.chars()
        .next()
        .is_some_and(|c| c.is_alphabetic() || c == '_')
}

#[note("typing-match-score")]
pub fn match_score(label: &str, query: &str) -> Option<u32> {
    if query.is_empty() {
        return Some(0);
    }
    let label_l = label.to_lowercase();
    let query_l = query.to_lowercase();
    // Inside one class the caller's order stands: it is the server's
    // relevance (`len` before `PartialOrd::le` for `le`).
    // Exactly what was typed comes first, as in a modern editor (`le` -> `le`).
    if label_l == query_l {
        return Some(4000);
    }
    if label_l.starts_with(&query_l) {
        return Some(3000);
    }
    if label_l.contains(&query_l) {
        return Some(2000);
    }
    // The letters in order, with gaps: fewer gaps is better.
    let mut gaps = 0u32;
    let mut chars = label_l.chars();
    for q in query_l.chars() {
        let mut skipped = 0u32;
        loop {
            match chars.next() {
                Some(c) if c == q => break,
                Some(_) => skipped += 1,
                None => return None,
            }
        }
        gaps += skipped;
    }
    Some(1000u32.saturating_sub(gaps))
}

#[note("typing-rank")]
pub fn rank<'a>(labels: impl IntoIterator<Item = &'a str>, query: &str) -> Vec<usize> {
    let mut scored: Vec<(usize, u32)> = labels
        .into_iter()
        .enumerate()
        .filter_map(|(i, l)| match_score(l, query).map(|s| (i, s)))
        .collect();
    scored.sort_by_key(|s| std::cmp::Reverse(s.1));
    scored.into_iter().map(|(i, _)| i).collect()
}

#[note("typing-rust-keywords")]
pub const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

#[note("typing-common-types")]
pub const COMMON_TYPES: &[&str] = &[
    "String", "Vec", "Option", "Result", "Some", "None", "Ok", "Err", "Box", "Rc", "Arc", "Cell",
    "RefCell", "HashMap", "HashSet", "BTreeMap", "BTreeSet", "Path", "PathBuf", "bool", "usize",
    "isize", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "f32", "f64", "char",
    "str",
];

#[note("typing-page")]
pub const PAGE: usize = 8;

#[note("typing-list-top")]
pub fn list_top(
    top: usize,
    selected: usize,
    moved_selection: bool,
    scrolled: isize,
    total: usize,
) -> usize {
    let last_top = total.saturating_sub(PAGE);
    let mut top = top.saturating_add_signed(scrolled).min(last_top);
    if moved_selection {
        if selected < top {
            top = selected;
        } else if selected >= top + PAGE {
            top = selected + 1 - PAGE;
        }
    }
    top.min(last_top)
}

#[note("typing-closes-an-expression")]
const CLOSES_AN_EXPRESSION: &[char] = &['"', '\'', ')', ']', '?'];

#[note("typing-opening")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opening {
    /// At once, with the `rust-analyzer…` row while it answers: `.` and `::`.
    Loud,
    /// Asked quietly, shown only if the server has items: `'`, `(` — a
    /// call's `(` must not flash an empty box.
    Quiet,
}

#[note("typing-typed-trigger")]
pub fn typed_trigger(before: &str, triggers: &[String]) -> Option<(char, Opening)> {
    let c = before.chars().last()?;
    if !triggers.iter().any(|t| t.chars().eq(std::iter::once(c))) {
        return None;
    }
    let rest = &before[..before.len() - c.len_utf8()];
    if !in_code(rest) {
        return None;
    }
    match c {
        '.' => opens_after_dot(before).then_some((c, Opening::Loud)),
        ':' => rest.ends_with(':').then_some((c, Opening::Loud)),
        _ => Some((c, Opening::Quiet)),
    }
}

#[note("typing-in-code")]
pub fn in_code(before: &str) -> bool {
    let mut in_string = false;
    let mut escaped = false;
    let mut prev = '\0';
    for c in before.chars() {
        if in_string {
            match (escaped, c) {
                (true, _) => escaped = false,
                (false, '\\') => escaped = true,
                (false, '"') => in_string = false,
                _ => {}
            }
        } else if c == '"' {
            in_string = true;
        } else if c == '/' && prev == '/' {
            return false;
        }
        prev = c;
    }
    !in_string
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[note("typing-signatureedit")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureEdit {
    /// A character the server lists for signature help was typed in code:
    /// ask, as LSP `TriggerCharacter`.
    Typed(char),
    /// Any other edit while the card is up: ask again, as LSP
    /// `ContentChange`, so a deleted `,` moves the mark back and a `)` that
    /// leaves the call closes the card (the server answers nothing).
    Follow,
}

#[note("typing-signature-edit")]
pub fn signature_edit(before: &str, triggers: &[String], open: bool) -> Option<SignatureEdit> {
    let typed = before.chars().last().filter(|c| {
        triggers.iter().any(|t| t.chars().eq(std::iter::once(*c)))
            && in_code(&before[..before.len() - c.len_utf8()])
    });
    match typed {
        Some(c) => Some(SignatureEdit::Typed(c)),
        None => open.then_some(SignatureEdit::Follow),
    }
}

#[cfg(test)]
#[path = "test/typing/test.rs"]
mod tests;
