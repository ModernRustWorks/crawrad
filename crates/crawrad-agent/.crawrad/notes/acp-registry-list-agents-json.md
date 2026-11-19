---
id: acp-registry-list-agents-json
kind: doc
target_file: src/acp_registry.rs
line_hint: 1004
author: 
created: 
title: "list_agents_json"
tags: []
signature: "pub fn list_agents_json (cfg : & crate :: AcpConfig , platform : & str , agents_dir : & std :: path :: Path ,) -> serde_json :: Value"
---

Builds JSON list of agents for the `acp.list_agents` MCP tool.

# Examples

```rust
use crawrad_agent::AcpConfig;
use crawrad_agent::acp_registry::list_agents_json;
use std::path::Path;

let cfg = AcpConfig::default();
let json = list_agents_json(&cfg, "windows-x86_64", Path::new("agents"));
assert!(json.is_array());
```
