---
id: active-project-window-title
kind: doc
target_file: src/active_project.rs
line_hint: 62
author: 
created: 
title: "window_title"
tags: []
signature: "pub fn window_title (project : Option < & str > , form_file : Option < & str >) -> String"
---

Formats the window title combining the active project and form file name.

# Examples

```rust
use crawrad_project::active_project::*;
assert_eq!(window_title(Some("app_a"), Some("form_main.crawform")), "CrawRAD — app_a · form_main.crawform");
```
