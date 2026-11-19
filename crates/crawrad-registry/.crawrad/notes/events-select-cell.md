---
id: events-select-cell
kind: doc
target_file: src/lib.rs
line_hint: 368
author: 
created: 
title: "Events::SELECT_CELL"
tags: []
signature: "pub const SELECT_CELL : EventDesc = EventDesc :: new (\"on_select_cell\" , \"OnSelectCell\" , \"fn(&mut self, col: usize, row: usize)\" ,)"
---

A cell in a grid was selected.

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::SELECT_CELL.name, "on_select_cell");
```
