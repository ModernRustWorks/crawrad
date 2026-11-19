---
id: capture-tool-captureask
kind: doc
target_file: src/capture_tool.rs
line_hint: 12
author: 
created: 
title: "CaptureAsk"
tags: []
signature: "pub struct CaptureAsk"
---

Specification of a requested window capture operation.

# Examples

```
use crawrad_mcp::capture_tool::CaptureAsk;

let ask = CaptureAsk { region: None, path: None };
assert!(ask.region.is_none());
```
