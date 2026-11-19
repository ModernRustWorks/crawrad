---
id: icon-strip-strip-pixels
kind: doc
target_file: src/icon_strip.rs
line_hint: 29
author: 
created: 
title: "strip_pixels"
tags: []
signature: "pub fn strip_pixels (side : u32 , extra : & [(String , Vec < IconStep >)]) -> (usize , usize , Vec < u32 >)"
---

Generates raw BGRA pixel buffer for the icon strip at the given cell side.

Generates the raw BGRA pixel buffer for the icon strip at the given cell
side (in screen pixels); each cell is `side * OVERSAMPLE` texels wide.
Native cells come first, followed by extra third-party icon cells.

# Examples

```rust
use crawrad_project::icon_strip::strip_pixels;

let (w, h, buf) = strip_pixels(4, &[]);
assert!(w > 0 && h > 0 && !buf.is_empty());
```
