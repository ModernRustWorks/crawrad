---
id: acp-registry-mcp-connect-agent
kind: doc
target_file: src/acp_registry.rs
line_hint: 1100
author: 
created: 
title: "mcp_connect_agent"
tags: []
signature: "pub fn mcp_connect_agent (id : & str , cfg : & mut crate :: AcpConfig ,) -> Result < serde_json :: Value , String >"
---

MCP tool handler for `acp.connect_agent`.

# Errors

Returns an error if the agent is not found in the registry or connection fails.

# Examples

```rust
use crawrad_agent::AcpConfig;
use crawrad_agent::acp_registry::mcp_connect_agent;

let mut cfg = AcpConfig::default();
let res = mcp_connect_agent("unknown-agent-id", &mut cfg);
assert!(res.is_err());
```
