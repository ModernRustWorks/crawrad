---
id: process-processextension-ping
kind: doc
target_file: src/process.rs
line_hint: 536
author: 
created: 
title: "ProcessExtension::ping"
tags: []
signature: "pub fn ping (& mut self) -> Result < bool , ExtensionError >"
---

Check subprocess liveness with a ping/pong heartbeat.

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
assert!(ext.ping().is_err());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the child process is not alive or times out.
