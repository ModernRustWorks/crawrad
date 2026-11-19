---
id: desktop-windowplace-new
kind: doc
target_file: src/desktop.rs
line_hint: 26
author: 
created: 
title: "WindowPlace::new"
tags: []
signature: "pub fn new ([x , y , width , height] : [f64 ; 4] , maximized : bool) -> Self"
---

From `[x, y, width, height]` and the maximized flag.

# Examples

```rust
use crawrad_project::desktop::WindowPlace;

let place = WindowPlace::new([10.0, 20.0, 1000.0, 700.0], true);
assert!(place.maximized);
```
