---
id: inspector-model-editors-shortcut-shortcuteditor
kind: doc
target_file: src/inspector_model/editors/shortcut.rs
line_hint: 14
author: 
created: 
title: "ShortcutEditor"
tags: []
signature: "pub struct ShortcutEditor"
---

A keyboard shortcut editor field.

# Examples

```rust
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_actions::inspector_model::editors::shortcut::ShortcutEditor;
use crawrad_ir::{Key, Shortcut};

let editor = ShortcutEditor::new(Shortcut::ctrl(Key::S));
assert_eq!(editor.template(), "RowText");
```
