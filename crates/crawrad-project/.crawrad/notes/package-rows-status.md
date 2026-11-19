---
id: package-rows-status
kind: doc
target_file: src/package_rows.rs
line_hint: 197
author: 
created: 
title: "status"
tags: []
signature: "pub fn status (state : & model :: PackageManagerState) -> & str"
---

What the status line of the panel says right now.

One line, and it is the same precedence the egui window uses: the error
first, then whatever the last action reported, then [`model::READY`]. A
panel that goes quiet after a failed `cargo add` is a panel that lost the
only sentence explaining why nothing happened.

# Examples

```rust
use crawrad_project::package_manager::PackageManagerState;
use crawrad_project::package_rows::status;

let state = PackageManagerState::default();
assert_eq!(status(&state), "Ready");
```
