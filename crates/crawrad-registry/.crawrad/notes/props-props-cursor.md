---
id: props-props-cursor
kind: doc
target_file: src/props.rs
line_hint: 554
author: 
created: 
title: "Props::cursor"
tags: []
signature: "pub fn cursor (default : crawrad_ir :: Cursor) -> PropDesc"
---

Mouse cursor (`Cursor`).

# Examples

```rust
use crawrad_ir::Cursor;
use crawrad_registry::Props;

let p = Props::cursor(Cursor::Default);
assert_eq!(p.key, "cursor");
```
