---
id: process-processextensionhost
kind: doc
target_file: src/process.rs
line_hint: 580
author: 
created: 
title: "ProcessExtensionHost"
tags: []
signature: "pub struct ProcessExtensionHost"
---

Host manager for external extension subprocesses.

# Examples

```rust
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let host = ProcessExtensionHost::new(None, cm);
assert!(host.extensions.is_empty());
```
