---
id: acp-registry-mcp-install-agent
kind: doc
target_file: src/acp_registry.rs
line_hint: 1079
author: 
created: 
title: "mcp_install_agent"
tags: []
signature: "pub fn mcp_install_agent (id : & str ,) -> Result < (String , std :: sync :: mpsc :: Receiver < Result < String , String > >) , String >"
---

MCP tool handler for `acp.install_agent`.

# Errors

Returns an error if the agent is not found or has no binary distribution.

# Examples

```rust
use crawrad_agent::acp_registry::mcp_install_agent;

let res = mcp_install_agent("unknown-agent-id");
assert!(res.is_err());
```
