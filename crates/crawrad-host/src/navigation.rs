//! **Where the caret was, to go back to** — the history behind *Go to
//! Definition*.
//!
//! Every jump records the place it left; *Back* returns there and keeps the
//! place it left for *Forward*. A new jump after going back drops the
//! forward side, as every browser and editor does. Plain typing or arrow
//! keys record nothing: only jumps are history.
//!
//! It also says which files are **not the user's**: the toolchain's sources
//! (`std`, under `RUSTUP_HOME`) and the crates cargo downloaded (under
//! `CARGO_HOME`). A definition can land there, and those files open read-only.

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("navigation-place")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Place {
    pub file: PathBuf,
    pub line: usize,
    pub byte: usize,
}

#[note("navigation-aids")]
#[derive(Debug, Clone, Default)]
pub struct Aids {
    pub jumps: History,
    pub hover: crate::hover_card::HoverCard,
    /// The parameter card: the same ticket-and-place as the hover's.
    pub signature: crate::hover_card::HoverCard,
    pub choices: Choices,
    pub format: Pending,
    pub references: crate::references::References,
    pub rename: crate::rename::Rename,
    /// The note dialog's Link item….
    pub links: crate::link_pick::LinkPick,
}

impl Aids {
    #[note("navigation-aids-choices-json")]
    pub fn choices_json(&self) -> serde_json::Value {
        match self.links.is_open() {
            true => self.links.to_json(),
            false => self.choices.to_json(),
        }
    }

    #[note("navigation-aids-rows")]
    pub fn rows(&self, files: &[PathBuf]) -> Vec<String> {
        if self.links.is_open() {
            return self.links.labels();
        }
        let name = |p: &PathBuf| {
            p.file_name()
                .unwrap_or(p.as_os_str())
                .to_string_lossy()
                .into()
        };
        let row = |(i, p)| {
            self.choices
                .places
                .get(i)
                .map_or_else(|| name(p), place_label)
        };
        files.iter().enumerate().map(row).collect()
    }
}

#[note("navigation-pending")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pending {
    tickets: u64,
    waiting: Option<(u64, PathBuf, i32)>,
}

impl Pending {
    #[note("navigation-pending-ask")]
    pub fn ask(&mut self, file: PathBuf, version: i32) -> u64 {
        self.tickets += 1;
        self.waiting = Some((self.tickets, file, version));
        self.tickets
    }

    pub fn ticket(&self) -> Option<u64> {
        self.waiting.as_ref().map(|(t, _, _)| *t)
    }

    #[note("navigation-pending-answered")]
    pub fn answered(&mut self, ticket: u64) -> Option<(PathBuf, i32)> {
        match self.waiting.take() {
            Some((t, f, v)) if t == ticket => Some((f, v)),
            other => {
                self.waiting = other;
                None
            }
        }
    }
}

#[note("navigation-choices")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Choices {
    pub places: Vec<Place>,
    pub from: Option<Place>,
}

impl Choices {
    pub fn offer(&mut self, places: Vec<Place>, from: Place) {
        self.places = places;
        self.from = Some(from);
    }

    pub fn is_open(&self) -> bool {
        !self.places.is_empty()
    }

    #[note("navigation-choices-choose")]
    pub fn choose(&mut self, i: usize) -> Option<(Place, Place)> {
        let to = self.places.get(i).cloned()?;
        let from = self.from.take()?;
        self.places.clear();
        Some((to, from))
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    #[note("navigation-choices-to-json")]
    pub fn to_json(&self) -> serde_json::Value {
        let items: Vec<String> = self.places.iter().map(place_label).collect();
        serde_json::json!({ "open": self.is_open(), "items": items })
    }
}

#[note("navigation-place-label")]
pub fn place_label(p: &Place) -> String {
    let name = p
        .file
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_default();
    let folder = p
        .file
        .parent()
        .and_then(Path::file_name)
        .map(|n| n.to_string_lossy());
    match folder {
        Some(f) => format!("{name}:{}  {f}", p.line + 1),
        None => format!("{name}:{}", p.line + 1),
    }
}

#[note("navigation-depth")]
pub const DEPTH: usize = 50;

#[note("navigation-history")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History {
    back: Vec<Place>,
    forward: Vec<Place>,
    /// The ticket of the newest *Go to Definition* asked and where it was
    /// asked from; an answer under another ticket is stale.
    pending: Option<(u64, Place)>,
    tickets: u64,
}

impl History {
    #[note("navigation-history-ask")]
    pub fn ask(&mut self, from: Place) -> u64 {
        self.tickets += 1;
        self.pending = Some((self.tickets, from));
        self.tickets
    }

    #[note("navigation-history-pending-ticket")]
    pub fn pending_ticket(&self) -> Option<u64> {
        self.pending.as_ref().map(|(t, _)| *t)
    }

    #[note("navigation-history-answered")]
    pub fn answered(&mut self, ticket: u64) -> Option<Place> {
        match self.pending.take() {
            Some((t, from)) if t == ticket => Some(from),
            other => {
                self.pending = other;
                None
            }
        }
    }

    #[note("navigation-history-jumped-from")]
    pub fn jumped_from(&mut self, from: Place) {
        if self.back.last() != Some(&from) {
            self.back.push(from);
            if self.back.len() > DEPTH {
                self.back.remove(0);
            }
        }
        self.forward.clear();
    }

    #[note("navigation-history-back")]
    pub fn back(&mut self, here: Place) -> Option<Place> {
        let to = self.back.pop()?;
        self.forward.push(here);
        Some(to)
    }

    #[note("navigation-history-forward")]
    pub fn forward(&mut self, here: Place) -> Option<Place> {
        let to = self.forward.pop()?;
        self.back.push(here);
        Some(to)
    }

    pub fn can_go_back(&self) -> bool {
        !self.back.is_empty()
    }

    pub fn can_go_forward(&self) -> bool {
        !self.forward.is_empty()
    }
}

#[note("navigation-word-at")]
pub fn word_at(line: &str, byte: usize) -> &str {
    let is_name = |c: char| c.is_alphanumeric() || c == '_';
    let mut byte = byte.min(line.len());
    while !line.is_char_boundary(byte) {
        byte -= 1;
    }
    let start = line[..byte]
        .char_indices()
        .rev()
        .take_while(|(_, c)| is_name(*c))
        .last()
        .map_or(byte, |(i, _)| i);
    let end = line[byte..]
        .find(|c| !is_name(c))
        .map_or(line.len(), |i| byte + i);
    &line[start..end]
}

#[note("navigation-foreign-roots")]
pub fn foreign_roots() -> Vec<PathBuf> {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from);
    let pick = |var: &str, default: &str| {
        std::env::var_os(var)
            .map(PathBuf::from)
            .or_else(|| home.as_ref().map(|h| h.join(default)))
    };
    let cargo = pick("CARGO_HOME", ".cargo");
    let mut roots: Vec<PathBuf> = pick("RUSTUP_HOME", ".rustup").into_iter().collect();
    if let Some(c) = cargo {
        roots.push(c.join("registry"));
        roots.push(c.join("git"));
    }
    roots
}

#[note("navigation-is-foreign")]
pub fn is_foreign(file: &Path, roots: &[PathBuf]) -> bool {
    let norm = |p: &Path| p.to_string_lossy().replace('\\', "/").to_lowercase();
    let f = norm(file);
    roots.iter().any(|r| {
        let r = norm(r);
        let r = r.trim_end_matches('/');
        !r.is_empty() && f.starts_with(r) && f[r.len()..].starts_with('/')
    })
}

#[note("navigation-locked")]
pub fn locked(file: &Path, roots: &[PathBuf], generated_suffix: &str) -> bool {
    let generated = file
        .file_name()
        .is_some_and(|n| n.to_string_lossy().ends_with(generated_suffix));
    generated || is_foreign(file, roots)
}

#[note("navigation-tab-icon")]
pub fn tab_icon(file: &Path, roots: &[PathBuf], generated_suffix: &str) -> &'static str {
    if locked(file, roots, generated_suffix) {
        "\u{1F512} "
    } else if file.to_string_lossy().ends_with(".rs") {
        "\u{1F980} "
    } else {
        "\u{1F4C4} "
    }
}

#[note("navigation-is-locked")]
pub fn is_locked(file: &Path, generated_suffix: &str) -> bool {
    locked(file, &foreign_roots(), generated_suffix)
}

#[note("navigation-pousocodigo")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PousoCodigo {
    pub linha: usize,      // 0-based line index
    pub start_byte: usize, // 0-based start byte
    pub end_byte: usize,   // 0-based end byte (seleciona intervalo se > start_byte)
}

impl PousoCodigo {
    #[note("navigation-pousocodigo-at")]
    pub fn at(linha: usize, byte: usize) -> Self {
        Self {
            linha,
            start_byte: byte,
            end_byte: byte,
        }
    }

    pub fn cursor(linha_1based: usize, col_1based: usize) -> Self {
        let linha = linha_1based.saturating_sub(1);
        let col = col_1based.saturating_sub(1);
        Self {
            linha,
            start_byte: col,
            end_byte: col,
        }
    }

    pub fn selection(linha_1based: usize, start_byte: usize, end_byte: usize) -> Self {
        Self {
            linha: linha_1based.saturating_sub(1),
            start_byte,
            end_byte,
        }
    }
}

#[cfg(test)]
#[path = "test/navigation/test.rs"]
mod tests;
