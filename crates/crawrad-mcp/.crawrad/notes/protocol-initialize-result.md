---
id: protocol-initialize-result
kind: doc
target_file: src/protocol.rs
line_hint: 59
author: 
created: 
title: "initialize_result"
tags: []
signature: "pub fn initialize_result () -> Value"
---

Builds the result payload for the MCP initialize request.

The `result` of `initialize`: the protocol version, the tools capability
(a fixed list) and who answers.

# Examples

```rust
use crawrad_mcp::protocol::initialize_result;

let init = initialize_result();
assert_eq!(init["serverInfo"]["name"], "crawrad-ide");
```
