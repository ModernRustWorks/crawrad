---
id: splash-window-area
kind: doc
target_file: src/splash_window.rs
line_hint: 13
author: 
created: 
title: "Area"
tags: []
signature: "pub type Area = [f32 ; 4]"
---

A rectangle in 96-DPI pixels: left, top, right, bottom.

# Examples

```rust
use crawrad_os::splash_window::Area;

let area: Area = [0.0, 0.0, 100.0, 100.0];
assert_eq!(area[2], 100.0);
```
