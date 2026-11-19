---
id: cards-extensioncardmanager-get-card-mut
kind: doc
target_file: src/cards.rs
line_hint: 113
author: 
created: 
title: "ExtensionCardManager::get_card_mut"
tags: []
signature: "pub fn get_card_mut (& mut self , card_id : & str) -> Option < & mut UiCardState >"
---

Retrieve a mutable reference to a card's state.

# Examples

```rust
use crawrad_extension_host::ExtensionCardManager;

let mut mgr = ExtensionCardManager::new();
assert!(mgr.get_card_mut("nonexistent").is_none());
```
