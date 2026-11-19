---
id: targets-shared-texto-das-pecas
kind: doc
target_file: src/targets/shared.rs
line_hint: 287
author: 
created: 
title: "texto_das_pecas"
tags: []
signature: "pub fn texto_das_pecas (doc : & crawrad_ir :: Document , reg : & crawrad_registry :: Registry) -> String"
---

The text of the pieces that the form needs, one per line.

# Examples

```rust
use crawrad_codegen::targets::shared::texto_das_pecas;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
let text = texto_das_pecas(&doc, &reg);
let _ = text;
```

# Panics

Panics if a required piece does not have an associated definition text.
