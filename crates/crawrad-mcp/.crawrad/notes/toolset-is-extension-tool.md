---
id: toolset-is-extension-tool
kind: doc
target_file: src/toolset.rs
line_hint: 56
author: 
created: 
title: "is_extension_tool"
tags: []
signature: "pub fn is_extension_tool (name : & str) -> bool"
---

Checks whether a tool name matches the extension tool pattern.

Whether `name` is an extension's tool: `<extension id>.tool.<name>`, the
name the capability registry and the extension host give it.

# Examples

```rust
use crawrad_mcp::toolset::is_extension_tool;

assert!(is_extension_tool("ext_sample.tool.run"));
assert!(!is_extension_tool("editor.save"));
```
