---
id: desktop-desktop-new
kind: doc
target_file: src/desktop.rs
line_hint: 58
author: 
created: 
title: "Desktop::new"
tags: []
signature: "pub fn new (window : WindowPlace , dock : String) -> Self"
---

A desktop with a window and a dock, no tabs yet (see [`Self::with_tabs`]).

# Examples

```rust
use crawrad_project::desktop::{Desktop, WindowPlace};

let win = WindowPlace::new([0.0, 0.0, 800.0, 600.0], false);
let d = Desktop::new(win, "dock layout".to_string());
assert!(d.window.is_some());
```
