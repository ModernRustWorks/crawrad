---
id: process-processextension-notify
kind: doc
target_file: src/process.rs
line_hint: 516
author: 
created: 
title: "ProcessExtension::notify"
tags: []
signature: "pub fn notify (& mut self , method : & str , params : Value) -> Result < () , ExtensionError >"
---

A notification to the extension (no answer awaited).

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
assert!(ext.notify("custom", serde_json::json!({})).is_err());
```

# Errors

Returns [`ExtensionError::IpcError`](crawrad_extension_api::error::ExtensionError::IpcError) if stdin is unavailable or writing fails.
