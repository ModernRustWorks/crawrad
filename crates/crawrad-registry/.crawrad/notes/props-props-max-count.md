---
id: props-props-max-count
kind: doc
target_file: src/props.rs
line_hint: 2037
author: 
created: 
title: "Props::max_count"
tags: []
signature: "pub fn max_count (default : i64) -> PropDesc"
---

Maximum notification count before showing `+` for a `Badge` (`MaxCount`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::max_count(99);
assert_eq!(p.key, "max_count");
```
