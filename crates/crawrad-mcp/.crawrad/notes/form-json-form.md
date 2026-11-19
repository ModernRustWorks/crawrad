---
id: form-json-form
kind: doc
target_file: src/form_json.rs
line_hint: 11
author: 
created: 
title: "form"
tags: []
signature: "pub fn form (doc : & crawrad_doc :: Doc) -> (Value , usize)"
---

The form's revision, selection, root and nodes, and how many child nodes.

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;
use crawrad_mcp::form_json::form;

let doc = Doc::new(Document::new("form_main", "Form"));
let (json, count) = form(&doc);
assert_eq!(count, 0);
assert_eq!(json["root"]["id"], "form_main");
```
