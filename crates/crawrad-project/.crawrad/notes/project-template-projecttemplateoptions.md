---
id: project-template-projecttemplateoptions
kind: doc
target_file: src/project_template.rs
line_hint: 132
author: 
created: 
title: "ProjectTemplateOptions"
tags: []
signature: "pub struct ProjectTemplateOptions"
---

Options for creating a project from a template.

# Examples

```rust
use crawrad_project::project_template::ProjectTemplateOptions;

let opts = ProjectTemplateOptions::default();
assert!(!opts.use_binding);
```
