---
id: items-editor-can-create-sub-item
kind: doc
target_file: src/items_editor.rs
line_hint: 122
author: 
created: 
title: "can_create_sub_item"
tags: []
signature: "pub fn can_create_sub_item (state : & ItemsEditorState , has_selection : bool) -> bool"
---

Returns whether creating a sub-item is currently allowed.

Is `New SubItem` enabled?

Two conditions, and both are visible on screen: there is a chosen item to be
the parent, and the collection accepts children. The second used to be
`!state.plana`, and `plana` came from a literal class name.

# Examples

```rust
use crawrad_actions::items_editor::{can_create_sub_item, ItemsEditorState};

let state = ItemsEditorState {
    node_id: "tv".into(),
    node_class: "TreeView".into(),
    node_name: "tree_view".into(),
    items: Vec::new(),
    selected: None,
    is_tree: true,
    has_images: false,
};
assert!(can_create_sub_item(&state, true));
assert!(!can_create_sub_item(&state, false));
```
