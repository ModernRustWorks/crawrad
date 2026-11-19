---
id: project-template-default-project-name
kind: doc
target_file: src/project_template.rs
line_hint: 500
author: 
created: 
title: "DEFAULT_PROJECT_NAME"
tags: []
signature: "pub const DEFAULT_PROJECT_NAME : & str = \"my_app\""
---

The name a new project gets when its field is left empty.

# Examples

```rust
use crawrad_project::project_template::DEFAULT_PROJECT_NAME;

assert_eq!(DEFAULT_PROJECT_NAME, "my_app");
```
