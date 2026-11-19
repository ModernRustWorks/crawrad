---
id: process-processextensionhost-take-messages
kind: doc
target_file: src/process.rs
line_hint: 753
author: 
created: 
title: "ProcessExtensionHost::take_messages"
tags: []
signature: "pub fn take_messages (& self) -> Vec < (MessageLevel , String) >"
---

Retrieve and clear messages across all active external extension processes.

# Examples

```rust
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let host = ProcessExtensionHost::new(None, cm);
assert!(host.take_messages().is_empty());
```
