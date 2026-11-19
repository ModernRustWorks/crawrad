---
id: package-manager-packagemanagerstate-is-installed
kind: doc
target_file: src/package_manager.rs
line_hint: 616
author: 
created: 
title: "PackageManagerState::is_installed"
tags: []
signature: "pub fn is_installed (& self , name : & str) -> bool"
---

Returns whether this crate is already a dependency of the open project.

Is this crate already a dependency of the open project?

It is the model's question and not the painter's because BOTH shells
ask it — the egui card hides the `Add` button with it, and the Makepad
row picks its template with it.

# Examples

```rust
use crawrad_project::package_manager::PackageManagerState;

let state = PackageManagerState::default();
assert!(!state.is_installed("serde"));
```
