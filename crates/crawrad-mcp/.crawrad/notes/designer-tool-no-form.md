---
id: designer-tool-no-form
kind: doc
target_file: src/designer_tool.rs
line_hint: 26
author: 
created: 
title: "no_form"
tags: []
signature: "pub fn no_form () -> Answer"
---

The answer when no form is open.

# Examples

```rust
use crawrad_mcp::designer_tool::no_form;

let (result, outcome) = no_form();
assert_eq!(outcome, "error");
```
