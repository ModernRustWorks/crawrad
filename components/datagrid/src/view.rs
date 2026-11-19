//! Hierarchical group tree views with expand/collapse state and incremental requery.

use crate::column::Column;
use crate::group::Aggregate;
pub use crate::query::{GroupLevel, Query};
use crate::sort::{Order, SortKey};
use crate::store::{Store, StoreError};
use std::collections::HashSet;

/// Statistics tracking incremental execution reuse across queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReuseStats {
    pub filter_runs: usize,
    pub sort_runs: usize,
    pub group_runs: usize,
}

/// A node in the hierarchical group tree.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupNode {
    /// One ordinal per level down to this node (as `Group.key`), so a header can
    /// show `store.key_value(column, ordinal)`.
    pub key: Vec<Option<u64>>,
    pub level: usize,
    pub row_count: usize,
    pub totals: Vec<Option<f64>>,
    pub children: Vec<GroupNode>,
    /// The row ids of this group in the view's sort order (only on the last level).
    pub rows: Vec<u32>,
}

/// An interactive query result view managing filtered selections, sorted rows,
/// hierarchical group trees, and expand/collapse state with incremental requery.
#[derive(Debug, Clone)]
pub struct View {
    query: Query,
    filtered_rows: Vec<u32>,
    sorted_rows: Vec<u32>,
    groups: Vec<GroupNode>,
    expanded: HashSet<Vec<Option<u64>>>,
    stats: ReuseStats,
}

impl View {
    /// Evaluates a query against `store`, constructing a new `View`.
    pub fn new(store: &Store, query: Query) -> Result<View, StoreError> {
        validate_query(store, &query)?;

        let filtered_rows: Vec<u32> = match &query.filter {
            Some(f) => {
                let bm = store.filter(f)?;
                bm.ones().collect()
            }
            None => (0..store.rows() as u32).collect(),
        };

        let sorted_rows = store.sort(&filtered_rows, &query.sort)?;

        let groups = build_groups(store, &sorted_rows, &query.group_by, &query.sort)?;

        Ok(View {
            query,
            filtered_rows,
            sorted_rows,
            groups,
            expanded: HashSet::new(),
            stats: ReuseStats {
                filter_runs: 1,
                sort_runs: 1,
                group_runs: 1,
            },
        })
    }

    /// The current active query.
    #[inline]
    pub fn query(&self) -> &Query {
        &self.query
    }

    /// Number of data rows that pass the filter.
    #[inline]
    pub fn row_count(&self) -> usize {
        self.sorted_rows.len()
    }

    /// The filtered and sorted row ids (no group headers).
    #[inline]
    pub fn rows(&self) -> &[u32] {
        &self.sorted_rows
    }

    /// The group tree's top level (empty when `group_by` is empty).
    #[inline]
    pub fn groups(&self) -> &[GroupNode] {
        &self.groups
    }

    /// Returns whether the group node at `path` is currently expanded.
    /// Returns `false` when `path` does not exist.
    pub fn is_expanded(&self, path: &[usize]) -> bool {
        self.node_at_path(path)
            .is_some_and(|node| self.expanded.contains(&node.key))
    }

    /// Returns whether the group node with `key` is currently expanded.
    #[inline]
    pub fn is_key_expanded(&self, key: &[Option<u64>]) -> bool {
        self.expanded.contains(key)
    }

    /// Expands or collapses the group at `path` (indexes from the top level
    /// down). Returns false when the path does not exist.
    pub fn set_expanded(&mut self, path: &[usize], expanded: bool) -> bool {
        if let Some(node) = self.node_at_path(path) {
            let key = node.key.clone();
            if expanded {
                self.expanded.insert(key);
            } else {
                self.expanded.remove(&key);
            }
            true
        } else {
            false
        }
    }

    /// Expands all group nodes in the current group tree.
    pub fn expand_all(&mut self) {
        fn collect_keys(nodes: &[GroupNode], set: &mut HashSet<Vec<Option<u64>>>) {
            for node in nodes {
                set.insert(node.key.clone());
                collect_keys(&node.children, set);
            }
        }
        collect_keys(&self.groups, &mut self.expanded);
    }

    /// Collapses all groups.
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }

    /// Re-runs with a changed query, reusing what it can.
    ///
    /// Note: assumes the same `Store` instance (the same dictionary codes).
    pub fn requery(&mut self, store: &Store, query: Query) -> Result<(), StoreError> {
        if self.query == query {
            return Ok(());
        }

        validate_query(store, &query)?;

        let filter_changed = self.query.filter != query.filter;
        let sort_changed = self.query.sort != query.sort;
        let group_changed = self.query.group_by != query.group_by;
        // an expanded key names ordinals of the old grouping columns: under
        // other columns the same numbers would open unrelated groups
        let columns_changed = self.query.group_by.len() != query.group_by.len()
            || self
                .query
                .group_by
                .iter()
                .zip(&query.group_by)
                .any(|(a, b)| a.column != b.column);
        if columns_changed {
            self.expanded.clear();
        }

        if filter_changed {
            let new_filtered: Vec<u32> = match &query.filter {
                Some(f) => {
                    let bm = store.filter(f)?;
                    bm.ones().collect()
                }
                None => (0..store.rows() as u32).collect(),
            };

            let new_sorted = store.sort(&new_filtered, &query.sort)?;

            let new_groups = build_groups(store, &new_sorted, &query.group_by, &query.sort)?;

            self.filtered_rows = new_filtered;
            self.sorted_rows = new_sorted;
            self.groups = new_groups;
            self.query = query;
            self.stats.filter_runs += 1;
            self.stats.sort_runs += 1;
            self.stats.group_runs += 1;
        } else if sort_changed {
            let new_sorted = store.sort(&self.filtered_rows, &query.sort)?;

            let new_groups = build_groups(store, &new_sorted, &query.group_by, &query.sort)?;

            self.sorted_rows = new_sorted;
            self.groups = new_groups;
            self.query = query;
            self.stats.sort_runs += 1;
            self.stats.group_runs += 1;
        } else if group_changed {
            let new_groups = build_groups(store, &self.sorted_rows, &query.group_by, &query.sort)?;

            self.groups = new_groups;
            self.query = query;
            self.stats.group_runs += 1;
        }

        Ok(())
    }

    /// Returns the reuse performance counters.
    #[inline]
    pub fn reuse_stats(&self) -> ReuseStats {
        self.stats
    }

    fn node_at_path(&self, path: &[usize]) -> Option<&GroupNode> {
        if path.is_empty() {
            return None;
        }
        let mut current_nodes = self.groups.as_slice();
        let mut target = None;
        for &idx in path {
            if idx >= current_nodes.len() {
                return None;
            }
            let node = &current_nodes[idx];
            target = Some(node);
            current_nodes = node.children.as_slice();
        }
        target
    }
}

fn validate_query(store: &Store, query: &Query) -> Result<(), StoreError> {
    for level in &query.group_by {
        let col = store
            .column(&level.column)
            .ok_or_else(|| StoreError::ColumnNotFound(level.column.clone()))?;
        if matches!(col, Column::Text { .. }) {
            return Err(StoreError::NotGroupable(level.column.clone()));
        }
        for tot in &level.totals {
            let tot_col = store
                .column(&tot.column)
                .ok_or_else(|| StoreError::ColumnNotFound(tot.column.clone()))?;
            if tot.aggregate != Aggregate::Count
                && !matches!(tot_col, Column::Int { .. } | Column::Float { .. })
            {
                return Err(StoreError::TypeMismatch {
                    column: tot.column.clone(),
                    expected: "Int or Float",
                });
            }
        }
    }
    for key in &query.sort {
        if store.column(&key.column).is_none() {
            return Err(StoreError::ColumnNotFound(key.column.clone()));
        }
    }
    Ok(())
}

fn build_groups(
    store: &Store,
    rows: &[u32],
    group_by: &[GroupLevel],
    sort: &[SortKey],
) -> Result<Vec<GroupNode>, StoreError> {
    if group_by.is_empty() {
        return Ok(Vec::new());
    }
    build_group_level(store, rows, &[], 0, group_by, sort)
}

fn build_group_level(
    store: &Store,
    rows: &[u32],
    parent_key: &[Option<u64>],
    level: usize,
    group_by: &[GroupLevel],
    sort: &[SortKey],
) -> Result<Vec<GroupNode>, StoreError> {
    if level >= group_by.len() || rows.is_empty() {
        return Ok(Vec::new());
    }

    let current_level = &group_by[level];
    let col_name = &current_level.column;
    let mut groups = store.group(rows, std::slice::from_ref(col_name), &current_level.totals)?;

    if is_column_desc(sort, col_name) {
        groups.reverse();
    }

    let is_last_level = level + 1 == group_by.len();
    let mut nodes = Vec::with_capacity(groups.len());

    for grp in groups {
        let mut key = Vec::with_capacity(parent_key.len() + 1);
        key.extend_from_slice(parent_key);
        key.push(grp.key[0]);

        let row_count = grp.rows.len();

        let (children, leaf_rows) = if is_last_level {
            (Vec::new(), grp.rows)
        } else {
            let ch = build_group_level(store, &grp.rows, &key, level + 1, group_by, sort)?;
            (ch, Vec::new())
        };

        nodes.push(GroupNode {
            key,
            level,
            row_count,
            totals: grp.totals,
            children,
            rows: leaf_rows,
        });
    }

    Ok(nodes)
}

#[inline]
fn is_column_desc(sort: &[SortKey], col_name: &str) -> bool {
    sort.iter()
        .find(|k| k.column == col_name)
        .is_some_and(|k| k.order == Order::Desc)
}

#[cfg(test)]
#[path = "test/view/test.rs"]
mod test;
