---
id: protocol-protocol-version
kind: doc
target_file: src/protocol.rs
line_hint: 7
author: 
created: 
title: "PROTOCOL_VERSION"
tags: []
signature: "pub const PROTOCOL_VERSION : & str = \"2024-11-05\""
---

The protocol version published by this MCP server.

# Examples

```rust
use crawrad_mcp::protocol::PROTOCOL_VERSION;

assert_eq!(PROTOCOL_VERSION, "2024-11-05");
```
