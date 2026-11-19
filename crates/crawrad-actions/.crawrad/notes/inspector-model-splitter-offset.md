---
id: inspector-model-splitter-offset
kind: doc
target_file: src/inspector_model.rs
line_hint: 627
author: 
created: 
title: "SPLITTER_OFFSET"
tags: []
signature: "pub const SPLITTER_OFFSET : f64 = 4.0 + 2.0 - 3.5"
---

The offset of the splitter strip's left edge from the name column.

The splitter strip's left edge from the name column's: the row's left
padding plus half the gap, less half the strip's own 7 px.

# Examples

```rust
use crawrad_actions::inspector_model::SPLITTER_OFFSET;

assert_eq!(SPLITTER_OFFSET, 2.5);
```
