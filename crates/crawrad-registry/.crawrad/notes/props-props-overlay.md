---
id: props-props-overlay
kind: doc
target_file: src/props.rs
line_hint: 2083
author: 
created: 
title: "Props::overlay"
tags: []
signature: "pub fn overlay (default : bool) -> PropDesc"
---

Whether a dimmed scrim covers the form for an open `Drawer` (`Overlay`, #500).

Whether a dimmed scrim covers the rest of the form when a `Drawer` is open (`Overlay`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::overlay(true);
assert_eq!(p.key, "overlay");
```
