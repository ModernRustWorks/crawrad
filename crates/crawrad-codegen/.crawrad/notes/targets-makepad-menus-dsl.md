---
id: targets-makepad-menus-dsl
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 120
author: 
created: 
title: "dsl"
tags: []
signature: "pub fn dsl (doc : & Document , emitted : & [& Node]) -> Result < String , Erro >"
---

The DSL of the menus, last in the body so they draw over every control.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::dsl;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let code = dsl(&doc, &[]).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if any menu component identifier cannot be formatted.
