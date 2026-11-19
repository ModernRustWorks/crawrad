---
id: form-file-is-form
kind: doc
target_file: src/form_file.rs
line_hint: 15
author: 
created: 
title: "is_form"
tags: []
signature: "pub fn is_form (path : & Path) -> bool"
---

Whether `path` is a form document (`.crawform`).

# Examples

```rust
use std::path::Path;
use crawrad_project::form_file::is_form;

assert!(is_form(Path::new("forms/Main.crawform")));
assert!(!is_form(Path::new("src/main.rs")));
```
