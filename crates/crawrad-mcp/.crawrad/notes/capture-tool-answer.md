---
id: capture-tool-answer
kind: doc
target_file: src/capture_tool.rs
line_hint: 73
author: 
created: 
title: "answer"
tags: []
signature: "pub fn answer (args : & Value , dpi : f64) -> Answer"
---

Answers a `view.capture` call: plans it, captures and encodes the PNG.

`dpi` is the window's scale (the IDE zoom included), turning the logical
region of the call into the physical pixels the capture reads.

# Examples

```
use crawrad_mcp::capture_tool::answer;
use serde_json::json;

// A test thread owns no window: the capture is refused, never invented.
let (result, _) = answer(&json!({}), 1.0);
assert!(result.is_error);
```
