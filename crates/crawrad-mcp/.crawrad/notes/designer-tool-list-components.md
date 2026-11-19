---
id: designer-tool-list-components
kind: doc
target_file: src/designer_tool.rs
line_hint: 88
author: 
created: 
title: "list_components"
tags: []
signature: "pub fn list_components (doc : Option < & crawrad_doc :: Doc >) -> Answer"
---

`designer.list_components`: every component of the open form.

# Examples

```rust
use crawrad_mcp::designer_tool::list_components;

let (res, outcome) = list_components(None);
assert_eq!(outcome, "error");
```
