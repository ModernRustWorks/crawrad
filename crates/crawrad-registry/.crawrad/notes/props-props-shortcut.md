---
id: props-props-shortcut
kind: doc
target_file: src/props.rs
line_hint: 576
author: 
created: 
title: "Props::shortcut"
tags: []
signature: "pub fn shortcut (default : impl Into < crawrad_ir :: Shortcut >) -> PropDesc"
---

Keyboard shortcut (`ShortCut` / `Shortcut`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::shortcut("Ctrl+S");
assert_eq!(p.key, "shortcut");
```
