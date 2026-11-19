---
id: acp-registry-launch-for
kind: doc
target_file: src/acp_registry.rs
line_hint: 115
author: 
created: 
title: "launch_for"
tags: []
signature: "pub fn launch_for (entry : & AcpAgentEntry , platform : & str , agents_dir : & std :: path :: Path ,) -> Result < AcpLaunch , String >"
---

Resolves launch command and arguments for an agent entry on the given platform.

# Errors

Returns an error if distribution details are missing or invalid for the platform.

# Examples

```rust
use crawrad_agent::{AcpAgentEntry, launch_for};
use std::path::Path;

let entry = AcpAgentEntry {
    id: "test".to_string(),
    name: "Test Agent".to_string(),
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
let launch = launch_for(&entry, "windows-x86_64", Path::new("agents")).unwrap();
assert_eq!(launch.program, "npx");
```
