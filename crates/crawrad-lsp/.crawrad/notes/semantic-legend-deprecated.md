---
id: semantic-legend-deprecated
kind: doc
target_file: src/semantic.rs
line_hint: 77
author: 
created: 
title: "Legend::deprecated"
tags: []
signature: "pub fn deprecated (& self , mods : u32) -> bool"
---

Returns whether modifier bits mark something deprecated.

Whether the token names something `#[deprecated]` (#730): measured on
rust-analyzer 0.3.3065, the modifier comes on the use and on the
declaration, while typing, before any check runs.

# Examples

```rust
use crawrad_lsp::semantic::Legend;

let legend = Legend {
    types: Vec::new(),
    modifiers: vec!["deprecated".to_string()],
};
assert!(legend.deprecated(1));
assert!(!legend.deprecated(0));
```
