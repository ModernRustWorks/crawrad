---
id: explorer-model-membernode
kind: doc
target_file: src/explorer_model.rs
line_hint: 137
author: 
created: 
title: "MemberNode"
tags: []
signature: "pub struct MemberNode"
---

A member of the group — a tree node.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::explorer_model::MemberNode;

let node = MemberNode {
    folder: PathBuf::from("member"),
    active: true,
    is_default: false,
};
assert!(node.active);
```
