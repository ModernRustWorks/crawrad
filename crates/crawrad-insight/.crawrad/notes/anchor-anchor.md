---
id: anchor-anchor
kind: doc
target_file: src/anchor.rs
line_hint: 9
author: 
created: 
title: "Anchor"
tags: []
signature: "pub struct Anchor"
---

The list's place, from the window's coordinates of the caret.

# Examples

```rust
use crawrad_insight::anchor::Anchor;

let a = Anchor { left: 10.0, top: 20.0, word: [10.0, 20.0] };
assert_eq!(a.left, 10.0);
```
