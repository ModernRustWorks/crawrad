---
id: editor-tabs-editortabs-caret-of
kind: doc
target_file: src/editor_tabs.rs
line_hint: 128
author: 
created: 
title: "EditorTabs::caret_of"
tags: []
signature: "pub fn caret_of (& self , file : & Path) -> Option < (usize , usize) >"
---

The caret `file` had when it last left the front.

# Examples

```rust
use std::path::Path;
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert_eq!(tabs.caret_of(Path::new("src/main.rs")), None);
```
