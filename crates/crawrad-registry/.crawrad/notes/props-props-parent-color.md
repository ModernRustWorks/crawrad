---
id: props-props-parent-color
kind: doc
target_file: src/props.rs
line_hint: 208
author: 
created: 
title: "Props::parent_color"
tags: []
signature: "pub fn parent_color (default : bool) -> PropDesc"
---

The control takes its parent's `Color` (`ParentColor`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::parent_color(true);
assert_eq!(p.key, "parent_color");
```
