---
id: clipboard-rgba-to-png
kind: doc
target_file: src/clipboard.rs
line_hint: 22
author: 
created: 
title: "rgba_to_png"
tags: []
signature: "pub fn rgba_to_png (width : usize , height : usize , rgba : & [u8]) -> Option < Vec < u8 > >"
---

Encodes an RGBA picture (4 bytes per pixel) as PNG.

`None` when a side is zero or the bytes do not match `width * height * 4`.

# Examples

```rust
use crawrad_os::clipboard::rgba_to_png;

let red = [255, 0, 0, 255];
assert!(rgba_to_png(1, 1, &red).is_some());
assert!(rgba_to_png(2, 1, &red).is_none());
```
