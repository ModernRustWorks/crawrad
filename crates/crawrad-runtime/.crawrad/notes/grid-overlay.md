---
id: grid-overlay
kind: doc
target_file: src/grid.rs
line_hint: 443
author: 
created: 
title: "Overlay"
tags: []
signature: "pub struct Overlay"
---

Loading card displayed over dimmed rows during lengthy operations.

Translucent loading card displayed over dimmed rows during lengthy source operations.

# Examples

```rust
use crawrad_runtime::grid::Overlay;

let overlay = Overlay {
    opacity: 0.8,
    title: "Loading...".into(),
    detail: "Please wait".into(),
};
assert_eq!(overlay.opacity, 0.8);
```
