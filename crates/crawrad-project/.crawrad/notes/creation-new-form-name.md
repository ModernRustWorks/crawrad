---
id: creation-new-form-name
kind: doc
target_file: src/creation.rs
title: "new_form_name"
tags: []
signature: "pub fn new_form_name (folder: & Path) -> (String, String)"
---

The stem and caption of the next new form in a folder.

A folder with no `.crawform` gets `form_main`, the name the project
templates use; after that `form_1`, `form_2`... the first one free.

# Examples

```rust
let dir = std::env::temp_dir().join("crawrad-doc-new-form-name");
let _ = std::fs::remove_dir_all(&dir);
std::fs::create_dir_all(&dir).unwrap();
let (stem, caption) = crawrad_project::creation::new_form_name(&dir);
assert_eq!((stem.as_str(), caption.as_str()), ("form_main", "Form1"));
let _ = std::fs::remove_dir_all(&dir);
```
