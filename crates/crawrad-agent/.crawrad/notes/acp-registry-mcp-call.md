---
id: acp-registry-mcp-call
kind: doc
target_file: src/acp_registry.rs
line_hint: 1039
author: 
created: 
title: "mcp_call"
tags: []
signature: "pub fn mcp_call (name : & str , args : & serde_json :: Value , cfg : & mut crate :: AcpConfig) -> AcpToolCall"
---

The `acp.list_agents`, `acp.install_agent` and `acp.connect_agent` tools.

# Examples

```rust
use crawrad_agent::AcpConfig;
use crawrad_agent::acp_registry::mcp_call;

let mut cfg = AcpConfig::default();
let call = mcp_call("acp.list_agents", &serde_json::json!({}), &mut cfg);
assert_eq!(call.audit, "applied");
```
