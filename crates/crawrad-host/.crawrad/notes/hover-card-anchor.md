---
id: hover-card-anchor
kind: doc
target_file: src/hover_card.rs
line_hint: 5
author: 
created: 
title: "Anchor"
tags: []
signature: "pub struct Anchor"
---

The name the card is about, on screen: its top-left and line height.

# Examples

```rust
use crawrad_host::hover_card::Anchor;

let anchor = Anchor { x: 10.0, y: 20.0, line_height: 14.0 };
assert_eq!(anchor.x, 10.0);
```
