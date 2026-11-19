---
id: props-props-min-value
kind: doc
target_file: src/props.rs
line_hint: 728
author: 
created: 
title: "Props::min_value"
tags: []
signature: "pub fn min_value (default : i64) -> PropDesc"
---

The smallest a `SpinEdit` goes (`MinValue`; with `MaxValue` both 0, no range).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::min_value(0);
assert_eq!(p.key, "min_value");
```
