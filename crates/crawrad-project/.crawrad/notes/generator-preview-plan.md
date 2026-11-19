---
id: generator-preview-plan
kind: doc
target_file: src/generator.rs
line_hint: 165
author: 
created: 
title: "preview_plan"
tags: []
signature: "pub fn preview_plan (member : Option < & Path > , ir : & Document , editing : & dyn Fn (& Path) -> Option < String > ,) -> Result < Vec < crawrad_codegen :: Arquivo > , crawrad_codegen :: Erro >"
---

Generates the Explorer preview's plan for a member's document.

The Explorer preview's plan for the document of `member` (#59): the plan
`Generate` makes, with a file that has unsaved edits judged by its text
on screen (`editing`), because the preview may not write the editor.

# Errors

Returns [`crawrad_codegen::Erro`] if plan generation fails.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_project::generator::preview_plan;

let doc = Document::new("MainForm", "Form");
let plan = preview_plan(None, &doc, &|_| None);
assert!(plan.is_ok());
```
