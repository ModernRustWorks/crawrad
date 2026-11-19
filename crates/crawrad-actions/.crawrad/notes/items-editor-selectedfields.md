---
id: items-editor-selectedfields
kind: doc
target_file: src/items_editor.rs
line_hint: 287
author: 
created: 
title: "SelectedFields"
tags: []
signature: "pub struct SelectedFields"
---

Fields displayed in the Item Properties group for the selected item.

What the Item Properties group shows for the selected item. `None` means
the field does not apply: no selection, a leaf has nothing to fold, and a
class without `images` has no icon to pick.

# Examples

```rust
use crawrad_actions::items_editor::SelectedFields;

let fields = SelectedFields {
    can_add_child: false,
    text: Some("Item1".into()),
    expanded: None,
    image_index: None,
};
assert!(fields.has_selection());
```
