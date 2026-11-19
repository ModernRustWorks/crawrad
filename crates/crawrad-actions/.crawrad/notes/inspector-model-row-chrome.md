---
id: inspector-model-row-chrome
kind: doc
target_file: src/inspector_model.rs
line_hint: 624
author: 
created: 
title: "ROW_CHROME"
tags: []
signature: "pub const ROW_CHROME : f64 = 24.0"
---

The total horizontal chrome and padding of an inspector row.

What a row of the Object Inspector takes besides its name and its value:
the shell's `CrawradInspectorRow` pads 4 left and 16 right (the scroll bar),
with 4 between the two columns.

# Examples

```rust
use crawrad_actions::inspector_model::ROW_CHROME;

assert_eq!(ROW_CHROME, 24.0);
```
