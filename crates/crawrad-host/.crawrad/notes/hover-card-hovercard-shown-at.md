---
id: hover-card-hovercard-shown-at
kind: doc
target_file: src/hover_card.rs
line_hint: 52
author: 
created: 
title: "HoverCard::shown_at"
tags: []
signature: "pub fn shown_at (& mut self , anchor : Anchor)"
---

The card is up, pointing at `anchor`.

# Examples

```rust
use crawrad_host::hover_card::{Anchor, HoverCard};

let mut card = HoverCard::default();
let anchor = Anchor { x: 5.0, y: 10.0, line_height: 12.0 };
card.shown_at(anchor);
assert_eq!(card.shown(), Some(anchor));
```
