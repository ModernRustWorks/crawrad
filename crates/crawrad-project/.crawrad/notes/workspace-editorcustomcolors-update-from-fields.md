---
id: workspace-editorcustomcolors-update-from-fields
kind: doc
target_file: src/workspace.rs
line_hint: 731
author: 
created: 
title: "EditorCustomColors::update_from_fields"
tags: []
signature: "pub fn update_from_fields (& mut self , inputs : & [String ; 9]) -> bool"
---

Updates the colors from the inputs, in [`Self::fields`] order.

Updates the colors from the inputs, in [`Self::fields`] order; an empty
input or one that is not a colour keeps its color. Returns whether any
color changed.

# Examples

```rust
use crawrad_project::workspace::EditorCustomColors;

let mut c = EditorCustomColors::default();
let inputs: [String; 9] = std::array::from_fn(|_| "#112233".to_string());
assert!(c.update_from_fields(&inputs));
assert_eq!(c.keyword, "#112233");
```
