---
id: cards-extensioncardmanager-find-node-mut
kind: doc
target_file: src/cards.rs
line_hint: 148
author: 
created: 
title: "ExtensionCardManager::find_node_mut"
tags: []
signature: "pub fn find_node_mut < 'a > (root : & 'a mut UiNode , element_id : & str) -> Option < & 'a mut UiNode >"
---

Find a mutable reference to an element by ID within a UI node tree.

# Examples

```rust
use crawrad_extension_api::ui::UiNode;
use crawrad_extension_host::ExtensionCardManager;

let mut root = UiNode::label("lbl1", "Hi");
assert!(ExtensionCardManager::find_node_mut(&mut root, "lbl1").is_some());
```
