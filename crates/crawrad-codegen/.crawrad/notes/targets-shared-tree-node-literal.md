---
id: targets-shared-tree-node-literal
kind: doc
target_file: src/targets/shared.rs
line_hint: 525
author: 
created: 
title: "tree_node_literal"
tags: []
signature: "pub fn tree_node_literal (item : & crawrad_ir :: Item , symbols : & [String]) -> String"
---

Builds the Rust literal constructing a runtime `TreeNode` from an IR `Item`.

# Examples

```rust
use crawrad_codegen::targets::shared::tree_node_literal;
use crawrad_ir::Item;

let item = Item::new("Node1");
let lit = tree_node_literal(&item, &[]);
assert!(lit.contains("TreeNode::leaf"));
```
