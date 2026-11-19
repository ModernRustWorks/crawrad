---
id: designer-tool-designer
kind: doc
target_file: src/designer_tool.rs
line_hint: 11
author: 
created: 
title: "Designer"
tags: []
signature: "pub trait Designer"
---

Trait for designer inspection and catalogue action execution.

What a designer tool needs from the IDE: the open form, and the catalogue
actions run as the agent — the same ones the user's gestures run.

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
