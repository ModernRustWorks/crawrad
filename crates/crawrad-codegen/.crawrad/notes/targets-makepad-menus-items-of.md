---
id: targets-makepad-menus-items-of
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 34
author: 
created: 
title: "items_of"
tags: []
signature: "pub fn items_of < 'a > (doc : & 'a Document , owner : & Node) -> Vec < & 'a Node >"
---

The `MenuItem` children of `owner`, in their order.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::items_of;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let owner = Node::new("menu", "MainMenu");
let items = items_of(&doc, &owner);
assert!(items.is_empty());
```
