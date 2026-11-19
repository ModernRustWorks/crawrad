---
id: designer-tool-get-component-properties
kind: doc
target_file: src/designer_tool.rs
line_hint: 110
author: 
created: 
title: "get_component_properties"
tags: []
signature: "pub fn get_component_properties (doc : Option < & crawrad_doc :: Doc > , args : & Value) -> Answer"
---

`designer.get_component_properties`: one component and its properties.

# Examples

```rust
use crawrad_mcp::designer_tool::get_component_properties;

let (res, outcome) = get_component_properties(None, &serde_json::json!({}));
assert_eq!(outcome, "error");
```
