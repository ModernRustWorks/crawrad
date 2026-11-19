---
id: props-props-removable
kind: doc
target_file: src/props.rs
line_hint: 251
author: 
created: 
title: "Props::removable"
tags: []
signature: "pub fn removable (default : bool) -> PropDesc"
---

Whether each chip shows a remove button (`Removable`, #500).

Whether each chip in a `Chips` component shows a remove button (`Removable`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::removable(true);
assert_eq!(p.key, "removable");
```
