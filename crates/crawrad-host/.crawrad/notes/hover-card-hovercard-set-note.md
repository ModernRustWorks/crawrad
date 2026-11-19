---
id: hover-card-hovercard-set-note
kind: doc
target_file: src/hover_card.rs
line_hint: 57
author: 
created: 
title: "HoverCard::set_note"
tags: []
signature: "pub fn set_note (& mut self , note : Option < (String , std :: path :: PathBuf , usize) >)"
---

The card up shows the note `id` of the item at `file`:`line` (#659).

# Examples

```rust
use crawrad_host::hover_card::HoverCard;

let mut card = HoverCard::default();
card.set_note(None);
assert_eq!(card.note(), None);
```
