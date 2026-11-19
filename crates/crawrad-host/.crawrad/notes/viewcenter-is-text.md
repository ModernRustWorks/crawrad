---
id: viewcenter-is-text
kind: doc
target_file: src/lib.rs
line_hint: 64
author: 
created: 
title: "ViewCenter::is_text"
tags: []
signature: "pub fn is_text (self) -> bool"
---

Returns `true` when the centre shows a text view instead of the canvas.

# Examples

```rust
use crawrad_host::ViewCenter;

assert!(ViewCenter::Code.is_text());
assert!(!ViewCenter::Design.is_text());
```
