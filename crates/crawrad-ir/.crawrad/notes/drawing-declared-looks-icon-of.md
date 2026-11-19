---
id: drawing-declared-looks-icon-of
kind: doc
target_file: src/drawing.rs
line_hint: 222
author: 
created: 
title: "icon_of"
tags: []
signature: "pub fn icon_of (& self , class : & str) -> & [crate :: icon :: IconStep]"
---

The palette icon of `class`, or an empty slice when the class has none.

The shell fills `icons` from the catalogue; the canvas reads it to draw the
non-visual badge with the same drawing the palette tile shows.

# Examples

```rust
use crawrad_ir::drawing::DeclaredLooks;

assert!(DeclaredLooks::default().icon_of("Timer").is_empty());
```
