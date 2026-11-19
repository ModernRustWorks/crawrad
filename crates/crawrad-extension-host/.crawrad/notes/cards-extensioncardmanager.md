---
id: cards-extensioncardmanager
kind: doc
target_file: src/cards.rs
line_hint: 36
author: 
created: 
title: "ExtensionCardManager"
tags: []
signature: "pub struct ExtensionCardManager"
---

Central manager for extension-contributed UI cards and interactive panels.

# Examples

```rust
use crawrad_extension_host::ExtensionCardManager;

let mgr = ExtensionCardManager::new();
assert!(mgr.list_cards().is_empty());
```
