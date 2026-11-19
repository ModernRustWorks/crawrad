---
id: props-props-window-state
kind: doc
target_file: src/props.rs
line_hint: 1190
author: 
created: 
title: "Props::window_state"
tags: []
signature: "pub fn window_state (default : & 'static str) -> PropDesc"
---

How the window opens (`WindowState`): normal, maximized or minimized.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::window_state("wsNormal");
assert_eq!(p.key, "window_state");
```
