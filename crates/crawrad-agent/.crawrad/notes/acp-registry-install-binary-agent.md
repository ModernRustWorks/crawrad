---
id: acp-registry-install-binary-agent
kind: doc
target_file: src/acp_registry.rs
line_hint: 858
author: 
created: 
title: "install_binary_agent"
tags: []
signature: "pub fn install_binary_agent (entry : & AcpAgentEntry , platform : & str , agents_dir : & std :: path :: Path ,) -> Result < PathBuf , String >"
---

Downloads and extracts a binary agent into `agents_dir/<id>/<version>/`.

# Errors

Returns an error if the agent is not a binary distribution or download fails.

# Examples

```rust,no_run
// Downloads and extracts binary archive over network.
use crawrad_agent::{AcpAgentEntry, install_binary_agent};
use std::path::Path;

let entry = AcpAgentEntry {
    id: "test".to_string(),
    name: "Test".to_string(),
    version: None,
    author: "Test".to_string(),
    badge: "official".to_string(),
    description: "Test".to_string(),
    package_cmd: "test".to_string(),
    transport: "stdio".to_string(),
    provider_key: "test".to_string(),
    default_model: "auto".to_string(),
    supported_models: Vec::new(),
    tags: Vec::new(),
    repository: None,
    website: None,
    distribution: None,
};
let res = install_binary_agent(&entry, "windows-x86_64", Path::new("agents"));
assert!(res.is_err());
```
