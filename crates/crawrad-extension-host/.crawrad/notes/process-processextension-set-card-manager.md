---
id: process-processextension-set-card-manager
kind: doc
target_file: src/process.rs
line_hint: 97
author: 
created: 
title: "ProcessExtension::set_card_manager"
tags: []
signature: "pub fn set_card_manager (& mut self , mgr : Arc < Mutex < ExtensionCardManager > >)"
---

Attach card manager for reactive UI cards dispatch.

# Examples

```rust
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_host::{ExtensionCardManager, ProcessExtension};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
let mut ext = ProcessExtension::new(manifest, PathBuf::from("bin/ext"), None);
ext.set_card_manager(Arc::new(Mutex::new(ExtensionCardManager::new())));
assert!(ext.card_manager.is_some());
```
