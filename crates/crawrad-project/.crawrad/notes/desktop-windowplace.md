---
id: desktop-windowplace
kind: doc
target_file: src/desktop.rs
line_hint: 15
author: 
created: 
title: "WindowPlace"
tags: []
signature: "pub struct WindowPlace"
---

The window bounds in physical pixels, and whether it was maximized.

The window, in physical pixels, and whether it was maximized. The
rectangle is the one Restore goes back to, so a maximized window keeps it.

# Examples

```rust
use crawrad_project::desktop::WindowPlace;

let wp = WindowPlace::new([0.0, 0.0, 800.0, 600.0], false);
assert!(!wp.maximized);
```
