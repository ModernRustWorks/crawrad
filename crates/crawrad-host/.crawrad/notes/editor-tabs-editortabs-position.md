---
id: editor-tabs-editortabs-position
kind: doc
target_file: src/editor_tabs.rs
line_hint: 95
author: 
created: 
title: "EditorTabs::position"
tags: []
signature: "pub fn position (& self , path : & Path) -> Option < usize >"
---

Returns the index of this file if already open.

Is this file already open?

# Examples

```rust
use std::path::Path;
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert_eq!(tabs.position(Path::new("src/main.rs")), None);
```
