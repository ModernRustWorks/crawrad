---
id: debug-panel-row
kind: doc
target_file: src/debug_panel.rs
line_hint: 22
author: 
created: 
title: "Row"
tags: []
signature: "pub enum Row"
---

One row of the panel.

# Examples

```rust
use crawrad_host::debug_panel::Row;

let row = Row::Section("CALL STACK");
assert_eq!(row.template(), "Section");
```
