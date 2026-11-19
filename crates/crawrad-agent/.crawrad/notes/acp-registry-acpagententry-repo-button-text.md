---
id: acp-registry-acpagententry-repo-button-text
kind: doc
target_file: src/acp_registry.rs
line_hint: 297
author: 
created: 
title: "AcpAgentEntry::repo_button_text"
tags: []
signature: "pub fn repo_button_text (& self) -> & 'static str"
---

Secondary button label for repository / documentation.

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
assert_eq!(entry.repo_button_text(), "GitHub ↗");
```
