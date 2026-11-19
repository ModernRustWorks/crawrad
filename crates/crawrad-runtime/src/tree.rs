//! The rows a tree shows: its nodes in pre-order, minus the ones under
//! a collapsed node. One rule for every target, so a click means the same
//! node everywhere.

use crawrad_note::note;
use std::collections::BTreeSet;

#[note("tree-max-depth")]
const MAX_DEPTH: usize = 32;

#[note("tree-treenode")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TreeNode {
    pub text: String,
    pub children: Vec<TreeNode>,
    pub symbol: String,
}

impl TreeNode {
    pub fn leaf(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            children: Vec::new(),
            symbol: String::new(),
        }
    }

    pub fn branch(text: impl Into<String>, children: Vec<TreeNode>) -> Self {
        Self {
            text: text.into(),
            children,
            symbol: String::new(),
        }
    }

    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }
}

#[note("tree-treerow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeRow {
    /// The node's pre-order index in the WHOLE tree (collapsed ones counted):
    /// what `selected` and `collapsed` hold.
    pub index: usize,
    pub level: usize,
    pub text: String,
    pub has_children: bool,
    pub collapsed: bool,
    pub symbol: String,
}

#[note("tree-visible-rows")]
pub fn visible_rows(nodes: &[TreeNode], collapsed: &BTreeSet<usize>) -> Vec<TreeRow> {
    fn walk(
        node: &TreeNode,
        level: usize,
        visible: bool,
        collapsed: &BTreeSet<usize>,
        counter: &mut usize,
        rows: &mut Vec<TreeRow>,
    ) {
        let idx = *counter;
        *counter += 1;

        let has_children = !node.children.is_empty();
        let is_collapsed = collapsed.contains(&idx);

        if visible {
            rows.push(TreeRow {
                index: idx,
                level,
                text: node.text.clone(),
                has_children,
                collapsed: is_collapsed,
                symbol: node.symbol.clone(),
            });
        }

        if level < MAX_DEPTH {
            let children_visible = visible && !is_collapsed;
            for child in &node.children {
                walk(child, level + 1, children_visible, collapsed, counter, rows);
            }
        }
    }

    let mut rows = Vec::new();
    let mut counter = 0;
    for node in nodes {
        walk(node, 0, true, collapsed, &mut counter, &mut rows);
    }
    rows
}

#[note("tree-row-texts")]
pub fn row_texts(rows: &[TreeRow]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            let prefix = if r.has_children {
                if r.collapsed { "▶ " } else { "▼ " }
            } else {
                "• "
            };
            if r.symbol.is_empty() {
                format!("{prefix}{}", r.text)
            } else {
                format!("{prefix}{} {}", r.symbol, r.text)
            }
        })
        .collect()
}

#[note("tree-row-of")]
pub fn row_of(rows: &[TreeRow], selected: Option<usize>) -> Option<usize> {
    let sel = selected?;
    rows.iter().position(|r| r.index == sel)
}

#[note("tree-count")]
pub fn count(nodes: &[TreeNode]) -> usize {
    fn count_node(node: &TreeNode, level: usize) -> usize {
        let mut total = 1;
        if level < MAX_DEPTH {
            for child in &node.children {
                total += count_node(child, level + 1);
            }
        }
        total
    }

    nodes.iter().map(|n| count_node(n, 0)).sum()
}

#[cfg(test)]
#[path = "test/tree/test.rs"]
mod test;
