---
id: capture
kind: doc
target_file: src/lib.rs
line_hint: 312
author: 
created: 
title: "Capture"
tags: []
signature: "pub struct Capture"
---

Pixels captured from the main window's client area.

`rgba` holds `width * height` pixels, rows top-down, 4 bytes each
(R, G, B, A with A = 255).

# Examples

```
let capture = crawrad_os::Capture { width: 1, height: 1, rgba: vec![0, 0, 0, 255] };
assert_eq!(capture.rgba.len(), 4);
```
