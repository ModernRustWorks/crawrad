---
id: cards-extensioncardmanager-list-cards
kind: doc
target_file: src/cards.rs
line_hint: 118
author: 
created: 
title: "ExtensionCardManager::list_cards"
tags: []
signature: "pub fn list_cards (& self) -> Vec < & UiCardState >"
---

List all currently known cards.

# Examples

```rust
use crawrad_extension_host::ExtensionCardManager;

let mgr = ExtensionCardManager::new();
assert_eq!(mgr.list_cards().len(), 0);
```
