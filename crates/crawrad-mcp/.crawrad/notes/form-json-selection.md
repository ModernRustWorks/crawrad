---
id: form-json-selection
kind: doc
target_file: src/form_json.rs
line_hint: 94
author: 
created: 
title: "selection"
tags: []
signature: "pub fn selection (doc : & crawrad_doc :: Doc) -> Value"
---

`designer.get_selection`: the selected ids and their nodes.

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;
use crawrad_mcp::form_json::selection;

let doc = Doc::new(Document::new("form_main", "Form"));
let sel = selection(&doc);
assert_eq!(sel["count"], 0);
```
