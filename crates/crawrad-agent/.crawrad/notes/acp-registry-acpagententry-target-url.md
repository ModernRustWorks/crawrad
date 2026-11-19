---
id: acp-registry-acpagententry-target-url
kind: doc
target_file: src/acp_registry.rs
line_hint: 308
author: 
created: 
title: "AcpAgentEntry::target_url"
tags: []
signature: "pub fn target_url (& self) -> & str"
---

Destination URL for external navigation.

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
    package_cmd: "test".to_string(),
    transport: "stdio".to_string(),
    provider_key: "test".to_string(),
    default_model: "auto".to_string(),
    supported_models: Vec::new(),
    tags: Vec::new(),
    repository: Some("https://github.com/example/repo".to_string()),
    website: None,
    distribution: None,
};
assert_eq!(entry.target_url(), "https://github.com/example/repo");
```
