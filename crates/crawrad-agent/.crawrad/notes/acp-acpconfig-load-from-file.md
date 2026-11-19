---
id: acp-acpconfig-load-from-file
kind: doc
target_file: src/acp.rs
line_hint: 729
author: 
created: 
title: "AcpConfig::load_from_file"
tags: []
signature: "pub fn load_from_file (path : & Path) -> Result < Self , std :: io :: Error >"
---

Loads the ACP config from file, preserving unknown JSON keys.

Loads the ACP config from file, preserving unknown JSON keys into `extra_fields`.

# Errors

Returns an error if the file cannot be read or contains invalid JSON.

# Examples

```rust
use crawrad_agent::AcpConfig;
use std::path::Path;

let config = AcpConfig::load_from_file(Path::new("non_existent_config.json")).unwrap();
assert_eq!(config.agent_id, "claude");
```
