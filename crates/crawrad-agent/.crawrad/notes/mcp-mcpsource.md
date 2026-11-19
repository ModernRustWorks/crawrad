---
id: mcp-mcpsource
kind: doc
target_file: src/mcp.rs
line_hint: 16
author: 
created: 
title: "McpSource"
tags: []
signature: "pub enum McpSource"
---

Source badge of an MCP server (e.g. built-in vs add-on).

# Examples

```rust
use crawrad_agent::McpSource;

let src = McpSource::BuiltIn;
assert_eq!(src, McpSource::BuiltIn);
```
