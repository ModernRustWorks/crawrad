---
id: editor-tabs-editortabs-is-empty
kind: doc
target_file: src/editor_tabs.rs
line_hint: 78
author: 
created: 
title: "EditorTabs::is_empty"
tags: []
signature: "pub fn is_empty (& self) -> bool"
---

Returns whether the strip contains only the Design tab.

Is the strip nothing but the Design tab?

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert!(tabs.is_empty());
```
