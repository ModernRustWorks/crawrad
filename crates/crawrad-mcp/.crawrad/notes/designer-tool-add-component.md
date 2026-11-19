---
id: designer-tool-add-component
kind: doc
target_file: src/designer_tool.rs
line_hint: 125
author: 
created: 
title: "add_component"
tags: []
signature: "pub fn add_component (d : & mut impl Designer , args : & Value) -> Answer"
---

Adds a component to the visual designer form.

`designer.add_component`: `component.add`, then the name and the
properties asked for, each through its own action.

# Examples

```rust
use crawrad_mcp::designer_tool::{add_component, Designer};

struct MockDesigner;
impl Designer for MockDesigner {
    fn doc(&self) -> Option<&crawrad_doc::Doc> { None }
    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> { None }
    fn run(&mut self, _: &str, _: &serde_json::Value) -> Result<(), String> { Ok(()) }
    fn prop_keys(&self, _: &str) -> Vec<&'static str> { vec![] }
}

let mut d = MockDesigner;
let (res, outcome) = add_component(&mut d, &serde_json::json!({}));
assert_eq!(outcome, "error");
```
