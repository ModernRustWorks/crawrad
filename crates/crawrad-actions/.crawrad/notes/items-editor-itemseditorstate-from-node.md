---
id: items-editor-itemseditorstate-from-node
kind: doc
target_file: src/items_editor.rs
line_hint: 90
author: 
created: 
title: "ItemsEditorState::from_node"
tags: []
signature: "pub fn from_node (node_id : & str , node : & crawrad_ir :: Node , registry : & crawrad_registry :: Registry ,) -> Self"
---

Builds the items editor state from a document node.

The registry enters because it is what knows whether this class's
collection has children and icons. Without a descriptor — a class unknown to the
catalogue — the collection is born FLAT: promising `New SubItem` without
knowing if anyone honors it is the bug, and the safe default is the one
that does not promise.

# Examples

```rust
use crawrad_actions::items_editor::ItemsEditorState;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let node = Node::new("tree1", "TreeView");
let registry = Registry::builtin();
let state = ItemsEditorState::from_node("tree1", &node, &registry);
assert_eq!(state.node_id, "tree1");
```
