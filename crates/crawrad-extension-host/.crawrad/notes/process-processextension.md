---
id: process-processextension
kind: doc
target_file: src/process.rs
line_hint: 54
author: 
created: 
title: "ProcessExtension"
tags: []
signature: "pub struct ProcessExtension"
---

A decoupled extension running as an independent subprocess.

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
let ext = ProcessExtension::new(manifest, PathBuf::from("bin/ext"), None);
assert_eq!(ext.manifest.id, "ext1");
```
