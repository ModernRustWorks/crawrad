---
id: designer-tool-designer-run
kind: doc
target_file: src/designer_tool.rs
line_hint: 19
author: 
created: 
title: "Designer::run"
tags: []
signature: "fn run (& mut self , name : & str , args : & Value) -> Result < () , String >"
---

Runs the catalogue action `name` with `args`; the error says why not.

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
assert!(d.run("test", &serde_json::json!({})).is_ok());
```

# Errors

Returns an error string describing why the catalogue action failed.
