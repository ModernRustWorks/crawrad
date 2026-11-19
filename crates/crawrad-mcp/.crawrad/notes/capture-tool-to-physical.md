---
id: capture-tool-to-physical
kind: doc
target_file: src/capture_tool.rs
line_hint: 73
author: 
created: 
title: "to_physical"
tags: []
signature: "pub fn to_physical (region : [f64 ; 4] , dpi : f64) -> [i32 ; 4]"
---

Converts logical window coordinates to physical device pixels.

# Examples

```
use crawrad_mcp::capture_tool::to_physical;

let physical = to_physical([10.0, 20.0, 100.0, 50.0], 1.5);
assert_eq!(physical, [15, 30, 150, 75]);
```
