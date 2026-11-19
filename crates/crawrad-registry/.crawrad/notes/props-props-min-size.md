---
id: props-props-min-size
kind: doc
target_file: src/props.rs
line_hint: 1869
author: 
created: 
title: "Props::min_size"
tags: []
signature: "pub fn min_size (default : i64) -> PropDesc"
---

Minimum size for resizing adjacent controls in Splitter (`MinSize`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::min_size(30);
assert_eq!(p.key, "min_size");
```
