---
id: client-bridge-next-to-this-exe
kind: doc
target_file: src/client.rs
line_hint: 106
author: 
created: 
title: "bridge_next_to_this_exe"
tags: []
signature: "pub fn bridge_next_to_this_exe () -> std :: path :: PathBuf"
---

Returns the path to the IDE's MCP bridge executable beside the running binary.

The IDE's MCP bridge, `crawrad-mcp-bridge(.exe)` beside the running executable:
the protocol wants an absolute command, never one left to the agent's PATH.

# Examples

```rust
use crawrad_agent::client::bridge_next_to_this_exe;

let path = bridge_next_to_this_exe();
assert!(path.to_string_lossy().contains("crawrad-mcp-bridge"));
```
