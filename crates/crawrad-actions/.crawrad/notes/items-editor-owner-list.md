---
id: items-editor-owner-list
kind: doc
target_file: src/items_editor.rs
line_hint: 149
author: 
created: 
title: "owner_list"
tags: []
signature: "pub fn owner_list < 'a > (items : & 'a mut Vec < Item > , path : & [usize] ,) -> Option < (& 'a mut Vec < Item > , usize) >"
---

Returns the containing slice and index for an item path.

The list containing the item pointed to by the path, and its position inside
it. It is what insert, delete, and move need.

# Examples

```rust
use crawrad_actions::items_editor::owner_list;
use crawrad_ir::Item;

let mut items = vec![Item::new("A"), Item::new("B")];
let (list, idx) = owner_list(&mut items, &[1]).unwrap();
assert_eq!(idx, 1);
assert_eq!(list[idx].text, "B");
```
