---
id: props-props-show-column-headers
kind: doc
target_file: src/props.rs
line_hint: 1913
author: 
created: 
title: "Props::show_column_headers"
tags: []
signature: "pub fn show_column_headers (default : bool) -> PropDesc"
---

Whether column headers are shown in a `ListView` (`ShowColumnHeaders`, #500).

Whether column headers are shown in a report-style `ListView` (`ShowColumnHeaders`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::show_column_headers(true);
assert_eq!(p.key, "show_column_headers");
```
