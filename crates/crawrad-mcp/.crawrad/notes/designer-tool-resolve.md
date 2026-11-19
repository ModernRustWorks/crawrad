---
id: designer-tool-resolve
kind: doc
target_file: src/designer_tool.rs
line_hint: 45
author: 
created: 
title: "resolve"
tags: []
signature: "pub fn resolve (doc : & crawrad_doc :: Doc , id : & str) -> Option < String >"
---

A component by id, or else by name: the id of the one found.

# Examples

```rust
use crawrad_mcp::designer_tool::resolve;

let doc = crawrad_doc::Doc::new(crawrad_ir::Document::new("MainForm", "Form"));
assert_eq!(resolve(&doc, "MainForm"), Some("MainForm".to_string()));
```
