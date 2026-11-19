---
id: document-aberto
kind: doc
target_file: src/document.rs
line_hint: 31
author: 
created: 
title: "Aberto"
tags: []
signature: "pub struct Aberto"
---

The form that actually came in, with the path where it came from.

The path is part of the result, and not a detail: it is what the status
bar shows, and it is what `Save` writes.

# Examples

```rust
use std::path::PathBuf;
use crawrad_ir::Document;
use crawrad_host::document::Aberto;

let aberto = Aberto { doc: Document::new("form", "Form"), caminho: PathBuf::from("form.crawform") };
assert_eq!(aberto.caminho, PathBuf::from("form.crawform"));
```
