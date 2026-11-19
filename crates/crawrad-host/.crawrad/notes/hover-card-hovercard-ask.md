---
id: hover-card-hovercard-ask
kind: doc
target_file: src/hover_card.rs
line_hint: 30
author: 
created: 
title: "HoverCard::ask"
tags: []
signature: "pub fn ask (& mut self , anchor : Anchor , by_key : bool) -> u64"
---

A hover is asked at `anchor`; the ticket its answer must carry.

# Examples

```rust
use crawrad_host::hover_card::{Anchor, HoverCard};

let mut card = HoverCard::default();
let ticket = card.ask(Anchor { x: 0.0, y: 0.0, line_height: 10.0 }, false);
assert_eq!(ticket, 1);
```
