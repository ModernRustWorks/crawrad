---
id: editor-tabs-editortabs-active-file
kind: doc
target_file: src/editor_tabs.rs
line_hint: 88
author: 
created: 
title: "EditorTabs::active_file"
tags: []
signature: "pub fn active_file (& self) -> Option < & Path >"
---

The file in front, if any.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert_eq!(tabs.active_file(), None);
```
