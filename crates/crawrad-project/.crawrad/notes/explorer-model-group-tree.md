---
id: explorer-model-group-tree
kind: doc
target_file: src/explorer_model.rs
line_hint: 160
author: 
created: 
title: "group_tree"
tags: []
signature: "pub fn group_tree (root : & Path , document : Option < & Path >) -> Vec < MemberNode >"
---

Members of the group, in manifest order — `default-members` first.

The order is that of [`crate::workspace::members`], and is **not** reordered by
"active": the tree is a list one learns to click on, and a list rebuilt with
each open form is a list one reads from scratch every time.

**The member of the open document enters even if the manifest does not declare it.**
A `members` with glob (`crates/*`), which `workspace` parser does not resolve,
or a root `Cargo.toml` that no one wrote yet, would leave the tree without the
project currently on screen — and a panel hiding what the user is editing is worse
than no panel at all.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_model::group_tree;

let tree = group_tree(Path::new("nonexistent"), None);
assert!(tree.is_empty());
```
