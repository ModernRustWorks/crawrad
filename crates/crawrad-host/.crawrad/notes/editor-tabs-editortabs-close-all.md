---
id: editor-tabs-editortabs-close-all
kind: doc
target_file: src/editor_tabs.rs
line_hint: 165
author: 
created: 
title: "EditorTabs::close_all"
tags: []
signature: "pub fn close_all (& mut self)"
---

Close every file tab, leaving the canvas.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let mut tabs = EditorTabs::default();
tabs.open("src/main.rs");
tabs.close_all();
assert!(tabs.is_empty());
```
