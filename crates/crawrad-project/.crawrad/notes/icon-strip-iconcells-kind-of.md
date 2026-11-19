---
id: icon-strip-iconcells-kind-of
kind: doc
target_file: src/icon_strip.rs
line_hint: 76
author: 
created: 
title: "IconCells::kind_of"
tags: []
signature: "pub fn kind_of (& self , class : & str) -> Option < f64 >"
---

Returns the kind index of a class (native first, then extra icons).

The kind index of a class: native first, or the offset into extra third-party icons.

# Examples

```rust
use crawrad_project::icon_strip::IconCells;

let cells = IconCells::default();
assert!(cells.kind_of("Button").is_some());
```
