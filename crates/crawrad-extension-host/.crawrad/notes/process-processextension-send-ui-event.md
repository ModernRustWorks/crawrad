---
id: process-processextension-send-ui-event
kind: doc
target_file: src/process.rs
line_hint: 473
author: 
created: 
title: "ProcessExtension::send_ui_event"
tags: []
signature: "pub fn send_ui_event (& mut self , card_id : & str , element_id : & str , event : UiEventKind ,) -> Result < () , ExtensionError >"
---

Send an interactive UI event notification from IDE to the extension subprocess.

# Examples

```rust
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_api::ui::UiEventKind;
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
assert!(ext.send_ui_event("c1", "e1", UiEventKind::Click).is_err());
```

# Errors

Returns [`ExtensionError::IpcError`](crawrad_extension_api::error::ExtensionError::IpcError) if stdin is not available or writing fails.
