---
id: protocol-toolcallresult-success-image
kind: doc
target_file: src/protocol.rs
line_hint: 127
author: 
created: 
title: "ToolCallResult::success_image"
tags: []
signature: "pub fn success_image (png_base64 : impl Into < String > , data : Value , text : impl Into < String > ,) -> Self"
---

A successful result carrying a PNG image, data and a line of text.

# Examples

```
use crawrad_mcp::ToolCallResult;
use serde_json::json;

let res = ToolCallResult::success_image("iVBORw0KGgo=", json!({"width": 1, "height": 1}), "Captured 1x1.");
assert!(!res.is_error);
assert_eq!(res.content.len(), 2);
```
