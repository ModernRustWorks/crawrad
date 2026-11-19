---
id: project-template-create-project-template-with-options
kind: doc
target_file: src/project_template.rs
line_hint: 161
author: 
created: 
title: "create_project_template_with_options"
tags: []
signature: "pub fn create_project_template_with_options (folder : & std :: path :: Path , name : & str , template_id : & str , git_init : bool , options : ProjectTemplateOptions ,) -> Result < PathBuf , String >"
---

Creates the files of a project from the chosen template with custom options.

# Errors

Returns an error message if the target folder exists and is non-empty, if
the toolchain version is too old, or if writing files fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::project_template::{create_project_template_with_options, ProjectTemplateOptions};

// Avoid filesystem operations during doctests.
let res = create_project_template_with_options(
    Path::new("."),
    "demo",
    "console_app",
    false,
    ProjectTemplateOptions::default(),
);
assert!(res.is_ok());
```
