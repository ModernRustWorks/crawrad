---
id: acp-registry-click-agent-action
kind: doc
target_file: src/acp_registry.rs
line_hint: 973
author: 
created: 
title: "click_agent_action"
tags: []
signature: "pub fn click_agent_action (agent : & AcpAgentEntry , cfg : & mut crate :: AcpConfig) -> AgentClickResult"
---

Handles clicking the primary action button for an agent in the catalog.

# Examples

```rust
use crawrad_agent::{AcpAgentEntry, AcpConfig};
use crawrad_agent::acp_registry::{AgentClickResult, click_agent_action};

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
let mut cfg = AcpConfig::default();
let res = click_agent_action(&entry, &mut cfg);
assert!(matches!(res, AgentClickResult::NeedsRequirement(_)));
```
