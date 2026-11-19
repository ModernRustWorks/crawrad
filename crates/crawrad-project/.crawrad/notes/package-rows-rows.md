---
id: package-rows-rows
kind: doc
target_file: src/package_rows.rs
line_hint: 107
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (state : & model :: PackageManagerState , has_project : bool) -> Vec < Row >"
---

The whole panel, as a list of rows.

`root` is `None` when there is no project to add a package TO, and that is
the first thing both tabs answer: an `Add` button over no project would be
a control that accepts the gesture and changes nothing, which is the dead
control this shell keeps taking off the screen.

# Examples

```rust
use crawrad_project::package_manager::PackageManagerState;
use crawrad_project::package_rows::rows;

let state = PackageManagerState::default();
let list = rows(&state, false);
assert!(!list.is_empty());
```
