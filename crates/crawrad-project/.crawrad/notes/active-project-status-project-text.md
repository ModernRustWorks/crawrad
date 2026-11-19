---
id: active-project-status-project-text
kind: doc
target_file: src/active_project.rs
line_hint: 72
author: 
created: 
title: "status_project_text"
tags: []
signature: "pub fn status_project_text (project : Option < & str >) -> String"
---

Formats the status bar text for the active project cell.

# Examples

```rust
use crawrad_project::active_project::*;
assert_eq!(status_project_text(Some("app_a")), "Project: app_a");
```
