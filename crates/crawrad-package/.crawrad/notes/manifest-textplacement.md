---
id: manifest-textplacement
kind: doc
target_file: src/manifest.rs
line_hint: 781
author: 
created: 
title: "TextPlacement"
tags: []
signature: "pub enum TextPlacement"
---

Where single-line text is drawn.

# Examples

```rust
use crawrad_package::manifest::TextPlacement;

let tp = TextPlacement::Centered;
assert_eq!(tp, TextPlacement::Centered);
```
