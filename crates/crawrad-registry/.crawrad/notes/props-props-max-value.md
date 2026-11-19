---
id: props-props-max-value
kind: doc
target_file: src/props.rs
line_hint: 742
author: 
created: 
title: "Props::max_value"
tags: []
signature: "pub fn max_value (default : i64) -> PropDesc"
---

The largest a `SpinEdit` goes (`MaxValue`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::max_value(100);
assert_eq!(p.key, "max_value");
```
