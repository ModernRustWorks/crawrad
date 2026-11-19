---
id: process-processextension-new
kind: doc
target_file: src/process.rs
line_hint: 74
author: 
created: 
title: "ProcessExtension::new"
tags: []
signature: "pub fn new (manifest : ExtensionManifest , binary_path : PathBuf , workspace_root : Option < PathBuf > ,) -> Self"
---

Create a new process extension definition.

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
