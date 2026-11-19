---
id: project-template-create-project-template
kind: doc
target_file: src/project_template.rs
line_hint: 140
author: 
created: 
title: "create_project_template"
tags: []
signature: "pub fn create_project_template (folder : & std :: path :: Path , name : & str , template_id : & str , git_init : bool , framework : crawrad_codegen :: AlvoGerador ,) -> Result < PathBuf , String >"
---

Creates the files of a project from the chosen template.

Supports all 8 templates:
- `gui_app` (Makepad Desktop)
- `console_app` (CLI tool)
- `library_crate` (Reusable library with crawrad.toml)
- `wasm_app` (Makepad Web with index.html)
- `mobile_app` (Makepad Mobile 390x844 with res/icon.png)
- `multiwindow_app` (Makepad Desktop with multiple windows and forms/)
- `package_plugin` (IDE extension with plugin.toml)
- `custom_template` (Configurable scaffold)

# Errors

Returns an error message if the target folder exists and is non-empty, if
the toolchain version is too old, or if writing files fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_codegen::AlvoGerador;
use crawrad_project::project_template::create_project_template;

// Avoid filesystem operations during doctests.
let res = create_project_template(Path::new("."), "demo", "console_app", false, AlvoGerador::Makepad);
assert!(res.is_ok());
```
