---
id: cards-extensioncardmanager-forget
kind: doc
target_file: src/cards.rs
line_hint: 56
author: 
created: 
title: "ExtensionCardManager::forget"
tags: []
signature: "pub fn forget (& mut self , extension_id : & str)"
---

The cards of `extension_id` taken out (it was uninstalled).

# Examples

```rust
use crawrad_extension_host::ExtensionCardManager;

let mut mgr = ExtensionCardManager::new();
mgr.forget("ext1");
assert!(mgr.list_cards().is_empty());
```
