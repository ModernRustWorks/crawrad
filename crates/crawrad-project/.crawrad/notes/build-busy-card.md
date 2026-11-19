---
id: build-busy-card
kind: doc
target_file: src/build_busy.rs
line_hint: 38
author: 
created: 
title: "Card"
tags: []
signature: "pub struct Card"
---

The corner card, shown only while the Build window is CLOSED.

# Examples

```rust
use crawrad_project::build_busy::Card;

let card = Card {
    icon: "✓".to_string(),
    headline: "Build succeeded".to_string(),
    detail: "1 package".to_string(),
    clock: "0.5s".to_string(),
    fraction: Some(1.0),
    running: false,
};
assert!(!card.running);
```
