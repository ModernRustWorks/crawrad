---
id: process-processextension-is-alive
kind: doc
target_file: src/process.rs
line_hint: 102
author: 
created: 
title: "ProcessExtension::is_alive"
tags: []
signature: "pub fn is_alive (& mut self) -> bool"
---

Check if the child process is currently running.

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
assert!(!ext.is_alive());
```
