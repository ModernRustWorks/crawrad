---
id: process-processextensionhost-can-handle
kind: doc
target_file: src/process.rs
line_hint: 728
author: 
created: 
title: "ProcessExtensionHost::can_handle"
tags: []
signature: "pub fn can_handle (& self , action_id : & str) -> bool"
---

Check if an action ID belongs to an external extension.

# Examples

```rust
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let host = ProcessExtensionHost::new(None, cm);
assert!(!host.can_handle("action"));
```
