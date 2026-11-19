---
id: options-editorcheck-widgets
kind: doc
target_file: src/options.rs
line_hint: 315
author: 
created: 
title: "EditorCheck::widgets"
tags: []
signature: "pub fn widgets (self) -> & 'static [& 'static str]"
---

The widgets that show it, by their DSL names.

# Examples

```rust
use crawrad_project::options::EditorCheck;

assert!(EditorCheck::LineNumbers.widgets().contains(&"opt_editor_line_numbers"));
```
