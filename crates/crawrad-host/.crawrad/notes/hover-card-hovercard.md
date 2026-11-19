---
id: hover-card-hovercard
kind: doc
target_file: src/hover_card.rs
line_hint: 13
author: 
created: 
title: "HoverCard"
tags: []
signature: "pub struct HoverCard"
---

The card's pending request and whether it is up.

# Examples

```rust
use crawrad_host::hover_card::HoverCard;

let card = HoverCard::default();
assert!(card.shown().is_none());
```
