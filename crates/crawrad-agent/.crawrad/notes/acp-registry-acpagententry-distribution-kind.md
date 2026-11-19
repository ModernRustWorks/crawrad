---
id: acp-registry-acpagententry-distribution-kind
kind: doc
target_file: src/acp_registry.rs
line_hint: 210
author: 
created: 
title: "AcpAgentEntry::distribution_kind"
tags: []
signature: "pub fn distribution_kind (& self) -> Option < & 'static str >"
---

Returns the kind of distribution ("npx", "uvx", or "binary").

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
    repository: None,
    website: None,
    distribution: Some(serde_json::json!({ "npx": { "package": "test" } })),
};
assert_eq!(entry.distribution_kind(), Some("npx"));
```
