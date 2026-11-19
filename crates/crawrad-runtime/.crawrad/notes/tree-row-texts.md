---
id: tree-row-texts
kind: doc
target_file: src/tree.rs
line_hint: 98
author: 
created: 
title: "row_texts"
tags: []
signature: "pub fn row_texts (rows : & [TreeRow]) -> Vec < String >"
---

Formats display texts with branch and leaf markers for tree rows.

The texts a list shows for `rows`: `"▶ "` for a collapsed node with
children, `"▼ "` for an expanded one, `"• "` for a leaf, then the text.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_runtime::tree::{row_texts, visible_rows, TreeNode};

let nodes = vec![TreeNode::leaf("Item")];
let rows = visible_rows(&nodes, &BTreeSet::new());
let texts = row_texts(&rows);
assert_eq!(texts, vec!["• Item"]);
```
