---
id: events-column-click
kind: doc
target_file: src/lib.rs
line_hint: 374
author: 
created: 
title: "Events::COLUMN_CLICK"
tags: []
signature: "pub const COLUMN_CLICK : EventDesc = EventDesc :: new (\"on_column_click\" , \"OnColumnClick\" , \"fn(&mut self, column: usize)\" ,)"
---

A column header was clicked; `column` counts from 0.

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::COLUMN_CLICK.name, "on_column_click");
```
