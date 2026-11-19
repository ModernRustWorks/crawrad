---
id: debug-panel-row-template
kind: doc
target_file: src/debug_panel.rs
line_hint: 64
author: 
created: 
title: "Row::template"
tags: []
signature: "pub fn template (& self) -> & 'static str"
---

The DSL template that draws this row.

# Examples

```rust
use crawrad_host::debug_panel::Row;

let row = Row::Breakpoint { file: "main.rs".into(), line: 10 };
assert_eq!(row.template(), "Breakpoint");
```
