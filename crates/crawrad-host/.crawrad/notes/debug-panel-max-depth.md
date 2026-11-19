---
id: debug-panel-max-depth
kind: doc
target_file: src/debug_panel.rs
line_hint: 19
author: 
created: 
title: "MAX_DEPTH"
tags: []
signature: "pub const MAX_DEPTH : usize = 8"
---

How deep the panel opens nested values: past this, a child has no ▸.

# Examples

```rust
use crawrad_host::debug_panel::MAX_DEPTH;

assert_eq!(MAX_DEPTH, 8);
```
