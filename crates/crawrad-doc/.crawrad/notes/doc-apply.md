---
id: doc-apply
kind: doc
target_file: src/lib.rs
line_hint: 208
author: 
created: 
title: "Doc::apply"
tags: []
signature: "pub fn apply (& mut self , tx : Transaction) -> Result < Applied , DocError >"
---

Applies a transaction atomically to the document.

Applies a transaction. Either everything goes in, or nothing goes in: if a command
fails halfway through, the previous ones are undone before returning the error.

# Examples

```rust
use crawrad_doc::{Command, Doc, Transaction};
use crawrad_ir::{Document, Value};

let mut doc = Doc::new(Document::new("form", "Form"));
let tx = Transaction::user("set").with(Command::SetProp {
    node: "form".into(),
    key: "title".into(),
    value: Some(Value::Str("Hello".into())),
});
let applied = doc.apply(tx).unwrap();
assert_eq!(applied.revision, 1);
```

# Errors

Returns [`DocError`](crate::DocError) if the transaction is empty or any command fails validation.
