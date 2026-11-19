---
id: inspector-model-name-width-min
kind: doc
target_file: src/inspector_model.rs
line_hint: 612
author: 
created: 
title: "NAME_WIDTH_MIN"
tags: []
signature: "pub const NAME_WIDTH_MIN : f64 = 48.0"
---

The minimum allowed width for the names column.

The narrowest the names may get: past it the splitter is a wall, not a
column, and a row whose name reads `Al` names nothing.

# Examples

```rust
use crawrad_actions::inspector_model::NAME_WIDTH_MIN;

assert_eq!(NAME_WIDTH_MIN, 48.0);
```
