---
id: value-color-transparent
kind: doc
target_file: src/value.rs
line_hint: 558
author: 
created: 
title: "Color::transparent"
tags: []
signature: "pub const fn transparent () -> Self"
---

Alpha 0: "nothing said", which every consumer reads as the theme's.

# Examples

```rust
use crawrad_ir::Color;

let c = Color::transparent();
assert_eq!(c.a, 0);
```
