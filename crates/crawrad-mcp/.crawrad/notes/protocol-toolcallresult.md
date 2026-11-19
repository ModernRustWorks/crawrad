---
id: protocol-toolcallresult
kind: doc
target_file: src/protocol.rs
line_hint: 94
author: 
created: 
title: "ToolCallResult"
tags: []
signature: "pub struct ToolCallResult"
---

Result payload for `tools/call`.

# Examples

```rust
use crawrad_mcp::protocol::ToolCallResult;

let res = ToolCallResult::success_text("Success");
assert!(!res.is_error);
assert_eq!(res.content.len(), 1);
```
