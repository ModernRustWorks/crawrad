---
id: props-props-interval
kind: doc
target_file: src/props.rs
line_hint: 1153
author: 
created: 
title: "Props::interval"
tags: []
signature: "pub fn interval (default_ms : i64) -> PropDesc"
---

Trigger interval in milliseconds for timers (`Interval`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::interval(1000);
assert_eq!(p.key, "interval");
```
