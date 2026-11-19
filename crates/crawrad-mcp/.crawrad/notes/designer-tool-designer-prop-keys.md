---
id: designer-tool-designer-prop-keys
kind: doc
target_file: src/designer_tool.rs
line_hint: 22
author: 
created: 
title: "Designer::prop_keys"
tags: []
signature: "fn prop_keys (& self , class : & str) -> Vec < & 'static str >"
---

The property keys the class declares, to refuse the unknown ones.

# Examples

```rust
use crawrad_mcp::designer_tool::Designer;

struct MockDesigner;
impl Designer for MockDesigner {
    fn doc(&self) -> Option<&crawrad_doc::Doc> { None }
    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> { None }
    fn run(&mut self, _: &str, _: &serde_json::Value) -> Result<(), String> { Ok(()) }
    fn prop_keys(&self, _: &str) -> Vec<&'static str> { vec!["text"] }
}

let d = MockDesigner;
assert_eq!(d.prop_keys("Button"), vec!["text"]);
```
