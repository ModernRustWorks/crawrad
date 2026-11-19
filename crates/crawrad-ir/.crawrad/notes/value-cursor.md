---
id: value-cursor
kind: doc
target_file: src/value.rs
line_hint: 293
author: 
created: 
title: "Cursor"
tags: []
signature: "pub enum Cursor"
---

Mouse cursor in the `.crawform` document.

# Examples

```rust
use crawrad_ir::Cursor;

assert_eq!(Cursor::default(), Cursor::Default);
assert_eq!(Cursor::Hand.as_str(), "hand");
```
