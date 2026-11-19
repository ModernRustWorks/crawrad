---
id: project-template-add-project
kind: doc
target_file: src/project_template.rs
line_hint: 447
author: 
created: 
title: "add_project"
tags: []
signature: "pub fn add_project (root : & std :: path :: Path , name : & str , template_id : & str , framework : crawrad_codegen :: AlvoGerador ,) -> Result < PathBuf , String >"
---

Adds a new project from `template_id` into an existing workspace.

Add Project (#471): a new project `name` from `template_id`, created inside
the workspace at `root` and listed in its `members`. Returns its folder.

# Errors

Returns an error message if the folder already exists, the toolchain is too old,
or writing files fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_codegen::AlvoGerador;
use crawrad_project::project_template::add_project;

// Avoid filesystem operations during doctests.
let res = add_project(Path::new("."), "demo", "console_app", AlvoGerador::Makepad);
assert!(res.is_ok());
```
