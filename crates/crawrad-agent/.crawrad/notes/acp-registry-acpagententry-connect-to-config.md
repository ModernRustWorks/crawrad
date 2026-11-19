---
id: acp-registry-acpagententry-connect-to-config
kind: doc
target_file: src/acp_registry.rs
line_hint: 316
author: 
created: 
title: "AcpAgentEntry::connect_to_config"
tags: []
signature: "pub fn connect_to_config (& self , cfg : & mut crate :: AcpConfig) -> Result < String , String >"
---

Connects this agent to the active ACP configuration.

# Errors

Returns an error if launch parameters cannot be resolved or binary is missing.

# Examples

```rust
use crawrad_agent::{AcpAgentEntry, AcpConfig};

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
    distribution: Some(serde_json::json!({
        "npx": { "package": "test-pkg" }
    })),
};
let mut cfg = AcpConfig::default();
let res = entry.connect_to_config(&mut cfg);
assert!(res.is_ok());
```
