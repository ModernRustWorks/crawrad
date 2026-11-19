//! Flat line view and windowed caching over tabular data sources and group trees.
//!
//! Provides two screen-reading models:
//! - [`GroupedFlat`]: Flattens an in-memory hierarchical [`View`] into a single flat line
//!   sequence (group headers, data rows, group footers) with $O(1)$ length, $O(\text{depth} \cdot \log n)$
//!   line indexing, and path-only recomputation on expand/collapse.
//! - [`WindowedRows`]: Paged window cache with LRU eviction and prefetching over any [`RowSource`].

use crate::source::{RowSource, Rows, SourceError};
use crate::store::{Store, StoreError};
use crate::view::{GroupNode, Query, View};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// The semantic kind of a visible line in a flat grid view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LineKind {
    Header,
    Row,
    Footer,
}

/// Stable identity of a visible line across sorting and requery operations.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LineKey {
    /// Group header or footer key containing the ordinal path from the root group down to this node.
    Group(Vec<Option<u64>>),
    /// Data row stable identifier within its underlying source or store.
    Row(u64),
}

/// A single rendered line within a flattened view sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// Line semantic kind: Header, Row, or Footer.
    pub kind: LineKind,
    /// Nesting depth (0 for top-level groups; rows sit at depth = number of group levels).
    pub depth: usize,
    /// Stable line key surviving re-sorts.
    pub key: LineKey,
}

/// Node in the internal index tree tracking visible subtree sizes and prefix sums.
#[derive(Debug, Clone)]
struct FlatNode {
    visible_size: usize,
    prefix_sums: Vec<usize>,
    row_prefix_sums: Vec<usize>,
    children: Vec<FlatNode>,
}

/// Flat sequential view over an in-memory hierarchical [`View`].
///
/// Flattens group headers, nested children, leaf data rows, and group footers
/// into a contiguous indexed line sequence. Reading any line `i` takes $O(\text{depth} \cdot \log \text{siblings})$
/// using prefix-sum binary search descending from the root, touching no earlier lines.
pub struct GroupedFlat {
    view: View,
    show_footers: bool,
    root_nodes: Vec<FlatNode>,
    root_prefix_sums: Vec<usize>,
    root_row_prefix_sums: Vec<usize>,
    row_positions: Mutex<Option<HashMap<u64, usize>>>,
    #[cfg(test)]
    pub(crate) visited_nodes: AtomicUsize,
    #[cfg(test)]
    pub(crate) prefix_sums_rewritten: AtomicUsize,
}

/// Type alias for [`GroupedFlat`].
pub type FlatView = GroupedFlat;

impl GroupedFlat {
    /// Constructs a flat line sequence owning `view`.
    ///
    /// When `show_footers` is true, expanded groups display a trailing footer line.
    pub fn new(view: View, show_footers: bool) -> Self {
        let mut flat = Self {
            view,
            show_footers,
            root_nodes: Vec::new(),
            root_prefix_sums: Vec::new(),
            root_row_prefix_sums: Vec::new(),
            row_positions: Mutex::new(None),
            #[cfg(test)]
            visited_nodes: AtomicUsize::new(0),
            #[cfg(test)]
            prefix_sums_rewritten: AtomicUsize::new(0),
        };
        flat.build_index();
        flat
    }

    /// Reference to the underlying query result view.
    #[inline]
    pub fn view(&self) -> &View {
        &self.view
    }

    /// Mutable reference to the underlying query result view.
    #[inline]
    pub fn view_mut(&mut self) -> &mut View {
        &mut self.view
    }

    /// Consumes this flat view, returning the inner [`View`].
    #[inline]
    pub fn into_view(self) -> View {
        self.view
    }

    /// Returns whether group footers are displayed for expanded groups.
    #[inline]
    pub fn show_footers(&self) -> bool {
        self.show_footers
    }

    /// Updates whether group footers are displayed, rebuilding the visible index.
    pub fn set_show_footers(&mut self, show_footers: bool) {
        if self.show_footers != show_footers {
            self.show_footers = show_footers;
            self.build_index();
        }
    }

    /// Total number of visible lines currently displayed.
    ///
    /// Runs in $O(1)$ time by reading the cached root prefix sum total.
    #[inline]
    pub fn len(&self) -> usize {
        if self.view.groups().is_empty() {
            self.view.row_count()
        } else {
            self.root_prefix_sums.last().copied().unwrap_or(0)
        }
    }

    /// Returns whether the flat sequence is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieves the visible [`Line`] at flat index `i`.
    ///
    /// Runs in $O(\text{depth} \cdot \log \text{siblings})$ without allocating memory for rows
    /// and without scanning earlier lines `0..i`.
    pub fn line(&self, i: usize) -> Option<Line> {
        if i >= self.len() {
            return None;
        }

        if self.view.groups().is_empty() {
            let row_id = self.view.rows()[i];
            return Some(Line {
                kind: LineKind::Row,
                depth: 0,
                key: LineKey::Row(row_id as u64),
            });
        }

        let num_levels = self.view.query().group_by.len();
        let mut target = i;
        let mut current_groups = self.view.groups();
        let mut current_nodes = &self.root_nodes;
        let mut current_prefix_sums = &self.root_prefix_sums;

        loop {
            let idx = self.find_partition_point(current_prefix_sums, target);
            if idx >= current_groups.len() {
                return None;
            }
            let start_offset = if idx == 0 {
                0
            } else {
                current_prefix_sums[idx - 1]
            };
            let local_offset = target - start_offset;
            let group = &current_groups[idx];
            let node = &current_nodes[idx];

            if local_offset == 0 {
                return Some(Line {
                    kind: LineKind::Header,
                    depth: group.level,
                    key: LineKey::Group(group.key.clone()),
                });
            }

            let body_offset = local_offset - 1;
            let children_size = if group.children.is_empty() {
                group.rows.len()
            } else {
                node.prefix_sums.last().copied().unwrap_or(0)
            };

            if body_offset < children_size {
                if group.children.is_empty() {
                    let row_id = group.rows[body_offset];
                    return Some(Line {
                        kind: LineKind::Row,
                        depth: num_levels,
                        key: LineKey::Row(row_id as u64),
                    });
                } else {
                    target = body_offset;
                    current_groups = &group.children;
                    current_nodes = &node.children;
                    current_prefix_sums = &node.prefix_sums;
                }
            } else if self.show_footers && body_offset == children_size {
                return Some(Line {
                    kind: LineKind::Footer,
                    depth: group.level,
                    key: LineKey::Group(group.key.clone()),
                });
            } else {
                return None;
            }
        }
    }

    /// Returns the total matching row count of the group node at `line_index`.
    ///
    /// Returns `Some(count)` if `line_index` points to a Header or Footer line,
    /// or `None` if it points to a Row or is out of bounds.
    pub fn group_row_count(&self, line_index: usize) -> Option<usize> {
        self.find_node_at_line(line_index)
            .map(|(node, _)| node.row_count)
    }

    /// Returns the aggregate totals of the group node at `line_index`.
    ///
    /// Returns `Some(totals)` if `line_index` points to a Header or Footer line,
    /// or `None` if it points to a Row or is out of bounds.
    pub fn group_totals(&self, line_index: usize) -> Option<&[Option<f64>]> {
        self.find_node_at_line(line_index)
            .map(|(node, _)| node.totals.as_slice())
    }

    /// Returns whether the group node at `line_index` is expanded.
    ///
    /// Returns `Some(expanded)` if `line_index` points to a Header or Footer line,
    /// or `None` if it points to a Row or is out of bounds.
    pub fn is_expanded(&self, line_index: usize) -> Option<bool> {
        self.find_node_at_line(line_index)
            .map(|(node, _)| self.view.is_key_expanded(&node.key))
    }

    /// Returns whether the group at `path` is currently expanded.
    #[inline]
    pub fn is_expanded_path(&self, path: &[usize]) -> bool {
        self.view.is_expanded(path)
    }

    /// Returns a reference to the [`GroupNode`] at `line_index`, if it is a Header or Footer line.
    pub fn group_at_line(&self, line_index: usize) -> Option<&GroupNode> {
        self.find_node_at_line(line_index).map(|(node, _)| node)
    }

    /// Sets group expansion state for the node at `path`.
    ///
    /// # Performance Cost
    /// Recomputes only the visible subtree sizes and prefix sums of sibling lists along
    /// the direct path from the target node up to the root. If the path has ancestor sibling lists
    /// of sizes $L_1, L_2, \dots, L_d$, the recomputation cost is $O(\sum_{k=1}^d L_k)$,
    /// touching zero unrelated subtrees.
    pub fn set_expanded(&mut self, path: &[usize], expanded: bool) -> bool {
        if path.is_empty() || !is_valid_path(self.view.groups(), path) {
            return false;
        }

        self.view.set_expanded(path, expanded);

        #[cfg(test)]
        let rewritten_counter = &self.prefix_sums_rewritten;

        update_path(
            &mut self.root_nodes,
            &mut self.root_prefix_sums,
            path,
            self.view.groups(),
            &self.view,
            self.show_footers,
            #[cfg(test)]
            rewritten_counter,
        );

        true
    }

    /// Toggles expansion of the group node located at `line_index`.
    ///
    /// If `line_index` points to a Header or Footer line, toggles its expansion and returns `true`.
    /// If it points to a Row line or is out of bounds, returns `false`.
    pub fn toggle(&mut self, line_index: usize) -> bool {
        if line_index >= self.len() || self.view.groups().is_empty() {
            return false;
        }

        let mut path = Vec::new();
        let mut target = line_index;
        let mut current_groups = self.view.groups();
        let mut current_nodes = &self.root_nodes;
        let mut current_prefix_sums = &self.root_prefix_sums;

        let target_path = loop {
            let idx = self.find_partition_point(current_prefix_sums, target);
            if idx >= current_groups.len() {
                return false;
            }
            path.push(idx);
            let start_offset = if idx == 0 {
                0
            } else {
                current_prefix_sums[idx - 1]
            };
            let local_offset = target - start_offset;
            let group = &current_groups[idx];
            let node = &current_nodes[idx];

            if local_offset == 0 {
                break Some(path);
            }

            let body_offset = local_offset - 1;
            let children_size = if group.children.is_empty() {
                group.rows.len()
            } else {
                node.prefix_sums.last().copied().unwrap_or(0)
            };

            if body_offset < children_size {
                if group.children.is_empty() {
                    break None;
                } else {
                    target = body_offset;
                    current_groups = &group.children;
                    current_nodes = &node.children;
                    current_prefix_sums = &node.prefix_sums;
                }
            } else if self.show_footers && body_offset == children_size {
                break Some(path);
            } else {
                break None;
            }
        };

        if let Some(path) = target_path {
            let is_currently_expanded = self.view.is_expanded(&path);
            self.set_expanded(&path, !is_currently_expanded)
        } else {
            false
        }
    }

    /// Expands all group nodes in the tree, rebuilding the visible index.
    pub fn expand_all(&mut self) {
        self.view.expand_all();
        self.build_index();
    }

    /// Collapses all group nodes in the tree, rebuilding the visible index.
    pub fn collapse_all(&mut self) {
        self.view.collapse_all();
        self.build_index();
    }

    /// Re-evaluates `query` against `store`, reusing cached intermediate computations when possible.
    pub fn requery(&mut self, store: &Store, query: Query) -> Result<(), StoreError> {
        self.view.requery(store, query)?;
        *self.row_positions.lock().expect("row_positions mutex") = None;
        self.build_index();
        Ok(())
    }

    /// Finds the current visible line index of a group key or row ID.
    ///
    /// Used to preserve selection and scroll positions after re-sorting or collapsing/expanding groups.
    /// Rows use a lazily built `HashMap<u64, position>` invalidated on `requery`, then map
    /// position to visible line index in $O(\text{depth} \cdot \log \text{siblings})$.
    ///
    /// Returns `None` if the row/group is currently inside a collapsed parent or not present.
    pub fn line_of_key(&self, key: &LineKey) -> Option<usize> {
        match key {
            LineKey::Group(group_key) => {
                if self.view.groups().is_empty() || group_key.is_empty() {
                    return None;
                }

                let mut current_groups = self.view.groups();
                let mut current_nodes = &self.root_nodes;
                let mut current_prefix_sums = &self.root_prefix_sums;
                let mut current_visible_offset = 0;

                for level in 0..group_key.len() {
                    let target_prefix = &group_key[..=level];
                    let k = current_groups
                        .iter()
                        .position(|g| g.key.as_slice() == target_prefix)?;
                    let sibling_start = if k == 0 {
                        0
                    } else {
                        current_prefix_sums[k - 1]
                    };
                    current_visible_offset += sibling_start;

                    if level + 1 == group_key.len() {
                        return Some(current_visible_offset);
                    }

                    let target_node = &current_nodes[k];
                    let target_group = &current_groups[k];
                    if !self.view.is_key_expanded(&target_group.key) {
                        return None;
                    }
                    current_visible_offset += 1;
                    current_groups = &target_group.children;
                    current_nodes = &target_node.children;
                    current_prefix_sums = &target_node.prefix_sums;
                }

                None
            }
            LineKey::Row(row_id) => {
                let mut guard = self.row_positions.lock().expect("row_positions mutex");
                let map = guard.get_or_insert_with(|| {
                    let mut map = HashMap::new();
                    if self.view.groups().is_empty() {
                        for (i, &rid) in self.view.rows().iter().enumerate() {
                            map.insert(rid as u64, i);
                        }
                    } else {
                        fn fill_positions(
                            nodes: &[GroupNode],
                            map: &mut HashMap<u64, usize>,
                            counter: &mut usize,
                        ) {
                            for node in nodes {
                                if node.children.is_empty() {
                                    for &rid in &node.rows {
                                        map.insert(rid as u64, *counter);
                                        *counter += 1;
                                    }
                                } else {
                                    fill_positions(&node.children, map, counter);
                                }
                            }
                        }
                        let mut counter = 0;
                        fill_positions(self.view.groups(), &mut map, &mut counter);
                    }
                    map
                });

                let &row_pos = map.get(row_id)?;

                if self.view.groups().is_empty() {
                    return Some(row_pos);
                }

                let mut target_row = row_pos;
                let mut current_visible_offset = 0;
                let mut current_groups = self.view.groups();
                let mut current_nodes = &self.root_nodes;
                let mut current_row_prefix_sums = &self.root_row_prefix_sums;
                let mut current_vis_prefix_sums = &self.root_prefix_sums;

                loop {
                    let idx = current_row_prefix_sums.partition_point(|&sum| sum <= target_row);
                    if idx >= current_groups.len() {
                        return None;
                    }
                    let row_start = if idx == 0 {
                        0
                    } else {
                        current_row_prefix_sums[idx - 1]
                    };
                    let vis_start = if idx == 0 {
                        0
                    } else {
                        current_vis_prefix_sums[idx - 1]
                    };
                    current_visible_offset += vis_start;
                    target_row -= row_start;
                    let node = &current_nodes[idx];
                    let group = &current_groups[idx];

                    if !self.view.is_key_expanded(&group.key) {
                        return None;
                    }

                    current_visible_offset += 1;

                    if group.children.is_empty() {
                        if target_row < group.rows.len() {
                            return Some(current_visible_offset + target_row);
                        } else {
                            return None;
                        }
                    }

                    current_groups = &group.children;
                    current_nodes = &node.children;
                    current_row_prefix_sums = &node.row_prefix_sums;
                    current_vis_prefix_sums = &node.prefix_sums;
                }
            }
        }
    }

    #[inline]
    fn find_partition_point(&self, prefix_sums: &[usize], target: usize) -> usize {
        #[cfg(test)]
        {
            let mut left = 0;
            let mut right = prefix_sums.len();
            while left < right {
                self.visited_nodes.fetch_add(1, Ordering::Relaxed);
                let mid = left + (right - left) / 2;
                if prefix_sums[mid] <= target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }
        #[cfg(not(test))]
        {
            prefix_sums.partition_point(|&sum| sum <= target)
        }
    }

    fn find_node_at_line(&self, line_index: usize) -> Option<(&GroupNode, LineKind)> {
        if line_index >= self.len() || self.view.groups().is_empty() {
            return None;
        }

        let mut target = line_index;
        let mut current_groups = self.view.groups();
        let mut current_nodes = &self.root_nodes;
        let mut current_prefix_sums = &self.root_prefix_sums;

        loop {
            let idx = self.find_partition_point(current_prefix_sums, target);
            if idx >= current_groups.len() {
                return None;
            }
            let start_offset = if idx == 0 {
                0
            } else {
                current_prefix_sums[idx - 1]
            };
            let local_offset = target - start_offset;
            let group = &current_groups[idx];
            let node = &current_nodes[idx];

            if local_offset == 0 {
                return Some((group, LineKind::Header));
            }

            let body_offset = local_offset - 1;
            let children_size = if group.children.is_empty() {
                group.rows.len()
            } else {
                node.prefix_sums.last().copied().unwrap_or(0)
            };

            if body_offset < children_size {
                if group.children.is_empty() {
                    return None;
                } else {
                    target = body_offset;
                    current_groups = &group.children;
                    current_nodes = &node.children;
                    current_prefix_sums = &node.prefix_sums;
                }
            } else if self.show_footers && body_offset == children_size {
                return Some((group, LineKind::Footer));
            } else {
                return None;
            }
        }
    }

    fn build_index(&mut self) {
        if self.view.groups().is_empty() {
            self.root_nodes.clear();
            self.root_prefix_sums.clear();
            self.root_row_prefix_sums.clear();
            return;
        }

        self.root_nodes = self
            .view
            .groups()
            .iter()
            .map(|g| build_node_index(g, &self.view, self.show_footers))
            .collect();

        self.root_prefix_sums = Vec::with_capacity(self.root_nodes.len());
        let mut acc_vis = 0;
        for node in &self.root_nodes {
            acc_vis += node.visible_size;
            self.root_prefix_sums.push(acc_vis);
        }

        self.root_row_prefix_sums = Vec::with_capacity(self.view.groups().len());
        let mut acc_row = 0;
        for g in self.view.groups() {
            acc_row += g.row_count;
            self.root_row_prefix_sums.push(acc_row);
        }
    }

    #[cfg(test)]
    pub fn visited_nodes(&self) -> usize {
        self.visited_nodes.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub fn reset_visited_nodes(&self) {
        self.visited_nodes.store(0, Ordering::Relaxed);
    }

    #[cfg(test)]
    pub fn prefix_sums_rewritten(&self) -> usize {
        self.prefix_sums_rewritten.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub fn reset_prefix_sums_rewritten(&self) {
        self.prefix_sums_rewritten.store(0, Ordering::Relaxed);
    }
}

fn build_node_index(group: &GroupNode, view: &View, show_footers: bool) -> FlatNode {
    let children: Vec<FlatNode> = group
        .children
        .iter()
        .map(|c| build_node_index(c, view, show_footers))
        .collect();

    let mut prefix_sums = Vec::with_capacity(children.len());
    let mut acc = 0;
    for c in &children {
        acc += c.visible_size;
        prefix_sums.push(acc);
    }

    let mut row_prefix_sums = Vec::with_capacity(group.children.len());
    let mut r_acc = 0;
    for c in &group.children {
        r_acc += c.row_count;
        row_prefix_sums.push(r_acc);
    }

    let is_expanded = view.is_key_expanded(&group.key);
    let visible_size = if is_expanded {
        let body_size = if group.children.is_empty() {
            group.rows.len()
        } else {
            prefix_sums.last().copied().unwrap_or(0)
        };
        1 + body_size + if show_footers { 1 } else { 0 }
    } else {
        1
    };

    FlatNode {
        visible_size,
        prefix_sums,
        row_prefix_sums,
        children,
    }
}

fn is_valid_path(groups: &[GroupNode], path: &[usize]) -> bool {
    if path.is_empty() {
        return false;
    }
    let mut current = groups;
    for (i, &idx) in path.iter().enumerate() {
        if idx >= current.len() {
            return false;
        }
        if i + 1 < path.len() {
            current = &current[idx].children;
        }
    }
    true
}

fn update_path(
    nodes: &mut [FlatNode],
    prefix_sums: &mut [usize],
    path: &[usize],
    groups: &[GroupNode],
    view: &View,
    show_footers: bool,
    #[cfg(test)] rewritten_counter: &AtomicUsize,
) {
    let head = path[0];
    let tail = &path[1..];
    if tail.is_empty() {
        let node = &mut nodes[head];
        let group = &groups[head];
        let is_expanded = view.is_key_expanded(&group.key);
        if is_expanded {
            let body_size = if group.children.is_empty() {
                group.rows.len()
            } else {
                node.prefix_sums.last().copied().unwrap_or(0)
            };
            node.visible_size = 1 + body_size + if show_footers { 1 } else { 0 };
        } else {
            node.visible_size = 1;
        }
    } else {
        let node = &mut nodes[head];
        let group = &groups[head];
        update_path(
            &mut node.children,
            &mut node.prefix_sums,
            tail,
            &group.children,
            view,
            show_footers,
            #[cfg(test)]
            rewritten_counter,
        );
        // The child's new size only shows when this ancestor is itself expanded;
        // a collapsed ancestor stays one header line.
        node.visible_size = if view.is_key_expanded(&group.key) {
            let body_size = node.prefix_sums.last().copied().unwrap_or(0);
            1 + body_size + if show_footers { 1 } else { 0 }
        } else {
            1
        };
    }

    let mut acc = 0;
    for (k, node) in nodes.iter().enumerate() {
        acc += node.visible_size;
        prefix_sums[k] = acc;
        #[cfg(test)]
        rewritten_counter.fetch_add(1, Ordering::Relaxed);
    }
}

/// A cached page of rows in [`WindowedRows`].
struct CachedPage {
    page_index: usize,
    rows: Rows,
    last_access: AtomicU64,
}

/// Windowed LRU page cache over any [`RowSource`] for ungrouped views.
///
/// Fetches pages of `page_size` lines and caches at most `max_pages` pages using an LRU eviction policy.
/// `ensure(range)` fetches missing pages plus one prefetch page before and after when they exist.
pub struct WindowedRows {
    source: Arc<dyn RowSource>,
    query: Query,
    page_size: usize,
    max_pages: usize,
    cached_len: Mutex<Option<usize>>,
    pages: Vec<CachedPage>,
    access_tick: AtomicU64,
    fetch_calls: AtomicUsize,
}

impl WindowedRows {
    /// Constructs a `WindowedRows` cache over `source` for `query`.
    pub fn new(
        source: Arc<dyn RowSource>,
        query: Query,
        page_size: usize,
        max_pages: usize,
    ) -> Self {
        assert!(page_size > 0, "page_size must be > 0");
        assert!(max_pages > 0, "max_pages must be > 0");
        Self {
            source,
            query,
            page_size,
            max_pages,
            cached_len: Mutex::new(None),
            pages: Vec::new(),
            access_tick: AtomicU64::new(0),
            fetch_calls: AtomicUsize::new(0),
        }
    }

    /// The underlying row source.
    #[inline]
    pub fn source(&self) -> &Arc<dyn RowSource> {
        &self.source
    }

    /// The active query.
    #[inline]
    pub fn query(&self) -> &Query {
        &self.query
    }

    /// Page size in lines.
    #[inline]
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    /// Maximum pages maintained in cache.
    #[inline]
    pub fn max_pages(&self) -> usize {
        self.max_pages
    }

    /// Number of pages currently cached in memory.
    #[inline]
    pub fn cached_pages_count(&self) -> usize {
        self.pages.len()
    }

    /// Total number of rows matching `query`, cached until `set_query`.
    ///
    /// A failing source is reported, never shown as an empty result.
    pub fn len(&self) -> Result<usize, SourceError> {
        let mut guard = self.cached_len.lock().expect("cached_len mutex poisoned");
        if let Some(count) = *guard {
            return Ok(count);
        }
        let count = self.source.count(&self.query)?;
        *guard = Some(count);
        Ok(count)
    }

    /// Returns whether the row count is 0.
    #[inline]
    pub fn is_empty(&self) -> Result<bool, SourceError> {
        Ok(self.len()? == 0)
    }

    /// The row count if it was already fetched; never calls the source.
    pub(crate) fn known_len(&self) -> Option<usize> {
        *self.cached_len.lock().expect("cached_len mutex poisoned")
    }

    /// Pages `ensure(range)` wants: the overlapping ones plus one before and one after
    /// when they exist — always one contiguous run, so no list is allocated.
    fn wanted_pages(
        &self,
        range: &std::ops::Range<usize>,
        total_rows: usize,
    ) -> std::ops::RangeInclusive<usize> {
        let clamped_end = range.end.min(total_rows);
        let first_page = range.start / self.page_size;
        let last_page = (clamped_end - 1) / self.page_size;
        let total_pages = total_rows.div_ceil(self.page_size);
        first_page.saturating_sub(1)..=(last_page + 1).min(total_pages - 1)
    }

    /// Whether `ensure(range)` would call the source (row count unknown or a wanted page missing).
    pub(crate) fn needs_fetch(&self, range: std::ops::Range<usize>) -> bool {
        let Some(total_rows) = self.known_len() else {
            return true;
        };
        if total_rows == 0 || range.start >= range.end || range.start >= total_rows {
            return false;
        }
        self.wanted_pages(&range, total_rows)
            .any(|p| !self.pages.iter().any(|cached| cached.page_index == p))
    }

    /// Fetches all missing pages overlapping `range` plus one prefetch page before and after when available.
    ///
    /// Blocking: designed to be called off the UI thread.
    pub fn ensure(&mut self, range: std::ops::Range<usize>) -> Result<(), SourceError> {
        let total_rows = self.len()?;
        if total_rows == 0 || range.start >= range.end || range.start >= total_rows {
            return Ok(());
        }

        let needed_pages = self.wanted_pages(&range, total_rows);

        for page_idx in needed_pages.clone() {
            if let Some(pos) = self.pages.iter().position(|p| p.page_index == page_idx) {
                self.pages[pos]
                    .last_access
                    .store(self.next_tick(), Ordering::Relaxed);
                continue;
            }

            let page_start = page_idx * self.page_size;
            let page_end = ((page_idx + 1) * self.page_size).min(total_rows);
            let rows = self.source.fetch(&self.query, page_start..page_end)?;
            self.fetch_calls.fetch_add(1, Ordering::Relaxed);

            if self.pages.len() >= self.max_pages {
                let mut lru_idx = None;
                let mut oldest_tick = u64::MAX;

                for (idx, page) in self.pages.iter().enumerate() {
                    if !needed_pages.contains(&page.page_index) {
                        // a page this call still wants is never the victim
                        let tick = page.last_access.load(Ordering::Relaxed);
                        if tick < oldest_tick {
                            oldest_tick = tick;
                            lru_idx = Some(idx);
                        }
                    }
                }

                let victim = lru_idx.unwrap_or_else(|| {
                    self.pages
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, p)| p.last_access.load(Ordering::Relaxed))
                        .map(|(idx, _)| idx)
                        .unwrap_or(0)
                });

                self.pages.remove(victim);
            }

            let tick = self.next_tick();
            self.pages.push(CachedPage {
                page_index: page_idx,
                rows,
                last_access: AtomicU64::new(tick),
            });
        }

        Ok(())
    }

    /// Peeks line `i` without performing any fetch.
    ///
    /// Returns `Some((line, &rows, offset_in_page))` if line `i` is currently cached,
    /// or `None` if it is not cached or out of bounds.
    pub fn peek(&self, i: usize) -> Option<(Line, &Rows, usize)> {
        let total_rows = self.known_len()?;
        if i >= total_rows {
            return None;
        }

        let page_idx = i / self.page_size;
        let page = self.pages.iter().find(|p| p.page_index == page_idx)?;
        let offset = i - page_idx * self.page_size;
        if offset >= page.rows.len {
            return None;
        }

        let row_id = *page.rows.ids.get(offset)?;
        page.last_access.store(self.next_tick(), Ordering::Relaxed);
        Some((
            Line {
                kind: LineKind::Row,
                depth: 0,
                key: LineKey::Row(row_id),
            },
            &page.rows,
            offset,
        ))
    }

    /// Ensures line `i` is loaded, then peeks it (`Ok(None)` past the end).
    pub fn line(&mut self, i: usize) -> Result<Option<(Line, &Rows, usize)>, SourceError> {
        self.ensure(i..i + 1)?;
        Ok(self.peek(i))
    }

    /// Changes the active query, clearing all cached pages and the row count.
    pub fn set_query(&mut self, query: Query) {
        self.query = query;
        let mut guard = self.cached_len.lock().expect("cached_len mutex poisoned");
        *guard = None;
        self.pages.clear();
    }

    /// Clears cached pages only, retaining the cached row count.
    pub fn invalidate(&mut self) {
        self.pages.clear();
    }

    /// Number of fetch calls issued to the underlying source.
    #[inline]
    pub fn fetch_calls(&self) -> usize {
        self.fetch_calls.load(Ordering::Relaxed)
    }

    #[inline]
    fn next_tick(&self) -> u64 {
        self.access_tick.fetch_add(1, Ordering::Relaxed)
    }
}

#[cfg(test)]
#[path = "test/flat/test.rs"]
mod test;
