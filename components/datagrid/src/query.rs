//! Query specification and builder API as plain data contract.

use crate::filter::{Filter, Owned};
use crate::group::Total;
use crate::sort::{Nulls, Order, SortKey};
use std::collections::HashSet;

/// Grouping hierarchy level with associated aggregate totals.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupLevel {
    pub column: String,
    pub totals: Vec<Total>,
}

impl GroupLevel {
    /// Creates a new grouping level for `column` with no totals.
    pub fn new(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            totals: Vec::new(),
        }
    }
}

/// Query specification defining filter predicates, sorting keys, and grouping hierarchy with per-level totals.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Query {
    pub filter: Option<Filter>,
    pub sort: Vec<SortKey>,
    pub group_by: Vec<GroupLevel>,
    pub summary: Vec<Total>,
}

impl Query {
    /// Creates a new empty query specification.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the filter predicate.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Appends an ascending sort key with nulls placed last.
    pub fn sort_asc(mut self, column: impl Into<String>) -> Self {
        self.sort.push(SortKey {
            column: column.into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        });
        self
    }

    /// Appends a descending sort key with nulls placed last.
    pub fn sort_desc(mut self, column: impl Into<String>) -> Self {
        self.sort.push(SortKey {
            column: column.into(),
            order: Order::Desc,
            nulls: Nulls::Last,
        });
        self
    }

    /// Appends an ascending sort key with nulls placed first.
    pub fn sort_asc_nulls_first(mut self, column: impl Into<String>) -> Self {
        self.sort.push(SortKey {
            column: column.into(),
            order: Order::Asc,
            nulls: Nulls::First,
        });
        self
    }

    /// Appends a descending sort key with nulls placed first.
    pub fn sort_desc_nulls_first(mut self, column: impl Into<String>) -> Self {
        self.sort.push(SortKey {
            column: column.into(),
            order: Order::Desc,
            nulls: Nulls::First,
        });
        self
    }

    /// Appends a grouping level for `column`.
    pub fn group(mut self, column: impl Into<String>) -> Self {
        self.group_by.push(GroupLevel::new(column));
        self
    }

    /// Adds an aggregate total to the last grouping level.
    ///
    /// # Panics
    /// Panics if no grouping level has been added yet via [`Query::group`].
    pub fn with_total(mut self, total: Total) -> Self {
        let last = self
            .group_by
            .last_mut()
            .expect("Query::with_total called without any group level; call .group(col) first");
        last.totals.push(total);
        self
    }

    /// Returns a normalized form of this query:
    /// - The filter is normalized per [`Filter::normalized`].
    /// - When sort keys repeat the same column, the first occurrence is kept and subsequent ones are dropped.
    pub fn normalized(&self) -> Query {
        let filter = self.filter.as_ref().map(Filter::normalized);
        let mut seen_cols = HashSet::new();
        let mut sort = Vec::with_capacity(self.sort.len());
        for key in &self.sort {
            if seen_cols.insert(key.column.clone()) {
                sort.push(key.clone());
            }
        }
        Query {
            filter,
            sort,
            group_by: self.group_by.clone(),
            summary: self.summary.clone(),
        }
    }

    /// Returns a copy of the query with its filter scoped within the specified group column values.
    ///
    /// For each `(column, value)` pair:
    /// - `Some(owned)` adds an `Eq` predicate: `col == owned`.
    /// - `None` adds an `IsNull` predicate: `col IS NULL`.
    pub fn within_group<S: AsRef<str>>(&self, columns_and_values: &[(S, Option<Owned>)]) -> Query {
        if columns_and_values.is_empty() {
            return self.clone();
        }

        let mut group_filters = Vec::with_capacity(columns_and_values.len());
        for (col, val) in columns_and_values {
            let col_str = col.as_ref().to_string();
            let f = match val {
                Some(v) => Filter::Eq(col_str, v.clone()),
                None => Filter::IsNull(col_str),
            };
            group_filters.push(f);
        }

        let combined = if group_filters.len() == 1 {
            group_filters.into_iter().next().expect("not empty")
        } else {
            Filter::And(group_filters)
        };

        let filter = match &self.filter {
            Some(existing) => Some(existing.clone().and(combined)),
            None => Some(combined),
        };

        Query {
            filter,
            sort: self.sort.clone(),
            group_by: self.group_by.clone(),
            summary: self.summary.clone(),
        }
    }
}

#[cfg(test)]
#[path = "test/query/test.rs"]
mod test;
