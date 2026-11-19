---
id: client-ide-mcp-server
kind: doc
target_file: src/client.rs
line_hint: 119
author: 
created: 
title: "ide_mcp_server"
tags: []
signature: "pub fn ide_mcp_server (bridge : & Path , instance : & str) -> serde_json :: Value"
---

Pure helper building an `McpServerStdio` JSON entry for the CrawRAD IDE bridge.

# Examples

```rust
use crawrad_agent::ide_mcp_server;
use std::path::Path;

let server = ide_mcp_server(Path::new("bridge"), "inst1");
assert_eq!(server["name"], "crawrad-ide");
```
