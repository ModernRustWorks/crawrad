---
id: form-file-path-for
kind: doc
target_file: src/form_file.rs
line_hint: 21
author: 
created: 
title: "path_for"
tags: []
signature: "pub fn path_for (current : & Path , ir : & Document) -> PathBuf"
---

Determines where a form document belongs based on its root name.

Where the form `ir` belongs, given that it is now at `current`: the same
folder, named after the form's `Name`.

`current` when nothing should move: the name is already the file's, the
`Name` is not a valid identifier (the generator refuses it anyway), or
another file already holds the new name — moving onto it would overwrite
a form that is not this one. A change of case alone does move: on Windows
the target "exists" because it is this very file.

# Examples

```rust
use std::path::Path;
use crawrad_ir::Document;
use crawrad_project::form_file::path_for;

let doc = Document::new("MainForm", "Form");
let path = path_for(Path::new("forms/Old.crawform"), &doc);
assert_eq!(path.file_name().unwrap(), "MainForm.crawform");
```
