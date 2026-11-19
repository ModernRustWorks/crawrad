---
id: process-processextensionhost-line-mark-clicked
kind: doc
target_file: src/process.rs
line_hint: 662
author: 
created: 
title: "ProcessExtensionHost::line_mark_clicked"
tags: []
signature: "pub fn line_mark_clicked (& mut self , extension : & str , id : & str , file : & str ,) -> Result < () , ExtensionError >"
---

A line mark's icon was clicked: its extension is told (#398).

# Examples

```rust
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let mut host = ProcessExtensionHost::new(None, cm);
assert!(host.line_mark_clicked("unknown", "mark1", "main.rs").is_err());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the extension is not found.
