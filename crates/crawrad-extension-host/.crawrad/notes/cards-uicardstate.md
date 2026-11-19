---
id: cards-uicardstate
kind: doc
target_file: src/cards.rs
line_hint: 10
author: 
created: 
title: "UiCardState"
tags: []
signature: "pub struct UiCardState"
---

Live state of an interactive extension card in the IDE dock.

# Examples

```rust
use crawrad_extension_api::contribution::{CardArea, CardContribution};
use crawrad_extension_host::UiCardState;

let contrib = CardContribution {
    id: "card1".to_string(),
    title: "Card 1".to_string(),
    area: CardArea::Bottom,
    icon: None,
    closeable: true,
};
let state = UiCardState::new("ext1", &contrib);
assert_eq!(state.id, "card1");
```
