---
id: splash-window-fill
kind: doc
target_file: src/splash_window.rs
line_hint: 16
author: 
created: 
title: "Fill"
tags: []
signature: "pub struct Fill"
---

A filled rectangle, `0x00RRGGBB`.

# Examples

```rust
use crawrad_os::splash_window::Fill;

let fill = Fill { area: [0.0, 0.0, 10.0, 10.0], color: 0x00FF0000 };
assert_eq!(fill.color, 0x00FF0000);
```
