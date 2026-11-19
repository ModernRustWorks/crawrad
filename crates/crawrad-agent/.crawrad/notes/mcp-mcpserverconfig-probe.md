---
id: mcp-mcpserverconfig-probe
kind: doc
target_file: src/mcp.rs
line_hint: 50
author: 
created: 
title: "McpServerConfig::probe"
tags: []
signature: "pub fn probe (& self) -> bool"
---

Probes if the server executable actually exists and is available in PATH.

# Examples

```rust
use crawrad_agent::{McpServerConfig, McpSource, McpTransport};
use std::collections::HashMap;

let config = McpServerConfig {
    name: "test".into(),
    transport: McpTransport::Stdio,
    command: "non_existent_tool_xyz".into(),
    args: vec![],
    env: HashMap::new(),
    source: McpSource::Custom,
    enabled: true,
};
assert!(!config.probe());
```
