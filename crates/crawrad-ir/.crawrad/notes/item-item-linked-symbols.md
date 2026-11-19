---
id: item-item-linked-symbols
kind: doc
target_file: src/item.rs
line_hint: 123
author: 
created: 
title: "Item::linked_symbols"
tags: []
signature: "pub fn linked_symbols (doc : & crate :: Document , node : & crate :: Node) -> Vec < String >"
---

Returns the icon symbols of the linked `ImageList`.

The icon symbols of the `ImageList` that `node`'s `images` names
(by id or name); empty when there is none.

# Examples

```rust
use crawrad_ir::{Document, Item, Node};

let doc = Document::new("form", "Form");
let node = Node::new("tv", "TreeView");
assert!(Item::linked_symbols(&doc, &node).is_empty());
```
