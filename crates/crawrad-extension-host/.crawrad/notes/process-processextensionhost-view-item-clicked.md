---
id: process-processextensionhost-view-item-clicked
kind: doc
target_file: src/process.rs
line_hint: 682
author: 
created: 
title: "ProcessExtensionHost::view_item_clicked"
tags: []
signature: "pub fn view_item_clicked (& mut self , extension : & str , view_id : & str , item_id : & str ,) -> Result < () , ExtensionError >"
---

An item of a view was clicked: its extension is told (#398).

# Examples

```rust
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let mut host = ProcessExtensionHost::new(None, cm);
assert!(host.view_item_clicked("unknown", "v1", "item1").is_err());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the extension is not found.
