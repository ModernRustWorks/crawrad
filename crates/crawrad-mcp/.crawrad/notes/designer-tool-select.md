---
id: designer-tool-select
kind: doc
target_file: src/designer_tool.rs
line_hint: 53
author: 
created: 
title: "select"
tags: []
signature: "pub fn select (doc : Option < & crawrad_doc :: Doc > , id : & str) -> Result < String , Answer >"
---

Verifies a component exists by ID or name in the active form.

`designer.select`: verifies a component exists by id or name in the active form.
Returns Ok(resolved_id) for the IDE to select, or Err(Answer).

# Examples

```rust
use crawrad_mcp::designer_tool::select;

let doc = crawrad_doc::Doc::new(crawrad_ir::Document::new("MainForm", "Form"));
assert_eq!(select(Some(&doc), "MainForm").unwrap(), "MainForm");
```

# Errors

Returns an error answer if no form is open or the component is not found.
