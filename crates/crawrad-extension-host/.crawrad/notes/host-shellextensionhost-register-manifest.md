---
id: host-shellextensionhost-register-manifest
kind: doc
target_file: src/host.rs
line_hint: 184
author: 
created: 
title: "ShellExtensionHost::register_manifest"
tags: []
signature: "pub fn register_manifest (& mut self , manifest : ExtensionManifest) -> Result < () , ExtensionError >"
---

Register a declarative extension manifest into the host catalog.

# Examples

```rust
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_host::ShellExtensionHost;

let mut host = ShellExtensionHost::default();
let manifest = ExtensionManifest {
    id: "ext.sample".to_string(),
    name: "Sample".to_string(),
    version: "0.1.0".to_string(),
    publisher: "Acme".to_string(),
    description: String::new(),
    entrypoint: None,
    activation_events: Vec::new(),
    contributions: ExtensionContributions::default(),
};
assert!(host.register_manifest(manifest).is_ok());
```

# Errors

Returns [`ExtensionError::InvalidManifest`](crawrad_extension_api::error::ExtensionError::InvalidManifest) if validation fails, or [`ExtensionError::Duplicate`](crawrad_extension_api::error::ExtensionError::Duplicate) if already registered.
