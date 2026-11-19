---
id: editor-tabs-editortabs-remember-caret
kind: doc
target_file: src/editor_tabs.rs
line_hint: 121
author: 
created: 
title: "EditorTabs::remember_caret"
tags: []
signature: "pub fn remember_caret (& mut self , file : & Path , line : usize , byte : usize)"
---

Remembers where the caret of `file` is (#555), for when it comes back.

# Examples

```rust
use std::path::Path;
use crawrad_host::editor_tabs::EditorTabs;

let mut tabs = EditorTabs::default();
tabs.open("src/main.rs");
tabs.remember_caret(Path::new("src/main.rs"), 10, 5);
assert_eq!(tabs.caret_of(Path::new("src/main.rs")), Some((10, 5)));
```
