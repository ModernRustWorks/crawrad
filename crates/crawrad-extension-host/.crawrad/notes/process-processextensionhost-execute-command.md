---
id: process-processextensionhost-execute-command
kind: doc
target_file: src/process.rs
line_hint: 733
author: 
created: 
title: "ProcessExtensionHost::execute_command"
tags: []
signature: "pub fn execute_command (& mut self , action_id : & str , arguments : Option < Value > ,) -> Result < Option < Value > , ExtensionError >"
---

Execute a command in the target external extension process.

# Examples

```rust
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let mut host = ProcessExtensionHost::new(None, cm);
assert!(host.execute_command("cmd", None).is_err());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the action ID or extension is not found.
