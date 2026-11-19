---
id: process-processextension-send-request
kind: doc
target_file: src/process.rs
line_hint: 326
author: 
created: 
title: "ProcessExtension::send_request"
tags: []
signature: "pub fn send_request (& mut self , method : & str , params : Option < Value > , timeout : Duration ,) -> Result < Value , ExtensionError >"
---

Send a JSON-RPC request to the child process and await its response.

# Examples

```rust
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_host::ProcessExtension;
use std::path::PathBuf;
use std::time::Duration;

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
assert!(ext.send_request("ping", None, Duration::from_millis(100)).is_err());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the process is not alive, serialization fails, or timeout expires.
