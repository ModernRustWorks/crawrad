---
id: outcome
kind: doc
target_file: src/lib.rs
line_hint: 264
author: 
created: 
title: "Outcome"
tags: []
signature: "pub struct Outcome"
---

What the designer wants to happen after a pointer event.

# Examples

```rust
use crawrad_designer::Outcome;

let o = Outcome::default();
assert!(!o.dragging);
```
