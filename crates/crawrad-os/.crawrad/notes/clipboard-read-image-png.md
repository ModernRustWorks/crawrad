---
id: clipboard-read-image-png
kind: doc
target_file: src/clipboard.rs
line_hint: 41
author: 
created: 
title: "read_image_png"
tags: []
signature: "pub fn read_image_png () -> Option < Vec < u8 > >"
---

The picture on the clipboard, as PNG bytes, or `None` when it holds none.

A screenshot copied with Print Screen or the Snipping Tool is read here.

# Examples

```rust,no_run
use crawrad_os::clipboard::read_image_png;

if let Some(png) = read_image_png() {
    println!("{} bytes", png.len());
}
```
