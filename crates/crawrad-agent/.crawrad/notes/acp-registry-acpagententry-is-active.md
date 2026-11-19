---
id: acp-registry-acpagententry-is-active
kind: doc
target_file: src/acp_registry.rs
line_hint: 245
author: 
created: 
title: "AcpAgentEntry::is_active"
tags: []
signature: "pub fn is_active (& self , endpoint : & str , provider : & str) -> bool"
---

Checks if this agent entry matches the currently active endpoint or provider.

# Examples

```rust
use crawrad_agent::AcpAgentEntry;

let entry = AcpAgentEntry {
    id: "test".to_string(),
    name: "Test".to_string(),
    version: None,
    author: "Test".to_string(),
    badge: "official".to_string(),
    description: "Test".to_string(),
    package_cmd: "test-cmd".to_string(),
    transport: "stdio".to_string(),
    provider_key: "test".to_string(),
    default_model: "auto".to_string(),
    supported_models: Vec::new(),
    tags: Vec::new(),
    repository: None,
    website: None,
    distribution: None,
};
assert!(entry.is_active("test-cmd", ""));
```
