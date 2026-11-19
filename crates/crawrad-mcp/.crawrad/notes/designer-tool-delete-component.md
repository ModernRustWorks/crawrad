---
id: designer-tool-delete-component
kind: doc
target_file: src/designer_tool.rs
line_hint: 247
author: 
created: 
title: "delete_component"
tags: []
signature: "pub fn delete_component (d : & mut impl Designer , args : & Value) -> Answer"
---

`designer.delete_component`: select it, then `edit.delete`; never the form.

# Examples

```rust
use crawrad_mcp::designer_tool::{delete_component, Designer};

struct MockDesigner;
impl Designer for MockDesigner {
    fn doc(&self) -> Option<&crawrad_doc::Doc> { None }
    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> { None }
    fn run(&mut self, _: &str, _: &serde_json::Value) -> Result<(), String> { Ok(()) }
    fn prop_keys(&self, _: &str) -> Vec<&'static str> { vec![] }
}

let mut d = MockDesigner;
let (res, outcome) = delete_component(&mut d, &serde_json::json!({}));
assert_eq!(outcome, "error");
```
