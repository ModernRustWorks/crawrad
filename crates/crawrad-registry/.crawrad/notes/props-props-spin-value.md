---
id: props-props-spin-value
kind: doc
target_file: src/props.rs
line_hint: 714
author: 
created: 
title: "Props::spin_value"
tags: []
signature: "pub fn spin_value (default : i64) -> PropDesc"
---

The number a `SpinEdit` holds (`Value`, #500).

Index of selected item (`ItemIndex`).
The number a `SpinEdit` holds (`Value`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::spin_value(42);
assert_eq!(p.key, "value");
```
