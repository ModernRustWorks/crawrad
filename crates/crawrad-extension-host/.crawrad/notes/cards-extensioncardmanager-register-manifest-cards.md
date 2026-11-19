---
id: cards-extensioncardmanager-register-manifest-cards
kind: doc
target_file: src/cards.rs
line_hint: 48
author: 
created: 
title: "ExtensionCardManager::register_manifest_cards"
tags: []
signature: "pub fn register_manifest_cards (& mut self , manifest : & ExtensionManifest)"
---

Register all cards declared in an extension manifest.

# Examples

```rust
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_host::ExtensionCardManager;

let mut mgr = ExtensionCardManager::new();
let manifest = ExtensionManifest {
    id: "ext1".to_string(),
    name: "Ext".to_string(),
    version: "0.1.0".to_string(),
    publisher: "Acme".to_string(),
    description: String::new(),
    entrypoint: None,
    activation_events: Vec::new(),
    contributions: ExtensionContributions::default(),
};
mgr.register_manifest_cards(&manifest);
```
