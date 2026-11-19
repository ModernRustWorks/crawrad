---
id: props-props-opened
kind: doc
target_file: src/props.rs
line_hint: 2072
author: 
created: 
title: "Props::opened"
tags: []
signature: "pub fn opened (default : bool) -> PropDesc"
---

Whether a `Drawer` is open and visible (`Opened`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::opened(true);
assert_eq!(p.key, "opened");
```
