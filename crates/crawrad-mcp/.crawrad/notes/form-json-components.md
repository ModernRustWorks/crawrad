---
id: form-json-components
kind: doc
target_file: src/form_json.rs
line_hint: 76
author: 
created: 
title: "components"
tags: []
signature: "pub fn components (doc : & crawrad_doc :: Doc) -> Vec < Value >"
---

`designer.list_components`: every node but the root, with its parent.

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;
use crawrad_mcp::form_json::components;

let doc = Doc::new(Document::new("form_main", "Form"));
let list = components(&doc);
assert!(list.is_empty());
```
