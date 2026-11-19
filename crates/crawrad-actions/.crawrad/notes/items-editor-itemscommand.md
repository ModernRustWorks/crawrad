---
id: items-editor-itemscommand
kind: doc
target_file: src/items_editor.rs
line_hint: 255
author: 
created: 
title: "ItemsCommand"
tags: []
signature: "pub enum ItemsCommand"
---

The five collection editing commands triggered by the Items Editor buttons.

# Examples

```rust
use crawrad_actions::items_editor::ItemsCommand;

let cmd = ItemsCommand::NewItem;
assert_eq!(cmd, ItemsCommand::NewItem);
```
