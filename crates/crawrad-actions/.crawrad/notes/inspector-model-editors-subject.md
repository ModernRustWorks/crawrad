---
id: inspector-model-editors-subject
kind: doc
target_file: src/inspector_model/editors.rs
line_hint: 100
author: 
created: 
title: "Subject"
tags: []
signature: "pub struct Subject < 'a >"
---

What a maker gets to look at before it decides.

The whole document is in here and that is not generosity: the options of a
`Reference` are the components of the open form, so at least one editor
cannot be built from the descriptor and the value alone.

# Examples

```rust
use crawrad_actions::inspector_model::editors::Subject;
use crawrad_ir::{Document, Value};
use crawrad_registry::{Category, PropDesc, Registry};

let desc = PropDesc::new("name", "Name", Value::Str(String::new()), Category::Identity);
let reg = Registry::builtin();
let val = Value::Str("form1".to_string());
let node_id = "form1".to_string();
let doc = Document::new("form1", "Form");
let subject = Subject {
    desc: &desc,
    registry: &reg,
    value: &val,
    node: &node_id,
    doc: &doc,
    open: false,
};
assert_eq!(subject.open, false);
```
