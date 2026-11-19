---
id: props-props-outline
kind: doc
target_file: src/props.rs
line_hint: 2015
author: 
created: 
title: "Props::outline"
tags: []
signature: "pub fn outline (default : bool) -> PropDesc"
---

Whether a `Badge` is drawn outlined rather than filled (`Outline`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::outline(true);
assert_eq!(p.key, "outline");
```
