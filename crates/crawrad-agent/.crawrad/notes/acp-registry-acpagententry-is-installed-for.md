---
id: acp-registry-acpagententry-is-installed-for
kind: doc
target_file: src/acp_registry.rs
line_hint: 224
author: 
created: 
title: "AcpAgentEntry::is_installed_for"
tags: []
signature: "pub fn is_installed_for (& self , platform : & str , agents_dir : & std :: path :: Path) -> bool"
---

Detects if the agent is installed for the specified platform and agents dir.

# Examples

```rust
use crawrad_agent::AcpAgentEntry;
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
assert!(!entry.is_installed_for("windows-x86_64", Path::new("agents")));
```
