---
id: hover-card-hovercard-shown
kind: doc
target_file: src/hover_card.rs
line_hint: 67
author: 
created: 
title: "HoverCard::shown"
tags: []
signature: "pub fn shown (& self) -> Option < Anchor >"
---

The name the card up points at, if one is up.

# Examples

```rust
use crawrad_host::hover_card::HoverCard;

let card = HoverCard::default();
assert_eq!(card.shown(), None);
```
