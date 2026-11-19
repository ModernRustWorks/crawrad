---
id: editor-tabs-editortabs-activate
kind: doc
target_file: src/editor_tabs.rs
line_hint: 114
author: 
created: 
title: "EditorTabs::activate"
tags: []
signature: "pub fn activate (& mut self , i : usize)"
---

Brings a tab to the front by index.

Bring a tab to the front. Out of range does nothing.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let mut tabs = EditorTabs::default();
tabs.open("src/main.rs");
tabs.activate(0);
assert_eq!(tabs.active_index(), Some(0));
```
