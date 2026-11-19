---
id: editor-tabs-editortabs-len
kind: doc
target_file: src/editor_tabs.rs
line_hint: 73
author: 
created: 
title: "EditorTabs::len"
tags: []
signature: "pub fn len (& self) -> usize"
---

How many file tabs there are — the Design tab not counted.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert_eq!(tabs.len(), 0);
```
