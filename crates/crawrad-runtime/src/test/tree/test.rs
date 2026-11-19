use super::*;
use std::collections::BTreeSet;

fn sample_two_level_tree() -> Vec<TreeNode> {
    vec![
        TreeNode::branch("A", vec![TreeNode::leaf("A1"), TreeNode::leaf("A2")]),
        TreeNode::leaf("B"),
    ]
}

#[test]
fn preorder_indexes_and_levels_of_two_level_tree() {
    let nodes = sample_two_level_tree();
    let collapsed = BTreeSet::new();
    let rows = visible_rows(&nodes, &collapsed);

    assert_eq!(rows.len(), 4);

    assert_eq!(rows[0].index, 0);
    assert_eq!(rows[0].level, 0);
    assert_eq!(rows[0].text, "A");
    assert!(rows[0].has_children);
    assert!(!rows[0].collapsed);

    assert_eq!(rows[1].index, 1);
    assert_eq!(rows[1].level, 1);
    assert_eq!(rows[1].text, "A1");
    assert!(!rows[1].has_children);
    assert!(!rows[1].collapsed);

    assert_eq!(rows[2].index, 2);
    assert_eq!(rows[2].level, 1);
    assert_eq!(rows[2].text, "A2");
    assert!(!rows[2].has_children);
    assert!(!rows[2].collapsed);

    assert_eq!(rows[3].index, 3);
    assert_eq!(rows[3].level, 0);
    assert_eq!(rows[3].text, "B");
    assert!(!rows[3].has_children);
    assert!(!rows[3].collapsed);
}

#[test]
fn collapsing_parent_hides_children_and_preserves_following_index() {
    let nodes = sample_two_level_tree();
    let mut collapsed = BTreeSet::new();
    collapsed.insert(0);

    let rows = visible_rows(&nodes, &collapsed);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].index, 0);
    assert_eq!(rows[0].text, "A");
    assert!(rows[0].has_children);
    assert!(rows[0].collapsed);

    assert_eq!(rows[1].index, 3);
    assert_eq!(rows[1].level, 0);
    assert_eq!(rows[1].text, "B");
    assert!(!rows[1].has_children);
    assert!(!rows[1].collapsed);
}

#[test]
fn collapsed_parent_inside_collapsed_parent_hides_subtree() {
    let tree = vec![TreeNode::branch(
        "R",
        vec![TreeNode::branch(
            "P",
            vec![TreeNode::branch("C", vec![TreeNode::leaf("L")])],
        )],
    )];

    // R is index 0, P is index 1, C is index 2, L is index 3.
    assert_eq!(count(&tree), 4);

    // Collapsing both R (0) and P (1)
    let mut collapsed = BTreeSet::new();
    collapsed.insert(0);
    collapsed.insert(1);

    let rows = visible_rows(&tree, &collapsed);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].index, 0);
    assert_eq!(rows[0].text, "R");
    assert!(rows[0].collapsed);

    // If R is expanded and P is collapsed, P shows collapsed and C/L are hidden
    collapsed.remove(&0);
    let rows_p = visible_rows(&tree, &collapsed);
    assert_eq!(rows_p.len(), 2);
    assert_eq!(rows_p[0].index, 0);
    assert!(!rows_p[0].collapsed);
    assert_eq!(rows_p[1].index, 1);
    assert_eq!(rows_p[1].text, "P");
    assert!(rows_p[1].collapsed);
}

#[test]
fn row_texts_gives_three_prefixes() {
    let rows = vec![
        TreeRow {
            index: 0,
            level: 0,
            text: "BranchCollapsed".to_string(),
            has_children: true,
            collapsed: true,
            symbol: String::new(),
        },
        TreeRow {
            index: 1,
            level: 0,
            text: "BranchExpanded".to_string(),
            has_children: true,
            collapsed: false,
            symbol: String::new(),
        },
        TreeRow {
            index: 2,
            level: 1,
            text: "Leaf".to_string(),
            has_children: false,
            collapsed: false,
            symbol: String::new(),
        },
    ];

    let texts = row_texts(&rows);
    assert_eq!(
        texts,
        vec![
            "▶ BranchCollapsed".to_string(),
            "▼ BranchExpanded".to_string(),
            "• Leaf".to_string(),
        ]
    );
}

#[test]
fn row_texts_includes_symbol_when_present() {
    let tree = vec![
        TreeNode::branch("Folder", vec![TreeNode::leaf("File").with_symbol("📄")])
            .with_symbol("📁"),
        TreeNode::leaf("NoIcon"),
    ];
    let rows = visible_rows(&tree, &BTreeSet::new());
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].symbol, "📁");
    assert_eq!(rows[1].symbol, "📄");
    assert_eq!(rows[2].symbol, "");

    let texts = row_texts(&rows);
    assert_eq!(
        texts,
        vec![
            "▼ 📁 Folder".to_string(),
            "• 📄 File".to_string(),
            "• NoIcon".to_string(),
        ]
    );

    let mut collapsed = BTreeSet::new();
    collapsed.insert(0);
    let collapsed_rows = visible_rows(&tree, &collapsed);
    let collapsed_texts = row_texts(&collapsed_rows);
    assert_eq!(
        collapsed_texts,
        vec!["▶ 📁 Folder".to_string(), "• NoIcon".to_string()]
    );
}

#[test]
fn row_of_maps_visible_node_and_none_for_hidden() {
    let nodes = sample_two_level_tree();
    let mut collapsed = BTreeSet::new();
    collapsed.insert(0);

    let rows = visible_rows(&nodes, &collapsed);
    // Visible rows are A (index 0 at row 0) and B (index 3 at row 1).

    assert_eq!(row_of(&rows, Some(0)), Some(0));
    assert_eq!(row_of(&rows, Some(3)), Some(1));
    assert_eq!(row_of(&rows, Some(1)), None);
    assert_eq!(row_of(&rows, Some(2)), None);
    assert_eq!(row_of(&rows, Some(99)), None);
    assert_eq!(row_of(&rows, None), None);
}

#[test]
fn count_computes_total_nodes_in_tree() {
    let nodes = sample_two_level_tree();
    assert_eq!(count(&nodes), 4);

    let empty: [TreeNode; 0] = [];
    assert_eq!(count(&empty), 0);

    let single = [TreeNode::leaf("Only")];
    assert_eq!(count(&single), 1);
}
