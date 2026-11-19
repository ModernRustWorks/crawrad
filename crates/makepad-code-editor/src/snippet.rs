//! **A snippet's tabstops while it is being filled**: the ranges Tab
//! visits in order, kept in the document so they follow every edit — typing
//! in `$1` moves `$2` and `$0` with it.

use crate::text::{Edit, Position};

/// The stops of the snippet being filled, and the next one Tab selects.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SnippetStops {
    stops: Vec<(Position, Position)>,
    next: usize,
}

impl SnippetStops {
    /// A snippet was written: its stops in tab order (`$0` last).
    pub fn start(&mut self, stops: Vec<(Position, Position)>) {
        (self.stops, self.next) = (stops, 0);
    }

    /// Whether Tab still has a stop to go to.
    pub fn active(&self) -> bool {
        self.next < self.stops.len()
    }

    /// The next stop to select; after the last (`$0`) the snippet is done.
    pub fn advance(&mut self) -> Option<(Position, Position)> {
        let stop = self.stops.get(self.next).copied()?;
        self.next += 1;
        Some(stop)
    }

    /// Esc, or the snippet is left: no stops.
    pub fn end(&mut self) {
        *self = Self::default();
    }

    /// The text changed: every stop moves with it.
    pub fn apply_edit(&mut self, edit: &Edit) {
        for (start, end) in &mut self.stops {
            *start = start.apply_edit(edit);
            *end = end.apply_edit(edit);
        }
    }
}

#[cfg(test)]
#[path = "test/snippet/test.rs"]
mod tests;
