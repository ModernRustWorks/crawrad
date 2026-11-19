---
id: cards-extensioncardmanager-get-card
kind: doc
target_file: src/cards.rs
line_hint: 108
author: 
created: 
title: "ExtensionCardManager::get_card"
tags: []
signature: "pub fn get_card (& self , card_id : & str) -> Option < & UiCardState >"
---

Retrieve an immutable reference to a card's state.

# Examples

```rust
use crawrad_extension_host::ExtensionCardManager;

let mgr = ExtensionCardManager::new();
assert!(mgr.get_card("nonexistent").is_none());
```
