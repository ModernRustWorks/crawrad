---
id: items-editor-selected-expanded
kind: doc
target_file: src/items_editor.rs
line_hint: 318
author: 
created: 
title: "selected_expanded"
tags: []
signature: "pub fn selected_expanded (state : & ItemsEditorState) -> Option < bool >"
---

The expanded state of the selected item; `None` for a leaf or no selection.

# Examples

```rust
use crawrad_actions::items_editor::{selected_expanded, ItemsEditorState};

let state = ItemsEditorState {
    node_id: "tv".into(),
    node_class: "TreeView".into(),
    node_name: "tree_view".into(),
    items: Vec::new(),
    selected: None,
    is_tree: true,
    has_images: false,
};
assert_eq!(selected_expanded(&state), None);
```
