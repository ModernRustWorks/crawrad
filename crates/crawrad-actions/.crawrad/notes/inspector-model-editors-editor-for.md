---
id: inspector-model-editors-editor-for
kind: doc
target_file: src/inspector_model/editors.rs
line_hint: 166
author: 
created: 
title: "editor_for"
tags: []
signature: "pub fn editor_for (subject : & Subject < '_ >) -> Option < Editor >"
---

The editor of a property, or `None` when nothing here accepts it.

`None` is not a hole in the coverage: it is the value in the document not
matching the kind the catalogue declares — a `.crawform` written by hand, a
catalogue that changed the type of a key already saved — and the caller
turns it into the row that shows the value and says `bad type`.

# Examples

```rust
use crawrad_actions::inspector_model::editors::{editor_for, Subject};
use crawrad_ir::{Document, Value};
use crawrad_registry::{Category, PropDesc, Registry};

let desc = PropDesc::new("name", "Name", Value::Str(String::new()), Category::Identity);
let reg = Registry::builtin();
let val = Value::Str("form".to_string());
let node_id = "form".to_string();
let doc = Document::new("form", "Form");
let subject = Subject {
    desc: &desc,
    registry: &reg,
    value: &val,
    node: &node_id,
    doc: &doc,
    open: false,
};
assert!(editor_for(&subject).is_some());
```
