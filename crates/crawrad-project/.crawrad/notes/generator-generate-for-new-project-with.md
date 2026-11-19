---
id: generator-generate-for-new-project-with
kind: doc
target_file: src/generator.rs
line_hint: 333
author: 
created: 
title: "generate_for_new_project_with"
tags: []
signature: "pub fn generate_for_new_project_with (member : & Path , doc : & Document , target : crawrad_codegen :: AlvoGerador ,) -> Result < () , String >"
---

Generates initial code for a new project using a specific target framework.

[`generate_for_new_project`] with the project's own target — a template
chooses its framework, and the preference in force must not override it.

# Errors

Returns an error sentence if generating or writing the project files fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_codegen::AlvoGerador;
use crawrad_ir::Document;
use crawrad_project::generator::generate_for_new_project_with;

let doc = Document::new("MainForm", "Form");
let res = generate_for_new_project_with(Path::new("."), &doc, AlvoGerador::Makepad);
assert!(res.is_ok());
```
