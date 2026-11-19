---
id: action-list-dialogstate
kind: doc
target_file: src/action_list.rs
line_hint: 195
author: 
created: 
title: "DialogState"
tags: []
signature: "pub struct DialogState"
---

What the ActionList / collection dialog shows (#936).

# Examples

```rust
use crawrad_actions::action_list::{COLLECTION_BUTTONS, state};
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let mut doc = Document::new("form1", "Form");
doc.add_child("form1", Node::new("grid1", "Grid")).unwrap();
let st = state(&doc, &Registry::builtin(), "grid1", &[], None);
assert_eq!(st.title, "grid1.Columns");
assert_eq!(st.buttons[0], COLLECTION_BUTTONS[0]);
assert_eq!(st.buttons[5], ""); // no design-time columns: no "Add all fields"
```
