---
id: layout-splitmode
kind: doc
target_file: src/layout.rs
line_hint: 2
author: 
created: 
title: "SplitMode"
tags: []
signature: "pub enum SplitMode"
---

Layout splitting options for terminal multi-pane viewing.

# Examples

```rust
use crawrad_terminal::SplitMode;

let mode = SplitMode::Single;
assert_eq!(mode, SplitMode::Single);
```
