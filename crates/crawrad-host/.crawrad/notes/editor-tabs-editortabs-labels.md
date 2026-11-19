---
id: editor-tabs-editortabs-labels
kind: doc
target_file: src/editor_tabs.rs
line_hint: 171
author: 
created: 
title: "EditorTabs::labels"
tags: []
signature: "pub fn labels (& self) -> Vec < Label >"
---

What each tab writes on itself, disambiguated.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let tabs = EditorTabs::default();
assert!(tabs.labels().is_empty());
```
