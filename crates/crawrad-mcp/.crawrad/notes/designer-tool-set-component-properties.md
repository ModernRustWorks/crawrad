---
id: designer-tool-set-component-properties
kind: doc
target_file: src/designer_tool.rs
line_hint: 208
author: 
created: 
title: "set_component_properties"
tags: []
signature: "pub fn set_component_properties (d : & mut impl Designer , args : & Value) -> Answer"
---

`designer.set_component_properties`: each key answers for itself (#599).

# Examples

```rust
use crawrad_mcp::designer_tool::{set_component_properties, Designer};

struct MockDesigner;
impl Designer for MockDesigner {
    fn doc(&self) -> Option<&crawrad_doc::Doc> { None }
    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> { None }
    fn run(&mut self, _: &str, _: &serde_json::Value) -> Result<(), String> { Ok(()) }
    fn prop_keys(&self, _: &str) -> Vec<&'static str> { vec![] }
}

let mut d = MockDesigner;
let (res, outcome) = set_component_properties(&mut d, &serde_json::json!({}));
assert_eq!(outcome, "error");
```
