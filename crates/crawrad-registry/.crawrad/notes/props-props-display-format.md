---
id: props-props-display-format
kind: doc
target_file: src/props.rs
line_hint: 2233
author: 
created: 
title: "Props::display_format"
tags: []
signature: "pub fn display_format (default : & str) -> PropDesc"
---

The mask a column's values are shown through, e.g. `#,##0.00`.

# Examples

```rust
use crawrad_registry::Props;

assert_eq!(Props::display_format("").key, "display_format");
```
