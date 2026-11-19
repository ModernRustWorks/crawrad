---
id: editor-tabs-editortabs-show-design
kind: doc
target_file: src/editor_tabs.rs
line_hint: 133
author: 
created: 
title: "EditorTabs::show_design"
tags: []
signature: "pub fn show_design (& mut self)"
---

Bring the canvas to the front, leaving every file open.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let mut tabs = EditorTabs::default();
tabs.open("src/main.rs");
tabs.show_design();
assert_eq!(tabs.active_index(), None);
```
