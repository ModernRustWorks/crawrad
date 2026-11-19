---
id: props-props-margins
kind: doc
target_file: src/props.rs
line_hint: 378
author: 
created: 
title: "Props::margins"
tags: []
signature: "pub fn margins (default : Edges) -> PropDesc"
---

Component outer margins (`Margins`).

# Examples

```rust
use crawrad_ir::Edges;
use crawrad_registry::Props;

let p = Props::margins(Edges::uniform(4.0));
assert_eq!(p.key, "margins");
```
