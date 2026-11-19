---
id: capture-tool-done
kind: doc
target_file: src/capture_tool.rs
line_hint: 97
author: 
created: 
title: "done"
tags: []
signature: "pub fn done (ask : & CaptureAsk , capture : Option < crawrad_os :: Capture >) -> Answer"
---

Finalizes a window capture operation into an MCP tool call answer.

# Examples

```
use crawrad_mcp::capture_tool::{CaptureAsk, done};

let ask = CaptureAsk { region: None, path: None };
let (result, outcome) = done(&ask, None);
assert_eq!(outcome, "error");
assert!(result.is_error);
```
