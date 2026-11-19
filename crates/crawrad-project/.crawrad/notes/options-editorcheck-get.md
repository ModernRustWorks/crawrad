---
id: options-editorcheck-get
kind: doc
target_file: src/options.rs
line_hint: 329
author: 
created: 
title: "EditorCheck::get"
tags: []
signature: "pub fn get (self , p : & crate :: workspace :: Preferences) -> bool"
---

Whether the box is checked.

# Examples

```rust
use crawrad_project::options::EditorCheck;
use crawrad_project::workspace::Preferences;

let p = Preferences::default();
assert_eq!(EditorCheck::LineNumbers.get(&p), p.editor_line_numbers);
```
