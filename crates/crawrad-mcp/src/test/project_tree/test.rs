//! `project.get_tree`: hidden entries and `target` out, depth bounded.

use super::{canonical_key, walk};

#[test]
fn the_tree_skips_hidden_and_target_and_stops_at_depth() {
    let root = std::env::temp_dir().join("crawrad_mcp_project_tree");
    let _ = std::fs::remove_dir_all(&root);
    for dir in ["src/deep", "target/debug", ".git"] {
        std::fs::create_dir_all(root.join(dir)).expect("the folders");
    }
    std::fs::write(root.join("Cargo.toml"), "x").expect("a file");
    std::fs::write(root.join("src/deep/a.rs"), "").expect("a deep file");

    let tree = walk(&root, 1, 2);
    let names: Vec<&str> = tree.iter().filter_map(|e| e["name"].as_str()).collect();
    assert_eq!(names, ["Cargo.toml", "src"]);
    assert_eq!(tree[0]["size_bytes"], 1);
    let src = &tree[1];
    assert_eq!(src["is_dir"], true);
    assert_eq!(src["children"][0]["name"], "deep");
    assert_eq!(
        src["children"][0]["children"].as_array().map(Vec::len),
        Some(0),
        "level 3 is past max_depth 2"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn keys_use_forward_slashes() {
    assert_eq!(canonical_key(".\\src\\main.rs"), "src/main.rs");
    assert_eq!(canonical_key("./a.rs"), "a.rs");
}
