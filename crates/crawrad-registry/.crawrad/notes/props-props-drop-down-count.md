---
id: props-props-drop-down-count
kind: doc
target_file: src/props.rs
line_hint: 2105
author: 
created: 
title: "Props::drop_down_count"
tags: []
signature: "pub fn drop_down_count (default : i64) -> PropDesc"
---

Rows visible in the drop-down before scrolling (`DropDownCount`, #500).

How many rows are visible in the drop-down list before scrolling (`DropDownCount`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::drop_down_count(8);
assert_eq!(p.key, "drop_down_count");
```
