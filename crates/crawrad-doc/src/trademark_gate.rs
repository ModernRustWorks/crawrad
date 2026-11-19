//! **The gate that keeps other vendors' marks out of CrawRAD.**
//!
//! The owner's rule (01/10/2026): the name of another vendor's IDE appears
//! nowhere in this project except as the name of a key scheme, because it can
//! bring legal trouble. On that day the repository carried 1,562 mentions of
//! it, its vendor and its component library, in code comments, ADRs,
//! studies and memory — no one had counted, so every PR could add one more.
//!
//! This gate counts them in every file git tracks and only lets the number go
//! DOWN, with equality enforced like the shell's line ratchet: a PR that
//! removes mentions lowers [`MAX_TRADEMARK_MENTIONS`] in the same commit, and
//! a PR that adds one fails. The sweep itself goes folder by folder; the floor
//! is zero, since the key scheme is called "Classic". A file's own path
//! counts too, so a note cannot carry the name in its file name.
//!
//! The marks are spelled here only once, split, so this file does not count
//! itself: [`marks`] joins them at run time.

use std::path::Path;

/// 316 on 01/10/2026; zero. Only goes down, and it is at the floor.
pub const MAX_TRADEMARK_MENTIONS: usize = 0;

/// How a mark must sit in the text to count.
#[derive(Clone, Copy)]
enum Fit {
    /// Anywhere, even inside a word.
    Anywhere,
    /// As a whole word only: as a fragment it is in ordinary words.
    WholeWord,
    /// Anywhere except right after `rust`, where it is our own product name.
    NotOurs,
}

/// The marks counted, lower case: the IDE, its vendor (old and new name), its
/// product line, its two UI libraries, its data-access library and its
/// classes, and its test framework.
fn marks() -> [(String, Fit); 11] {
    [
        (["del", "phi"].concat(), Fit::Anywhere),
        (["embar", "cadero"].concat(), Fit::Anywhere),
        (["bor", "land"].concat(), Fit::Anywhere),
        (["rad st", "udio"].concat(), Fit::NotOurs),
        (["v", "cl"].concat(), Fit::WholeWord),
        (["f", "mx"].concat(), Fit::WholeWord),
        (["fire", "monkey"].concat(), Fit::Anywhere),
        (["fire", "dac"].concat(), Fit::Anywhere),
        (["tfd", "phys"].concat(), Fit::Anywhere),
        (["tfd", "connection"].concat(), Fit::Anywhere),
        (["dun", "itx"].concat(), Fit::Anywhere),
    ]
}

/// How many times the marks appear in `text`, ignoring case.
pub fn mentions_in(text: &str) -> usize {
    let lower = text.to_lowercase();
    let bytes = lower.as_bytes();
    let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    marks()
        .iter()
        .map(|(mark, fit)| {
            lower
                .match_indices(mark.as_str())
                .filter(|(at, _)| match fit {
                    Fit::Anywhere => true,
                    Fit::WholeWord => {
                        (*at == 0 || !is_word(bytes[at - 1]))
                            && bytes.get(at + mark.len()).is_none_or(|&b| !is_word(b))
                    }
                    Fit::NotOurs => !lower[..*at].ends_with("craw"),
                })
                .count()
        })
        .sum()
}

/// Every mention in the files of `files` under `root`, by file, counting the
/// file's own path too. A file that is not text counts only its path.
pub fn mentions_by_file(root: &Path, files: &[String]) -> Vec<(String, usize)> {
    files
        .iter()
        .filter_map(|f| {
            let text = std::fs::read_to_string(root.join(f)).unwrap_or_default();
            let n = mentions_in(f) + mentions_in(&text);
            (n > 0).then(|| (f.clone(), n))
        })
        .collect()
}

#[cfg(test)]
#[path = "test/trademark_gate/test.rs"]
mod tests;
