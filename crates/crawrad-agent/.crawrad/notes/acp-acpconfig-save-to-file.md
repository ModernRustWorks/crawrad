---
id: acp-acpconfig-save-to-file
kind: doc
target_file: src/acp.rs
line_hint: 836
author: 
created: 
title: "AcpConfig::save_to_file"
tags: []
signature: "pub fn save_to_file (& self , path : & Path) -> Result < () , std :: io :: Error >"
---

Atomically saves the ACP config to file.

Atomically saves the ACP config using read-modify-write to preserve unknown third-party keys.

# Errors

Returns an error if creating directories, serializing JSON, or writing fails.

# Examples

```rust,no_run
// Writes configuration to filesystem.
use crawrad_agent::AcpConfig;
use std::path::Path;

let config = AcpConfig::default();
let res = config.save_to_file(Path::new("config.json"));
assert!(res.is_ok());
```
