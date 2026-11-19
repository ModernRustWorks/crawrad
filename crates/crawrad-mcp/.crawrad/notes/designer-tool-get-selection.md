---
id: designer-tool-get-selection
kind: doc
target_file: src/designer_tool.rs
line_hint: 100
author: 
created: 
title: "get_selection"
tags: []
signature: "pub fn get_selection (doc : Option < & crawrad_doc :: Doc >) -> Answer"
---

`designer.get_selection`: the components selected in the designer.

# Examples

```rust
use crawrad_mcp::designer_tool::get_selection;

let (res, outcome) = get_selection(None);
assert_eq!(outcome, "error");
```
