---
id: dependencies-documento-tem-bindings
kind: doc
target_file: src/dependencies.rs
line_hint: 115
author: 
created: 
title: "documento_tem_bindings"
tags: []
signature: "pub fn documento_tem_bindings (doc : & crawrad_ir :: Document) -> bool"
---

Identifies whether a `.crawform` document has data binding definitions.

# Examples

```rust
use crawrad_codegen::documento_tem_bindings;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert!(!documento_tem_bindings(&doc));
```
