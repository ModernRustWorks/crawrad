//! **The values a stop writes after the code**: VS Code's inline
//! values, the presentation the owner approved for the debugger — one step
//! further: what a line USES, not just which local it names.
//!
//! On the lines from [`LINES_ABOVE`] above the stop down to it, every path that
//! starts at a local of the frame (`self.btn_ok.enabled`, `total`) is evaluated
//! by the session, and the line gets `path = value` after its end, in grey.
//! Measured on 30/09/2026: writing each LOCAL instead put
//! `self = FormMain { tree_view1: … }` on every line of a handler — true, and
//! useless. A bare `self` is therefore never shown; a method call ends the path
//! (`self.items.len()` shows `self.items`); comments and strings are not code.

use crawrad_dap::ShownVariable;
use crawrad_note::note;

#[note("inline-values-lines-above")]
pub const LINES_ABOVE: usize = 20;

#[note("inline-values-value-chars")]
pub const VALUE_CHARS: usize = 40;

#[note("inline-values-paths")]
pub fn paths(text: &str, stop_line: usize, locals: &[&str]) -> Vec<(usize, Vec<String>)> {
    // Never above the `fn` the program stands in (asked by the owner on the
    // print, 30/09/2026): a function above the stop did not run in this frame,
    // and values written there would read as if it had.
    let window = stop_line.saturating_sub(LINES_ABOVE);
    let first = text
        .lines()
        .enumerate()
        .take(stop_line + 1)
        .skip(window)
        .filter(|(_, code)| starts_a_fn(code))
        .map(|(line, _)| line)
        .last()
        .unwrap_or(window);
    let mut out = Vec::new();
    for (line, code) in text
        .lines()
        .enumerate()
        .skip(first)
        .take(stop_line + 1 - first)
    {
        let mut found: Vec<String> = Vec::new();
        for path in chains_of(code) {
            let root = path.split('.').next().unwrap_or("");
            let bare_self = path == "self";
            if locals.contains(&root) && !bare_self && !found.contains(&path) {
                found.push(path);
            }
        }
        if !found.is_empty() {
            out.push((line, found));
        }
    }
    out
}

#[note("inline-values-starts-a-fn")]
fn starts_a_fn(code: &str) -> bool {
    let mut rest = code.trim_start();
    loop {
        let before = rest;
        for prefix in ["pub ", "async ", "const ", "unsafe ", "default "] {
            rest = rest.strip_prefix(prefix).unwrap_or(rest);
        }
        if let Some(r) = rest.strip_prefix("pub(") {
            rest = r.split_once(") ").map_or(rest, |(_, after)| after);
        }
        if let Some(r) = rest.strip_prefix("extern ") {
            rest = r.split_once(' ').map_or(rest, |(_, after)| after);
        }
        if rest == before {
            break;
        }
    }
    rest.starts_with("fn ")
}

#[note("inline-values-expressions")]
pub fn expressions(lines: &[(usize, Vec<String>)]) -> Vec<String> {
    let mut all: Vec<String> = Vec::new();
    for (_, paths) in lines {
        for p in paths {
            if !all.contains(p) {
                all.push(p.clone());
            }
        }
    }
    all
}

#[note("inline-values-texts")]
pub fn texts(
    lines: &[(usize, Vec<String>)],
    asked: &[String],
    values: &[ShownVariable],
) -> Vec<(usize, String)> {
    let value_of = |p: &String| {
        let i = asked.iter().position(|a| a == p)?;
        values.get(i).filter(|v| !v.value.starts_with("<error"))
    };
    lines
        .iter()
        .filter_map(|(line, paths)| {
            let shown: Vec<String> = paths
                .iter()
                .filter_map(|p| {
                    value_of(p).map(|v| format!("{p} = {}", cut(&v.value, VALUE_CHARS)))
                })
                .collect();
            (!shown.is_empty()).then(|| (*line, format!("    {}", shown.join(", "))))
        })
        .collect()
}

#[note("inline-values-chains-of")]
fn chains_of(code: &str) -> Vec<String> {
    let chars: Vec<char> = code.chars().collect();
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut out = Vec::new();
    let mut i = 0;
    let mut in_string = false;
    while i < chars.len() {
        let c = chars[i];
        if in_string {
            if c == '\\' {
                i += 2;
                continue;
            }
            in_string = c != '"';
            i += 1;
            continue;
        }
        if c == '"' {
            in_string = true;
            i += 1;
            continue;
        }
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            break;
        }
        if !(c.is_alphabetic() || c == '_') {
            i += 1;
            continue;
        }
        let after_dot = i > 0 && chars[i - 1] == '.';
        let mut parts: Vec<String> = Vec::new();
        loop {
            let start = i;
            while i < chars.len() && ident(chars[i]) {
                i += 1;
            }
            let part: String = chars[start..i].iter().collect();
            if chars.get(i) == Some(&'(') || chars.get(i) == Some(&'!') {
                break; // a call or a macro is not a value
            }
            parts.push(part);
            let more = chars.get(i) == Some(&'.')
                && chars
                    .get(i + 1)
                    .is_some_and(|c| c.is_alphabetic() || *c == '_');
            if !more {
                break;
            }
            i += 1;
        }
        if !after_dot && !parts.is_empty() {
            out.push(parts.join("."));
        }
    }
    out
}

#[note("inline-values-cut")]
fn cut(value: &str, max: usize) -> String {
    let one_line = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.chars().count() <= max {
        return one_line;
    }
    let kept: String = one_line.chars().take(max - 1).collect();
    format!("{}…", kept.trim_end())
}

#[cfg(test)]
#[path = "test/inline_values/test.rs"]
mod tests;
