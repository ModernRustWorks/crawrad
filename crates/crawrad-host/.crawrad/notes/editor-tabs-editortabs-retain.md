---
id: editor-tabs-editortabs-retain
kind: doc
target_file: src/editor_tabs.rs
line_hint: 156
author: 
created: 
title: "EditorTabs::retain"
tags: []
signature: "pub fn retain (& mut self , keep : impl Fn (& std :: path :: Path) -> bool)"
---

Closes file tabs that do not satisfy the predicate.

Close every file tab `keep` says no to, through [`Self::close`], so the
tab in front moves as a single close moves it.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let mut tabs = EditorTabs::default();
tabs.open("src/main.rs");
tabs.retain(|_| false);
assert!(tabs.is_empty());
```
