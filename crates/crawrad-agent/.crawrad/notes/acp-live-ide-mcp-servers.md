---
id: acp-live-ide-mcp-servers
kind: doc
target_file: src/acp_live.rs
line_hint: 49
author: 
created: 
title: "ide_mcp_servers"
tags: []
signature: "pub fn ide_mcp_servers () -> Vec < Value >"
---

The IDE's own MCP server, handed to every ACP session.

# Examples

```rust
use crawrad_agent::acp_live::ide_mcp_servers;

let servers = ide_mcp_servers();
assert_eq!(servers.len(), 1);
```
