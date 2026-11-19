//! **The References panel**: what *Find All References* found,
//! grouped by file, ready to draw.
//!
//! The language server's answer arrives already in the editor's unit (a
//! file, a line and the symbol's bytes in the line's text — see
//! `crawrad_lsp::references::spans`). Here it becomes the panel: one group per
//! file with its count, a row per occurrence with the line cut around the
//! name, and the marks a RAD project needs — an occurrence in the
//! generated form source is **generated** (the line is not the user's: the
//! form is where it changes), one in the toolchain or a downloaded crate is
//! **read-only**. Both open locked, by the rule of Go to Definition.

use std::path::{Path, PathBuf};

use crate::navigation::{Pending, Place, locked};
use crawrad_note::note;

#[note("references-hit")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: String,
}

impl Hit {
    pub fn new(line: usize, start: usize, end: usize, text: String) -> Self {
        Self {
            line,
            start,
            end,
            text,
        }
    }
}

#[note("references-group")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub file: PathBuf,
    /// The generated form source: the line belongs to the form.
    pub generated: bool,
    /// Not the user's to edit (toolchain, downloaded crate, generated).
    pub read_only: bool,
    pub open: bool,
    pub hits: Vec<Hit>,
}

#[note("references-row")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    /// A file: its name, its folder, how many, its mark, open or closed.
    File {
        group: usize,
        name: String,
        folder: String,
        count: usize,
        mark: Option<&'static str>,
        open: bool,
    },
    /// An occurrence: the 1-based line, and the line cut in three — before
    /// the name, the name, after it.
    Hit {
        group: usize,
        hit: usize,
        line: String,
        before: String,
        word: String,
        after: String,
    },
}

impl Row {
    #[note("references-row-template")]
    pub fn template(&self) -> &'static str {
        match self {
            Self::File { .. } => "RefFile",
            Self::Hit { .. } => "RefHit",
        }
    }
}

#[note("references-before")]
pub const BEFORE: usize = 16;
pub const AFTER: usize = 80;

#[note("references-references")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct References {
    pub symbol: String,
    pub groups: Vec<Group>,
    /// The request waiting for its answer (the newest only).
    pub pending: Pending,
    /// Asked and answered at least once: the panel has something to say.
    pub answered: bool,
    /// Where each drawn row is in the window (row, x, y, w, h), written by
    /// the window as it draws — what MCP gives a test to click.
    pub rects: Vec<(usize, [f64; 4])>,
}

impl References {
    #[note("references-references-show")]
    pub fn show(
        &mut self,
        symbol: &str,
        hits: Vec<(PathBuf, Hit)>,
        roots: &[PathBuf],
        generated_suffix: &str,
    ) {
        let mut groups: Vec<Group> = Vec::new();
        for (file, hit) in hits {
            match groups.iter_mut().find(|g| g.file == file) {
                Some(g) => g.hits.push(hit),
                None => groups.push(Group {
                    generated: is_generated(&file, generated_suffix),
                    read_only: locked(&file, roots, generated_suffix),
                    open: true,
                    hits: vec![hit],
                    file,
                }),
            }
        }
        groups.sort_by(|a, b| a.file.cmp(&b.file));
        for g in &mut groups {
            g.hits.sort_by_key(|h| (h.line, h.start));
            g.hits.dedup();
        }
        self.symbol = symbol.to_string();
        self.groups = groups;
        self.answered = true;
    }

    pub fn total(&self) -> usize {
        self.groups.iter().map(|g| g.hits.len()).sum()
    }

    #[note("references-references-title")]
    pub fn title(&self) -> String {
        match (self.answered, self.total(), self.groups.len()) {
            (false, ..) => "Find All References: put the caret on a name".into(),
            (true, 0, _) => format!("No references to `{}`", self.symbol),
            (true, 1, _) => format!("`{}` \u{2014} 1 reference", self.symbol),
            (true, n, 1) => format!("`{}` \u{2014} {n} references in 1 file", self.symbol),
            (true, n, f) => format!("`{}` \u{2014} {n} references in {f} files", self.symbol),
        }
    }

    #[note("references-references-toggle")]
    pub fn toggle(&mut self, group: usize) {
        if let Some(g) = self.groups.get_mut(group) {
            g.open = !g.open;
        }
    }

    #[note("references-references-rows")]
    pub fn rows(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for (gi, g) in self.groups.iter().enumerate() {
            rows.push(Row::File {
                group: gi,
                name: name_of(&g.file),
                folder: folder_of(&g.file),
                count: g.hits.len(),
                mark: match (g.generated, g.read_only) {
                    (true, _) => Some("generated"),
                    (false, true) => Some("read-only"),
                    _ => None,
                },
                open: g.open,
            });
            if !g.open {
                continue;
            }
            for (hi, h) in g.hits.iter().enumerate() {
                let (before, word, after) = cut(h);
                rows.push(Row::Hit {
                    group: gi,
                    hit: hi,
                    line: (h.line + 1).to_string(),
                    before,
                    word,
                    after,
                });
            }
        }
        rows
    }

    #[note("references-references-place")]
    pub fn place(&self, group: usize, hit: usize) -> Option<Place> {
        let g = self.groups.get(group)?;
        let h = g.hits.get(hit)?;
        Some(Place {
            file: g.file.clone(),
            line: h.line,
            byte: h.start,
        })
    }

    #[note("references-references-to-json")]
    pub fn to_json(&self) -> serde_json::Value {
        let files: Vec<serde_json::Value> = self
            .groups
            .iter()
            .map(|g| {
                let hits: Vec<serde_json::Value> = g
                    .hits
                    .iter()
                    .map(|h| {
                        let (before, word, after) = cut(h);
                        serde_json::json!({ "line": h.line + 1, "word": word, "text": format!("{before}{word}{after}") })
                    })
                    .collect();
                serde_json::json!({
                    "file": g.file.display().to_string(),
                    "count": g.hits.len(),
                    "generated": g.generated,
                    "read_only": g.read_only,
                    "open": g.open,
                    "hits": hits,
                })
            })
            .collect();
        serde_json::json!({
            "symbol": self.symbol,
            "title": self.title(),
            "total": self.total(),
            "waiting": self.pending.ticket().is_some(),
            "files": files,
            "rows": self.rects.iter().map(|(i, r)| serde_json::json!({ "row": i, "rect": r })).collect::<Vec<_>>(),
        })
    }
}

#[note("references-same-file")]
pub fn same_file(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| p.to_string_lossy().replace('\\', "/").to_lowercase();
    norm(a) == norm(b)
}

fn is_generated(file: &Path, generated_suffix: &str) -> bool {
    file.file_name()
        .is_some_and(|n| n.to_string_lossy().ends_with(generated_suffix))
}

fn name_of(file: &Path) -> String {
    file.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default()
}

fn folder_of(file: &Path) -> String {
    file.parent()
        .and_then(Path::file_name)
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default()
}

#[note("references-cut")]
pub fn cut(h: &Hit) -> (String, String, String) {
    let text = h.text.as_str();
    let start = floor(text, h.start.min(text.len()));
    let end = floor(text, h.end.clamp(start, text.len()));
    let before = text[..start].trim_start();
    let before: String = match before.chars().count() > BEFORE {
        true => {
            let skip = before.chars().count() - BEFORE;
            format!("\u{2026}{}", before.chars().skip(skip).collect::<String>())
        }
        false => before.to_string(),
    };
    let after = &text[end..];
    let after: String = match after.chars().count() > AFTER {
        true => format!("{}\u{2026}", after.chars().take(AFTER).collect::<String>()),
        false => after.to_string(),
    };
    (before, text[start..end].to_string(), after)
}

#[note("references-floor")]
fn floor(text: &str, mut b: usize) -> usize {
    while !text.is_char_boundary(b) {
        b -= 1;
    }
    b
}

#[cfg(test)]
#[path = "test/references/test.rs"]
mod tests;
