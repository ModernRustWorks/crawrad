---
id: project-template-create-or-add
kind: doc
target_file: src/project_template.rs
line_hint: 432
author: 
created: 
title: "create_or_add"
tags: []
signature: "pub fn create_or_add (add_to : Option < & std :: path :: Path > , folder : & std :: path :: Path , name : & str , template_id : & str , git_init : bool , framework : crawrad_codegen :: AlvoGerador ,) -> Result < PathBuf , String >"
---

Creates a project in a new workspace or adds it to an existing workspace.

The dialog's Create button (#471): into the workspace `add_to` (Add Project),
or a new workspace in `folder` (New Project). Returns the project's folder.

# Errors

Returns an error message if creating the project or workspace fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_codegen::AlvoGerador;
use crawrad_project::project_template::create_or_add;

// Avoid filesystem operations during doctests.
let res = create_or_add(None, Path::new("."), "demo", "console_app", false, AlvoGerador::Makepad);
assert!(res.is_ok());
```
