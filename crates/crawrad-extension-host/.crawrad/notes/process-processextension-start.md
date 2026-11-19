---
id: process-processextension-start
kind: doc
target_file: src/process.rs
line_hint: 129
author: 
created: 
title: "ProcessExtension::start"
tags: []
signature: "pub fn start (& mut self) -> Result < () , ExtensionError >"
---

Launch the extension subprocess and perform initialization.

Launch the extension subprocess and perform the `extension/initialize` handshake.

# Examples

```rust
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_host::ProcessExtension;
use std::path::PathBuf;

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
let mut ext = ProcessExtension::new(manifest, PathBuf::from("nonexistent_binary"), None);
assert!(ext.start().is_err());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the binary is missing, spawning fails, or the handshake fails.
