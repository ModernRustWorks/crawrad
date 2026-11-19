---
id: hostmodel-with-doc
kind: doc
target_file: src/lib.rs
line_hint: 141
author: 
created: 
title: "HostModel::with_doc"
tags: []
signature: "pub fn with_doc (doc : Doc) -> Self"
---

Creates a `HostModel` initialized with a document.

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;
use crawrad_host::HostModel;

let doc = Doc::new(Document::new("f", "Form"));
let host = HostModel::with_doc(doc);
assert!(host.has_doc());
```
