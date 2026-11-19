---
id: workspace-preferences-color-field-errors
kind: doc
target_file: src/workspace.rs
line_hint: 1089
author: 
created: 
title: "Preferences::color_field_errors"
tags: []
signature: "pub fn color_field_errors (inputs : & [String ; 15]) -> [Option < & 'static str > ; 15]"
---

Validates the fifteen color fields.

Validates the fifteen color fields, returning `Some(NOT_A_COLOUR)` for
any empty field or text that cannot be parsed as a color.

# Examples

```rust
use crawrad_project::workspace::Preferences;

let inputs: [String; 15] = std::array::from_fn(|_| "#112233".to_string());
assert_eq!(Preferences::color_field_errors(&inputs)[0], None);
```
