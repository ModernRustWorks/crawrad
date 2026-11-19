---
id: form-file-save
kind: doc
target_file: src/form_file.rs
line_hint: 40
author: 
created: 
title: "save"
tags: []
signature: "pub fn save (ir : & Document , current : & Path) -> Result < PathBuf , String >"
---

Saves the form document to its intended location on disk.

Saves the form where its `Name` puts it ([`path_for`]) and removes the file
it was read from when that is another one. Hands back the path written,
which the caller makes the open document's path.

The file is moved first and written after, so a failure in between leaves
the last saved form under its new name: one copy, never none.

# Errors

Returns an error if renaming the existing file or saving the document fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_ir::Document;
use crawrad_project::form_file::save;

let doc = Document::new("MainForm", "Form");
let res = save(&doc, Path::new("forms/MainForm.crawform"));
assert!(res.is_ok());
```
