---
id: items-editor-set-selected-expanded
kind: doc
target_file: src/items_editor.rs
line_hint: 323
author: 
created: 
title: "set_selected_expanded"
tags: []
signature: "pub fn set_selected_expanded (state : & mut ItemsEditorState , expanded : bool)"
---

Sets the expanded state of the selected item if it has children.

# Examples

```rust
use crawrad_actions::items_editor::{set_selected_expanded, ItemsEditorState};
use crawrad_ir::Item;

let mut state = ItemsEditorState {
    node_id: "tv".into(),
    node_class: "TreeView".into(),
    node_name: "tree_view".into(),
    items: vec![Item::with_children("Root", vec![Item::new("Child")])],
    selected: Some(vec![0]),
    is_tree: true,
    has_images: false,
};
set_selected_expanded(&mut state, true);
assert!(state.items[0].expanded);
```
