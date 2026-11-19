---
id: items-editor-itemedit
kind: doc
target_file: src/items_editor.rs
line_hint: 330
author: 
created: 
title: "ItemEdit"
tags: []
signature: "pub enum ItemEdit"
---

An edit typed or clicked in the Item Properties group.

# Examples

```rust
use crawrad_actions::items_editor::ItemEdit;

let edit = ItemEdit::Text("NewName".into());
assert_eq!(edit, ItemEdit::Text("NewName".into()));
```
