---
id: designer-tool-designer-doc-mut
kind: doc
target_file: src/designer_tool.rs
line_hint: 16
author: 
created: 
title: "Designer::doc_mut"
tags: []
signature: "fn doc_mut (& mut self) -> Option < & mut crawrad_doc :: Doc >"
---

The same form, to select a component before an action.

# Examples

```rust
use crawrad_mcp::designer_tool::Designer;

struct MockDesigner;
impl Designer for MockDesigner {
    fn doc(&self) -> Option<&crawrad_doc::Doc> { None }
    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> { None }
    fn run(&mut self, _: &str, _: &serde_json::Value) -> Result<(), String> { Ok(()) }
    fn prop_keys(&self, _: &str) -> Vec<&'static str> { vec![] }
}

let mut d = MockDesigner;
assert!(d.doc_mut().is_none());
```
