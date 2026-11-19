---
id: inspector-model-editors-file-path-filepatheditor
kind: doc
target_file: src/inspector_model/editors/file_path.rs
line_hint: 14
author: 
created: 
title: "FilePathEditor"
tags: []
signature: "pub struct FilePathEditor"
---

A file path editor field.

# Examples

```rust
use crawrad_actions::inspector_model::editors::file_path::FilePathEditor;
use crawrad_actions::inspector_model::editor::PropEditor;

let ed = FilePathEditor::new("src/main.rs");
assert_eq!(ed.template(), "RowText");
```
