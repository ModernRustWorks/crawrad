---
id: components-tool-bar-tool-bar
kind: doc
target_file: src/components/tool_bar.rs
line_hint: 4
author: 
created: 
title: "tool_bar"
tags: []
signature: "pub fn tool_bar () -> ComponentDesc"
---

A toolbar container organizing command buttons and separators.

`ToolBar`: container toolbar organizing command buttons (SpeedButton, separators).

# Examples

```rust
use crawrad_registry::tool_bar;

let desc = tool_bar();
assert_eq!(desc.class, "ToolBar");
```
