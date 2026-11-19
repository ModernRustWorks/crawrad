---
id: editor-tabs-editortabs-close
kind: doc
target_file: src/editor_tabs.rs
line_hint: 138
author: 
created: 
title: "EditorTabs::close"
tags: []
signature: "pub fn close (& mut self , i : usize)"
---

Closes a tab and hands the front to a neighbour.

**Close a tab, and hand the front to a neighbour.**

To the RIGHT first, then to the left, then the canvas. It is what VS
Code does, and the reason is that closing several in a row should walk
forward through the strip instead of bouncing back to the start.

Closing a tab that is not the active one never moves the front — but it
can move the active INDEX, because the ones after it shift down. Not
correcting that is the classic off-by-one that shows the wrong file.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let mut tabs = EditorTabs::default();
tabs.open("src/main.rs");
tabs.close(0);
assert!(tabs.is_empty());
```
