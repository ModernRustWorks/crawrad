// This file is EMITTED VERBATIM into the user's project. Everything in it —
// comments included — ends up in front of whoever opens the generated code, so
// it is English from the first line (ADR 0030). The house rationale, in
// Portuguese, lives in `pieces/mod.rs`, which is never emitted.
//
// Authoring rules, the same ones the egui target's `desenho_comum.rs` carries:
//
// 1. No `//!` and no `#![...]`. This text lands in the MIDDLE of a file, after
//    other items, and an inner attribute after an item is `error[E0753]`.
// 2. No `use`, and no type of ours. Only `std`. On the other side none of our
//    crates exist -- the generated project depends on none of them, and that
//    is the proof this is generation and not an interpreter in disguise.
//    Do not NAME one either, not even in a comment: the gate that proves it
//    reads the whole emitted file, and it is right to.

/// A list of strings, for properties like `Memo.lines`, `ListBox.items` and
/// `ComboBox.items`.
///
/// # It is a newtype over `Vec<String>`, and that is the whole design
///
/// `Deref`/`DerefMut` give it every method a `Vec` has — `iter`, `len`,
/// `is_empty`, `get`, indexing, `sort` — with the types a Rust programmer
/// expects, and without this file listing them one by one. What it adds on top
/// is only what a *list of lines* needs and a `Vec` does not have: [`text`] and
/// [`set_text`].
///
/// Deref is the one shape where the idiom is not abused: a transparent wrapper
/// around a collection is exactly the case `String`/`str` and `Vec`/`[T]`
/// already are in `std`.
///
/// # The names are Rust's, and that is a decision
///
/// `push`, not `add`. No API borrowed from another language, however familiar
/// that language is to whoever wrote the IDE.
///
/// It matters because this text lands **inside the user's own project**.
/// Whoever opens it is reading Rust, and a dialect that exists nowhere else is
/// a cost paid by every person who ever opens a generated project.
///
/// [`text`]: Strings::text
/// [`set_text`]: Strings::set_text
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Strings {
    pub(crate) items: Vec<String>,
}

impl Strings {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from_slice(slice: &[&str]) -> Self {
        Self {
            items: slice.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn from_text(text: &str) -> Self {
        let mut s = Self::new();
        s.set_text(text);
        s
    }

    /// Appends an item.
    ///
    /// It shadows `Vec::push` on purpose, to take anything that becomes a
    /// `String` — `push("a")` instead of `push("a".to_string())`.
    ///
    /// **Every** item that ever enters this list comes through here, including
    /// the ones the designer wrote: one door in is one place to look when
    /// something arrives wrong.
    pub fn push(&mut self, item: impl Into<String>) {
        self.items.push(item.into());
    }

    /// Every line joined with `\n`.
    pub fn text(&self) -> String {
        self.items.join("\n")
    }

    /// Replaces every line from a `\n`-separated string.
    pub fn set_text(&mut self, text: &str) {
        self.items.clear();
        for line in text.lines() {
            self.push(line);
        }
    }
}

impl std::ops::Deref for Strings {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl std::ops::DerefMut for Strings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl From<Vec<String>> for Strings {
    fn from(items: Vec<String>) -> Self {
        Self { items }
    }
}

impl From<&[&str]> for Strings {
    fn from(slice: &[&str]) -> Self {
        Self::from_slice(slice)
    }
}
