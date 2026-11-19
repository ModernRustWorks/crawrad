---
id: semantic-legend
kind: doc
target_file: src/semantic.rs
line_hint: 42
author: 
created: 
title: "Legend"
tags: []
signature: "pub struct Legend"
---

The server's legend: token types and modifiers by index.

# Examples

```rust
use crawrad_lsp::semantic::Legend;

let legend = Legend::default();
assert!(legend.types.is_empty());
```
