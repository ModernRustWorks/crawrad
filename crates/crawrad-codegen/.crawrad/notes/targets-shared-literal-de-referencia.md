---
id: targets-shared-literal-de-referencia
kind: doc
target_file: src/targets/shared.rs
line_hint: 97
author: 
created: 
title: "literal_de_referencia"
tags: []
signature: "pub fn literal_de_referencia (doc : & crawrad_ir :: Document , id : & str) -> String"
---

The Rust literal of a reference — the target's name, or empty.

**A target that disappeared becomes empty, not an invented name.** Deleting the engine and
leaving the connection pointing to it is a state that the document accepts, and the
application must be born saying *"none"* instead of citing a component
that no longer exists.

# Examples

```rust
use crawrad_codegen::targets::shared::literal_de_referencia;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert_eq!(literal_de_referencia(&doc, "missing"), "String::new()");
```
