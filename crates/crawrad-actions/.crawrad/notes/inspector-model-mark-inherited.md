---
id: inspector-model-mark-inherited
kind: doc
target_file: src/inspector_model.rs
line_hint: 1
author:
created:
title: "mark_inherited"
tags: []
signature: "pub fn mark_inherited (rows : & mut Vec < Row > , doc : & crawrad_doc :: Doc , selection : & [NodeId])"
---

Marks the `Events` rows of an inherited component with the base's handlers.

It also puts the line that names the base form on top. The rows of an own
component, or of a form that inherits from nothing, stay as they are.

# Examples

```
use crawrad_actions::inspector_model::{mark_inherited, Row};
use crawrad_doc::Doc;
use crawrad_ir::Document;

let doc = Doc::new(Document::new("form_a", "Form"));
let mut rows: Vec<Row> = Vec::new();
mark_inherited(&mut rows, &doc, &["form_a".to_string()]);
assert!(rows.is_empty());
```
