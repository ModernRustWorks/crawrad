---
id: inspector-model-action-toggle-category
kind: doc
target_file: src/inspector_model.rs
line_hint: 128
author: 
created: 
title: "ACTION_TOGGLE_CATEGORY"
tags: []
signature: "pub const ACTION_TOGGLE_CATEGORY : & str = \"inspector.toggle_category\""
---

The one that folds a whole CATEGORY away — `Layout`, `Advanced`.

Sibling of [`ACTION_TOGGLE_GROUP`] and not the same id: a group is one
property with sub-rows, a category is the heading over unrelated ones. See
`crate::InspectorCommand::ToggleCategory`.

# Examples

```rust
use crawrad_actions::inspector_model::ACTION_TOGGLE_CATEGORY;

assert_eq!(ACTION_TOGGLE_CATEGORY, "inspector.toggle_category");
```
