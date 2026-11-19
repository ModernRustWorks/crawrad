---
id: drawing-drawing
kind: doc
target_file: src/drawing.rs
line_hint: 199
author: 
created: 
title: "Drawing"
tags: []
signature: "pub type Drawing = Vec < Verb >"
---

The drawing of a class, in verbs.

**The drawing of a class, in verbs.**

Empty for every native component: their drawing is code, in
`crawrad-runtime-makepad/src/components/`, and that is how it stays. This vocabulary
exists for whoever **cannot** bring code — and not to rewrite what already
works.

# Examples

```rust
use crawrad_ir::drawing::Drawing;

let drawing: Drawing = Vec::new();
assert!(drawing.is_empty());
```
