---
id: designer-tool-wire-event
kind: doc
target_file: src/designer_tool.rs
line_hint: 289
author: 
created: 
title: "wire_event"
tags: []
signature: "pub fn wire_event (d : & mut impl Designer , args : & Value) -> Result < Wired , Answer >"
---

`designer.wire_event`, first half: `event.bind` on the component.

# Examples

```rust
use crawrad_mcp::designer_tool::{wire_event, Designer};

struct MockDesigner;
impl Designer for MockDesigner {
    fn doc(&self) -> Option<&crawrad_doc::Doc> { None }
    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> { None }
    fn run(&mut self, _: &str, _: &serde_json::Value) -> Result<(), String> { Ok(()) }
    fn prop_keys(&self, _: &str) -> Vec<&'static str> { vec![] }
}

let mut d = MockDesigner;
assert!(wire_event(&mut d, &serde_json::json!({})).is_err());
```

# Errors

Returns an error answer if parameters are missing or binding fails.
