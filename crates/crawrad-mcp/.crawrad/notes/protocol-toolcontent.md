---
id: protocol-toolcontent
kind: doc
target_file: src/protocol.rs
line_hint: 86
author: 
created: 
title: "ToolContent"
tags: []
signature: "pub enum ToolContent"
---

A tool call result item: text, or a base64 image (`view.capture`).

# Examples

```rust
use crawrad_mcp::protocol::ToolContent;

let item = ToolContent::Text { text: "done".to_string() };
match item {
    ToolContent::Text { text } => assert_eq!(text, "done"),
    ToolContent::Image { .. } => unreachable!(),
}
```
