---
id: process-processextensionhost-unregister
kind: doc
target_file: src/process.rs
line_hint: 762
author: 
created: 
title: "ProcessExtensionHost::unregister"
tags: []
signature: "pub fn unregister (& mut self , id : & str) -> bool"
---

Unregisters an external extension by its identifier.

Extension `id` taken out: its process stopped, its actions, marks and
views forgotten. `false` when it was not here.

# Examples

```rust
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let mut host = ProcessExtensionHost::new(None, cm);
assert!(!host.unregister("unknown"));
```
