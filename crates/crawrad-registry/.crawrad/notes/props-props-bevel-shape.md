---
id: props-props-bevel-shape
kind: doc
target_file: src/props.rs
line_hint: 1104
author: 
created: 
title: "Props::bevel_shape"
tags: []
signature: "pub fn bevel_shape (default : & 'static str) -> PropDesc"
---

The visual shape drawn by a `Bevel` (#500).

What a `Bevel` draws (#500): the whole box, a frame, one edge, or
nothing at all (a spacer).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::bevel_shape("Box");
assert_eq!(p.key, "bevel_shape");
```
