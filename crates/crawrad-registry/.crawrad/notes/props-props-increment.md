---
id: props-props-increment
kind: doc
target_file: src/props.rs
line_hint: 756
author: 
created: 
title: "Props::increment"
tags: []
signature: "pub fn increment (default : i64) -> PropDesc"
---

The step of a `SpinEdit`'s buttons (`Increment`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::increment(1);
assert_eq!(p.key, "increment");
```
