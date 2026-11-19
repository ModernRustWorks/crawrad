---
id: project-template-structure-preview
kind: doc
target_file: src/project_template.rs
line_hint: 472
author: 
created: 
title: "structure_preview"
tags: []
signature: "pub fn structure_preview (tpl : & TemplateDef , name : & str , add_to : Option < & std :: path :: Path > ,) -> String"
---

Previews the project structure tree for the template dialog.

The dialog's "Project structure" (#471): the template's tree as Create
writes it — inside the new workspace, or inside `add_to` (Add Project).

# Examples

```rust
use crawrad_project::project_template::{TEMPLATES, structure_preview};

let preview = structure_preview(&TEMPLATES[0], "my_app", None);
assert!(preview.contains("my_app"));
```
