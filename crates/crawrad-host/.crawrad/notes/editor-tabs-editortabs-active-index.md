---
id: editor-tabs-editortabs-active-index
kind: doc
target_file: src/editor_tabs.rs
line_hint: 83
author: 
created: 
title: "EditorTabs::active_index"
tags: []
signature: "pub fn active_index (& self) -> Option < usize >"
---

Which file tab is in front, if it is a file and not the canvas.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert_eq!(tabs.active_index(), None);
```
