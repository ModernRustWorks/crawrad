---
id: process-processextension-shutdown
kind: doc
target_file: src/process.rs
line_hint: 555
author: 
created: 
title: "ProcessExtension::shutdown"
tags: []
signature: "pub fn shutdown (& mut self) -> Result < () , ExtensionError >"
---

Gracefully shutdown the extension subprocess.

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
let mut ext = ProcessExtension::new(manifest, PathBuf::from("bin/ext"), None);
assert!(ext.shutdown().is_ok());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if communication fails during shutdown.
