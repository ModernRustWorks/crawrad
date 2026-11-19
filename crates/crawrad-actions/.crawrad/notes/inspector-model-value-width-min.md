---
id: inspector-model-value-width-min
kind: doc
target_file: src/inspector_model.rs
line_hint: 615
author: 
created: 
title: "VALUE_WIDTH_MIN"
tags: []
signature: "pub const VALUE_WIDTH_MIN : f64 = 64.0"
---

The minimum width reserved for the values column.

What the values keep however far the splitter goes right — enough for a
check box and a number.

# Examples

```rust
use crawrad_actions::inspector_model::VALUE_WIDTH_MIN;

assert_eq!(VALUE_WIDTH_MIN, 64.0);
```
