---
id: props-props-group-index
kind: doc
target_file: src/props.rs
line_hint: 1563
author: 
created: 
title: "Props::group_index"
tags: []
signature: "pub fn group_index (default : i64) -> PropDesc"
---

Mutually exclusive button grouping index (`GroupIndex`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::group_index(1);
assert_eq!(p.key, "group_index");
```
