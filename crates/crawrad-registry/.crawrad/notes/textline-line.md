---
id: textline-line
kind: doc
target_file: src/lib.rs
line_hint: 230
author: 
created: 
title: "TextLine::line"
tags: []
signature: "pub fn line (self , y : f32 , h : f32) -> f32"
---

Calculates the baseline coordinate of the text.

The **baseline** of the text — the line on which letters rest, which is
where the classic RAD IDE draws the pink guideline. It sits *below* the letters, so
it and the spacing indicator do not interfere on the screen.

# Examples

```rust
use crawrad_registry::TextLine;

let y = TextLine::Top(2.0).line(10.0, 24.0);
assert_eq!(y, 10.0 + 2.0 + TextLine::ASCENT);
```
