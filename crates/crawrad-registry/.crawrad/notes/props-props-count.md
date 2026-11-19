---
id: props-props-count
kind: doc
target_file: src/props.rs
line_hint: 2026
author: 
created: 
title: "Props::count"
tags: []
signature: "pub fn count (default : i64) -> PropDesc"
---

Notification count for a `Badge` (`Count`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::count(5);
assert_eq!(p.key, "count");
```
