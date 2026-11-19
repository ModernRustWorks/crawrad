---
id: acp-registry-acpagententry-action-button-text
kind: doc
target_file: src/acp_registry.rs
line_hint: 259
author: 
created: 
title: "AcpAgentEntry::action_button_text"
tags: []
signature: "pub fn action_button_text (& self , is_active : bool) -> & 'static str"
---

Primary action button label for this agent in the catalog.

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
    distribution: None,
};
assert_eq!(entry.action_button_text(true), "In use");
```
