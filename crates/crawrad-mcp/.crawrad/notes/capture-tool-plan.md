---
id: capture-tool-plan
kind: doc
target_file: src/capture_tool.rs
line_hint: 21
author: 
created: 
title: "plan"
tags: []
signature: "pub fn plan (name : & str , args : & Value) -> Option < Result < CaptureAsk , Answer > >"
---

Plans a window capture operation from tool name and arguments.

# Errors

Returns an error answer with code `invalid-arguments` when partial region
coordinates are supplied, when dimensions are non-positive, or when the
destination path is not absolute.

# Examples

```
use crawrad_mcp::capture_tool::plan;
use serde_json::json;

assert!(plan("unrelated.tool", &json!({})).is_none());
let planned = plan("view.capture", &json!({})).unwrap().unwrap();
assert!(planned.region.is_none());
```
