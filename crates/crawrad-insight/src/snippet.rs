//! **LSP snippets expanded**: the text to write and where its
//! tabstops are, so the editor can select `$1`, then `$2` on Tab, and end on
//! `$0`. Pure.
//!
//! Grammar (LSP 3.17 *Snippet Syntax*): `$1`, `${1}`, `${1:placeholder}`
//! (placeholders nest), `${1|one,two|}` (the first choice is written), `$0`
//! the final caret, variables `$NAME` / `${NAME:default}` (the default, or
//! nothing), and `\` escaping `$`, `}`, `\`, `,` and `|`. Measured on
//! rust-analyzer (2026-10-04): `match $1 {\n    $0\n}` and
//! `soma(${1:a}, ${2:b})$0`.
//!
//! Each line after the first gets the indentation of the line it is written
//! on, as a modern editor does: `match` typed four spaces in keeps its body inside.

use crawrad_note::note;
#[note("snippet-expanded")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expanded {
    pub text: String,
    pub stops: Vec<(usize, usize)>,
}

#[note("snippet-expand")]
pub fn expand(snippet: &str, indent: &str) -> Expanded {
    let chars: Vec<char> = snippet.chars().collect();
    let mut out = Out {
        text: String::new(),
        indent,
        found: Vec::new(),
    };
    let mut i = 0;
    parse(&chars, &mut i, &mut out, false);
    let mut numbered: Vec<(u32, (usize, usize))> = Vec::new();
    for (n, range) in out.found {
        if !numbered.iter().any(|(m, _)| *m == n) {
            numbered.push((n, range));
        }
    }
    numbered.sort_by_key(|(n, _)| if *n == 0 { u32::MAX } else { *n });
    if !numbered.iter().any(|(n, _)| *n == 0) {
        let end = out.text.len();
        numbered.push((0, (end, end)));
    }
    Expanded {
        text: out.text,
        stops: numbered.into_iter().map(|(_, r)| r).collect(),
    }
}

struct Out<'a> {
    text: String,
    indent: &'a str,
    /// Every tabstop met, in text order: (number, byte range).
    found: Vec<(u32, (usize, usize))>,
}

impl Out<'_> {
    fn push(&mut self, c: char) {
        self.text.push(c);
        if c == '\n' {
            self.text.push_str(self.indent);
        }
    }
}

#[note("snippet-parse")]
fn parse(chars: &[char], i: &mut usize, out: &mut Out<'_>, nested: bool) {
    while *i < chars.len() {
        let c = chars[*i];
        match c {
            '\\' if chars.get(*i + 1).is_some_and(|n| "$}\\,|".contains(*n)) => {
                out.push(chars[*i + 1]);
                *i += 2;
            }
            '}' if nested => return,
            '$' => {
                *i += 1;
                dollar(chars, i, out);
            }
            _ => {
                out.push(c);
                *i += 1;
            }
        }
    }
}

#[note("snippet-dollar")]
fn dollar(chars: &[char], i: &mut usize, out: &mut Out<'_>) {
    let digits = |i: &mut usize| {
        let start = *i;
        while chars.get(*i).is_some_and(char::is_ascii_digit) {
            *i += 1;
        }
        chars[start..*i]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .ok()
    };
    let name = |i: &mut usize| {
        let start = *i;
        while chars
            .get(*i)
            .is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_')
        {
            *i += 1;
        }
        *i > start
    };
    if let Some(n) = digits(i) {
        let at = out.text.len();
        out.found.push((n, (at, at)));
        return;
    }
    if chars.get(*i) != Some(&'{') {
        if !name(i) {
            out.push('$');
        }
        return;
    }
    *i += 1;
    let start = out.text.len();
    let number = digits(i);
    if number.is_none() && !name(i) {
        out.push('$');
        out.push('{');
        return;
    }
    match chars.get(*i) {
        Some(':') => {
            *i += 1;
            parse(chars, i, out, true);
        }
        Some('|') => {
            *i += 1;
            let mut first = true;
            while *i < chars.len() && !(chars[*i] == '|' && chars.get(*i + 1) == Some(&'}')) {
                match chars[*i] {
                    '\\' if *i + 1 < chars.len() => {
                        if first {
                            out.push(chars[*i + 1]);
                        }
                        *i += 2;
                    }
                    ',' => {
                        first = false;
                        *i += 1;
                    }
                    c => {
                        if first {
                            out.push(c);
                        }
                        *i += 1;
                    }
                }
            }
            *i += 1;
        }
        _ => {}
    }
    if chars.get(*i) == Some(&'}') {
        *i += 1;
    }
    if let Some(n) = number {
        out.found.push((n, (start, out.text.len())));
    }
}

#[note("snippet-placed")]
pub fn placed(
    text: &str,
    stops: &[(usize, usize)],
    (line, byte): (usize, usize),
    lines: &[String],
) -> Vec<[(usize, usize); 2]> {
    let written: Vec<&str> = text.split('\n').collect();
    let ws = |l: &str| l.len() - l.trim_start().len();
    let at = |offset: usize| {
        let before = &text[..offset];
        let k = before.matches('\n').count();
        let col = before.rfind('\n').map_or(offset, |nl| offset - nl - 1);
        if k == 0 {
            return (line, byte + col);
        }
        let real = lines.get(line + k).map_or("", String::as_str);
        let shift = ws(real) as isize - ws(written.get(k).copied().unwrap_or("")) as isize;
        let mut b = (col as isize + shift).clamp(0, real.len() as isize) as usize;
        while !real.is_char_boundary(b) {
            b -= 1;
        }
        (line + k, b)
    };
    stops.iter().map(|(a, b)| [at(*a), at(*b)]).collect()
}

#[cfg(test)]
#[path = "test/snippet/test.rs"]
mod tests;
