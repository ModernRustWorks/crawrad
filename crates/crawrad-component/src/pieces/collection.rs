// This file is EMITTED VERBATIM into the user's project. Everything in it —
// comments included — ends up in front of whoever opens the generated code, so
// it is English from the first line (ADR 0030). The house rationale, in
// Portuguese, lives in `pieces/mod.rs`, which is never emitted.
//
// Authoring rules, the same ones the egui target's `desenho_comum.rs` carries:
//
// 1. No `//!` and no `#![...]`. This text lands in the MIDDLE of a file, after
//    other items, and an inner attribute after an item is `error[E0753]`.
// 2. No `use`, and no type of ours. Only `std` or `core`. On the other side
//    none of our crates exist -- the generated project depends on none of them,
//    and that is the proof this is generation and not an interpreter in
//    disguise. Do not NAME one either, not even in a comment: the gate that
//    proves it reads the whole emitted file, and it is right to.

/// A typed collection of items, for properties like `Grid.columns`,
/// `StatusBar.panels` and `HeaderControl.sections`.
///
/// # It is a newtype over `Vec<T>`, and that is the whole design
///
/// `Deref` and `DerefMut` give it slice and indexing methods that a `Vec`
/// provides — `iter`, `len`, `is_empty`, `get`, indexing — with the types a
/// Rust programmer expects.
///
/// Deref is the one shape where the idiom is not abused: a transparent wrapper
/// around a collection is exactly the case `String`/`str` and `Vec`/`[T]`
/// already are in `std`.
///
/// # The names are Rust's, and that is a decision
///
/// `push`, not `add`. No API borrowed from another language, however familiar
/// that language is to whoever wrote the IDE.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Collection<T> {
    pub(crate) items: Vec<T>,
}

impl<T> Collection<T> {
    /// Creates an empty collection.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Creates a collection with the given items.
    pub fn with_items(items: Vec<T>) -> Self {
        Self { items }
    }

    /// Appends an item to the back of the collection.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Returns the number of items in the collection.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if the collection has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns a reference to the item at the given index, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Returns a mutable reference to the item at the given index, or `None` if out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    /// Removes and returns the item at the given index.
    pub fn remove(&mut self, index: usize) -> T {
        self.items.remove(index)
    }

    /// Clears the collection, removing all items.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the items.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.items.iter()
    }

    /// Returns a mutable iterator over the items.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.items.iter_mut()
    }

    /// Extracts a slice over the entire collection.
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    /// Extracts a mutable slice over the entire collection.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.items
    }

    /// Consumes the collection, returning the underlying vector.
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T> core::ops::Deref for Collection<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T> core::ops::DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<T> From<Vec<T>> for Collection<T> {
    fn from(items: Vec<T>) -> Self {
        Self { items }
    }
}

impl<T> From<Collection<T>> for Vec<T> {
    fn from(c: Collection<T>) -> Self {
        c.items
    }
}

impl<T> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Collection<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Collection<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl<T> core::iter::FromIterator<T> for Collection<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}
