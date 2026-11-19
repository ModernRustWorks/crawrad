---
id: layout-align-is-fixed
kind: doc
target_file: src/layout.rs
line_hint: 51
author: 
created: 
title: "Align::is_fixed"
tags: []
signature: "pub fn is_fixed (self) -> bool"
---

Returns whether the alignment fixes the component position.

An aligned component cannot be dragged or resized by hand —
layout is in command, as in the classic IDE.

# Examples

```rust
use crawrad_designer::layout::Align;

assert!(Align::Top.is_fixed());
assert!(!Align::None.is_fixed());
```
