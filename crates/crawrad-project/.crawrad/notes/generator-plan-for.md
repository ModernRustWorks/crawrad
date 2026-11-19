---
id: generator-plan-for
kind: doc
target_file: src/generator.rs
line_hint: 182
author: 
created: 
title: "plan_for"
tags: []
signature: "pub fn plan_for (ir : & Document , package : & str ,) -> Result < Vec < crawrad_codegen :: Arquivo > , crawrad_codegen :: Erro >"
---

What generating a document would write, without touching the disk.

The single door: whoever wants to know what `Generate` would write comes
through here, with the same generator and the same package name. It is the
pair of [`crawrad_codegen::escrever`] — decide here, write there — and it is
what stops a second generation path from being born with its own idea of
the plan.

# Errors

Returns [`crawrad_codegen::Erro`] if generating the plan fails.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_project::generator::plan_for;

let doc = Document::new("MainForm", "Form");
let plan = plan_for(&doc, "app");
assert!(plan.is_ok());
```
