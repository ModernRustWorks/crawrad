---
id: hover-card-hovercard-note
kind: doc
target_file: src/hover_card.rs
line_hint: 62
author: 
created: 
title: "HoverCard::note"
tags: []
signature: "pub fn note (& self) -> Option < & (String , std :: path :: PathBuf , usize) >"
---

The note of the card up, if it is a noted item's.

# Examples

```rust
use crawrad_host::hover_card::HoverCard;

let card = HoverCard::default();
assert!(card.note().is_none());
```
