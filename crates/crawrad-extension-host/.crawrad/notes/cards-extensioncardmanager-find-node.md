---
id: cards-extensioncardmanager-find-node
kind: doc
target_file: src/cards.rs
line_hint: 133
author: 
created: 
title: "ExtensionCardManager::find_node"
tags: []
signature: "pub fn find_node < 'a > (root : & 'a UiNode , element_id : & str) -> Option < & 'a UiNode >"
---

Find an immutable reference to an element by ID within a UI node tree.

# Examples

```rust
use crawrad_extension_api::ui::UiNode;
use crawrad_extension_host::ExtensionCardManager;

let root = UiNode::label("lbl1", "Hi");
assert!(ExtensionCardManager::find_node(&root, "lbl1").is_some());
```
