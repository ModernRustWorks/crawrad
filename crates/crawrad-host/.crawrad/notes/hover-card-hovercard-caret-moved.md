---
id: hover-card-hovercard-caret-moved
kind: doc
target_file: src/hover_card.rs
line_hint: 72
author: 
created: 
title: "HoverCard::caret_moved"
tags: []
signature: "pub fn caret_moved (& mut self , caret : (usize , usize)) -> bool"
---

Updates the caret position and reports if the caret moved while open.

The caret moved while the card is up — once per move. A `)` typed over
its auto-closed twin moves the caret with no edit (#394), and a card
that only followed edits stayed open after the call was left.

# Examples

```rust
use crawrad_host::hover_card::HoverCard;

let mut card = HoverCard::default();
assert!(!card.caret_moved((1, 2)));
```
