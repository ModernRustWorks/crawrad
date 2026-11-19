---
id: debug-panel-rows
kind: doc
target_file: src/debug_panel.rs
line_hint: 79
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (debug : & Debug , breakpoints : & Breakpoints) -> Vec < Row >"
---

Every row, top to bottom.

# Examples

```rust
use crawrad_host::{Breakpoints, Debug};
use crawrad_host::debug_panel::rows;

let debug = Debug::default();
let bp = Breakpoints::default();
let list = rows(&debug, &bp);
assert!(!list.is_empty());
```
