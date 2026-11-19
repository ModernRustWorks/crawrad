---
id: hover-card-hovercard-answered
kind: doc
target_file: src/hover_card.rs
line_hint: 41
author: 
created: 
title: "HoverCard::answered"
tags: []
signature: "pub fn answered (& mut self , ticket : u64) -> Option < (Anchor , bool) >"
---

Consumes the pending request matching `ticket` if valid.

The answer for `ticket` arrived: its anchor and whether asked by
key, once; an older ticket gets nothing.

# Examples

```rust
use crawrad_host::hover_card::{Anchor, HoverCard};

let mut card = HoverCard::default();
let ticket = card.ask(Anchor { x: 0.0, y: 0.0, line_height: 10.0 }, false);
assert!(card.answered(ticket).is_some());
```
