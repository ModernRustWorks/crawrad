---
id: hover-card-hovercard-close
kind: doc
target_file: src/hover_card.rs
line_hint: 79
author: 
created: 
title: "HoverCard::close"
tags: []
signature: "pub fn close (& mut self) -> bool"
---

Closes the card and clears any pending request.

The mouse left, a key was pressed: no card and nothing awaited.
Whether a card was up.

# Examples

```rust
use crawrad_host::hover_card::HoverCard;

let mut card = HoverCard::default();
assert!(!card.close());
```
