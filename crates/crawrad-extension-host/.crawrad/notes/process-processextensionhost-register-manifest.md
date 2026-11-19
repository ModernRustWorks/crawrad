---
id: process-processextensionhost-register-manifest
kind: doc
target_file: src/process.rs
line_hint: 607
author: 
created: 
title: "ProcessExtensionHost::register_manifest"
tags: []
signature: "pub fn register_manifest (& mut self , manifest : ExtensionManifest ,) -> Result < bool , ExtensionError >"
---

Register an external extension from its manifest.

# Examples

```rust
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let mut host = ProcessExtensionHost::new(None, cm);
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
assert_eq!(host.register_manifest(manifest).unwrap(), false);
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if registering fails.
