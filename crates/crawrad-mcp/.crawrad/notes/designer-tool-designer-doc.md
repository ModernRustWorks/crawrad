---
id: designer-tool-designer-doc
kind: doc
target_file: src/designer_tool.rs
line_hint: 13
author: 
created: 
title: "Designer::doc"
tags: []
signature: "fn doc (& self) -> Option < & crawrad_doc :: Doc >"
---

The form open in the designer, if any.

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

let d = MockDesigner;
assert!(d.doc().is_none());
```
