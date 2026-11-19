---
id: props-props-parent-font
kind: doc
target_file: src/props.rs
line_hint: 197
author: 
created: 
title: "Props::parent_font"
tags: []
signature: "pub fn parent_font (default : bool) -> PropDesc"
---

The control takes its parent's `Font` (`ParentFont`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::parent_font(true);
assert_eq!(p.key, "parent_font");
```
