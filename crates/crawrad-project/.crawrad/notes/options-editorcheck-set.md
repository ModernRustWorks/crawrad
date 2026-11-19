---
id: options-editorcheck-set
kind: doc
target_file: src/options.rs
line_hint: 339
author: 
created: 
title: "EditorCheck::set"
tags: []
signature: "pub fn set (self , p : & mut crate :: workspace :: Preferences , on : bool)"
---

Stores the box.

# Examples

```rust
use crawrad_project::options::EditorCheck;
use crawrad_project::workspace::Preferences;

let mut p = Preferences::default();
EditorCheck::LineNumbers.set(&mut p, false);
assert!(!p.editor_line_numbers);
```
