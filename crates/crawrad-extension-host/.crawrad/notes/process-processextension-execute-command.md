---
id: process-processextension-execute-command
kind: doc
target_file: src/process.rs
line_hint: 434
author: 
created: 
title: "ProcessExtension::execute_command"
tags: []
signature: "pub fn execute_command (& mut self , command : & str , arguments : Option < Value > ,) -> Result < Option < Value > , ExtensionError >"
---

Execute a command in the extension subprocess.

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
let mut ext = ProcessExtension::new(manifest, PathBuf::from("nonexistent"), None);
assert!(ext.execute_command("cmd", None).is_err());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the extension cannot be started or command execution fails.
