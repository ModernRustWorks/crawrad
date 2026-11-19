---
id: package-manager-packagemanagerstate-is-busy
kind: doc
target_file: src/package_manager.rs
line_hint: 390
author: 
created: 
title: "PackageManagerState::is_busy"
tags: []
signature: "pub fn is_busy (& self) -> bool"
---

Whether any thread of this panel is still out.

# Examples

```rust
use crawrad_project::package_manager::PackageManagerState;

let state = PackageManagerState::default();
assert!(!state.is_busy());
```
