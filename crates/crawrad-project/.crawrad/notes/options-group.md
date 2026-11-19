---
id: options-group
kind: doc
target_file: src/options.rs
line_hint: 156
author: 
created: 
title: "Group"
tags: []
signature: "pub struct Group"
---

A tree group, and whether it opens expanded by default.

# Examples

```rust
use crawrad_project::options::{Category, Group};

let group = Group {
    name: "Test",
    items: &[Category::CommonlyUsed],
    open_by_default: true,
};
assert!(group.open_by_default);
```
