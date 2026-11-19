---
id: doc-preview
kind: doc
target_file: src/lib.rs
line_hint: 320
author: 
created: 
title: "Doc::preview"
tags: []
signature: "pub fn preview (& self , tx : & Transaction) -> Result < Preview , DocError >"
---

Simulates applying a transaction without modifying the document.

**Changes nothing** and answers what the transaction would do: the nodes touched and
the resulting document. It is the `preview: true` that every mutating command of
the RTA will expose to agents.

# Examples

```rust
use crawrad_doc::{Command, Doc, Transaction};
use crawrad_ir::{Document, Value};

let doc = Doc::new(Document::new("form", "Form"));
let tx = Transaction::user("set").with(Command::SetProp {
    node: "form".into(),
    key: "title".into(),
    value: Some(Value::Str("Hello".into())),
});
let preview = doc.preview(&tx).unwrap();
assert_eq!(preview.result.root, "form");
```

# Errors

Returns [`DocError`](crate::DocError) if simulated application fails validation.
