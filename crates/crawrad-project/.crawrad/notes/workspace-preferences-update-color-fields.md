---
id: workspace-preferences-update-color-fields
kind: doc
target_file: src/workspace.rs
line_hint: 1057
author: 
created: 
title: "Preferences::update_color_fields"
tags: []
signature: "pub fn update_color_fields (& mut self , inputs : & [String ; 15]) -> bool"
---

Updates from the page's inputs, in [`Self::color_fields`] order.

Updates from the page's inputs, in [`Self::color_fields`] order. A
syntax colour changed turns the scheme `Custom`; the diagnostic ones
stand apart from the scheme. Returns whether anything changed.

# Examples

```rust
use crawrad_project::workspace::Preferences;

let mut p = Preferences::default();
let inputs: [String; 15] = std::array::from_fn(|_| "#112233".to_string());
assert!(p.update_color_fields(&inputs));
```
