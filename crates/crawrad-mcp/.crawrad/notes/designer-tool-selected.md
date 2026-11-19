---
id: designer-tool-selected
kind: doc
target_file: src/designer_tool.rs
line_hint: 67
author: 
created: 
title: "selected"
tags: []
signature: "pub fn selected (doc : & crawrad_doc :: Doc , id : & str) -> Answer"
---

The answer when a component was selected.

# Examples

```rust
use crawrad_mcp::designer_tool::selected;

let doc = crawrad_doc::Doc::new(crawrad_ir::Document::new("MainForm", "Form"));
let (res, outcome) = selected(&doc, "MainForm");
assert_eq!(outcome, "applied");
```
