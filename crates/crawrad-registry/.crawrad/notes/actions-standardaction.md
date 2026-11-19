---
id: actions-standardaction
kind: doc
target_file: src/actions.rs
line_hint: 20
author: 
created: 
title: "StandardAction"
tags: []
signature: "pub struct StandardAction"
---

A standard-action template for pre-defined actions.

A standard-action template — the preset catalog of pre-defined actions.
It is not a new class: it is a preset of the `Action` component that fills
caption, hint, shortcut and icon in one go.

# Examples

```rust
use crawrad_registry::StandardAction;

let action = StandardAction {
    name: "Action",
    base_name: "action",
    category: "(No Category)",
    text: "Action",
    hint: "Standard generic action",
    shortcut: "",
    image_index: -1,
    dataset_operation: "None",
};
assert_eq!(action.base_name, "action");
```
