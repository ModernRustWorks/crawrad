---
id: editor-tabs-editortabs
kind: doc
target_file: src/editor_tabs.rs
line_hint: 57
author: 
created: 
title: "EditorTabs"
tags: []
signature: "pub struct EditorTabs"
---

The open documents, and which one is in front.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert!(tabs.is_empty());
```
