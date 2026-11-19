---
id: explorer-rows-problems-state-icon
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 74
author: 
created: 
title: "state_icon"
tags: []
signature: "pub fn state_icon (problems : Problems , unsaved : bool) -> Option < RowIcon >"
---

Icon for a file state based on errors, warnings, or unsaved edits.

The icon of a file's state, when errors, warnings or unsaved edits take
precedence over its kind drawing: error > warning > unsaved.

# Examples

```rust
use crawrad_project::explorer_rows::icon::ERROR;
use crawrad_project::explorer_rows::problems::{state_icon, Problems};

let p = Problems { errors: 1, warnings: 0 };
assert_eq!(state_icon(p, false), Some(ERROR));
```
