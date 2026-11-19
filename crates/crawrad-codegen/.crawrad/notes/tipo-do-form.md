---
id: tipo-do-form
kind: doc
target_file: src/lib.rs
line_hint: 323
author: 
created: 
title: "tipo_do_form"
tags: []
signature: "pub fn tipo_do_form (nome : & str) -> String"
---

The name of the Rust type that a form named `nome` generates.

# Examples

```rust
use crawrad_codegen::tipo_do_form;

assert_eq!(tipo_do_form("form_main"), "FormMain");
```

`form_main` becomes `FormMain`. It lives here, and not in the egui generator, because
whoever migrates a rename needs to know the **old** and the **new** name — and
that is the same rule for any target that emits Rust.
