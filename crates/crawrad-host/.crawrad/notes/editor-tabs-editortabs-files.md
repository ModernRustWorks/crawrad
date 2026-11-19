---
id: editor-tabs-editortabs-files
kind: doc
target_file: src/editor_tabs.rs
line_hint: 68
author: 
created: 
title: "EditorTabs::files"
tags: []
signature: "pub fn files (& self) -> & [PathBuf]"
---

The files, in the order the strip draws them.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert!(tabs.files().is_empty());
```
