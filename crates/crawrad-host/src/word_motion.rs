//! Where `Ctrl+Left` and `Ctrl+Right` stop on a line — pure: a line and a
//! byte in, a byte out.

use crawrad_note::note;
#[note("word-motion-class")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    Space,
    /// Letter, digit or `_` — the identifier of any language this IDE writes.
    Word,
    Symbol,
}

fn class(c: char) -> Class {
    if c.is_whitespace() {
        Class::Space
    } else if c.is_alphanumeric() || c == '_' {
        Class::Word
    } else {
        Class::Symbol
    }
}

#[note("word-motion-chars-at")]
fn chars_at(line: &str, byte: usize) -> (Vec<(usize, char)>, usize) {
    let cs: Vec<(usize, char)> = line.char_indices().collect();
    let i = cs.partition_point(|(b, _)| *b < byte);
    (cs, i)
}

#[note("word-motion-word-start-left")]
pub fn word_start_left(line: &str, byte: usize) -> Option<usize> {
    let (cs, mut i) = chars_at(line, byte);
    if i == 0 {
        return None;
    }
    while i > 0 && class(cs[i - 1].1) == Class::Space {
        i -= 1;
    }
    if i == 0 {
        return Some(0);
    }
    let k = class(cs[i - 1].1);
    while i > 0 && class(cs[i - 1].1) == k {
        i -= 1;
    }
    Some(cs.get(i).map_or(line.len(), |(b, _)| *b))
}

#[note("word-motion-word-end-right")]
pub fn word_end_right(line: &str, byte: usize) -> Option<usize> {
    let (cs, mut i) = chars_at(line, byte);
    if i >= cs.len() {
        return None;
    }
    while i < cs.len() && class(cs[i].1) == Class::Space {
        i += 1;
    }
    if i >= cs.len() {
        return Some(line.len());
    }
    let k = class(cs[i].1);
    while i < cs.len() && class(cs[i].1) == k {
        i += 1;
    }
    Some(cs.get(i).map_or(line.len(), |(b, _)| *b))
}

#[note("word-motion-column-of-byte")]
pub fn column_of_byte(line: &str, byte: usize) -> usize {
    let mut n = 1;
    for (i, _) in line.char_indices() {
        if i >= byte {
            return n;
        }
        n += 1;
    }
    n
}

#[note("word-motion-motion")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    /// The START of the word to the left (`cursorWordStartLeft`).
    WordLeft,
    /// The END of the word to the right (`cursorWordEndRight` on Windows and
    /// Linux): walking left lands before a word, walking right after one.
    WordRight,
    FileStart,
    FileEnd,
}

#[note("word-motion-target")]
pub fn target(lines: &[String], (l, b): (usize, usize), m: Motion) -> (usize, usize) {
    if lines.is_empty() {
        return (0, 0);
    }
    let l = l.min(lines.len() - 1);
    match m {
        Motion::FileStart => (0, 0),
        Motion::FileEnd => (lines.len() - 1, lines[lines.len() - 1].len()),
        Motion::WordLeft => match word_start_left(&lines[l], b) {
            Some(n) => (l, n),
            None if l > 0 => (l - 1, lines[l - 1].len()),
            // already at the file's first character: it stays
            None => (0, 0),
        },
        Motion::WordRight => match word_end_right(&lines[l], b) {
            Some(n) => (l, n),
            None if l + 1 < lines.len() => (l + 1, 0),
            None => (l, lines[l].len()),
        },
    }
}

#[cfg(test)]
#[path = "test/word_motion/test.rs"]
mod tests;
