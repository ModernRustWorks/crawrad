---
id: doc-apply-one
kind: doc
target_file: src/lib.rs
line_hint: 288
author: 
created: 
title: "Doc::apply_one"
tags: []
signature: "pub fn apply_one (& mut self , label : impl Into < String > , c : Command) -> Result < Applied , DocError >"
---

Convenience: a transaction of a single command.

# Examples

```rust
use crawrad_doc::{Command, Doc};
use crawrad_ir::{Document, Value};

let mut doc = Doc::new(Document::new("form", "Form"));
let applied = doc.apply_one("set", Command::SetProp {
    node: "form".into(),
    key: "title".into(),
    value: Some(Value::Str("Hello".into())),
}).unwrap();
assert_eq!(applied.revision, 1);
```

# Errors

Returns [`DocError`](crate::DocError) if the command fails validation.
