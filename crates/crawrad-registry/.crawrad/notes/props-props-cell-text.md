---
id: props-props-cell-text
kind: doc
target_file: src/props.rs
line_hint: 163
author: 
created: 
title: "Props::cell_text"
tags: []
signature: "pub fn cell_text (default : & str) -> PropDesc"
---

Design-time grid cells text (`Cells`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::cell_text("A,B\n1,2");
assert_eq!(p.key, "cell_text");
```
