---
id: package-manager-packagemanagerstate-installed-count
kind: doc
target_file: src/package_manager.rs
line_hint: 381
author: 
created: 
title: "PackageManagerState::installed_count"
tags: []
signature: "pub fn installed_count (& self) -> Option < usize >"
---

Returns how many packages are installed, or `None` while reading.

How many packages are installed, or `None` while the FIRST read is
out — a tab saying `Installed (0)` over a project with dependencies
is the lie the owner caught on 16/09/2026.

# Examples

```rust
use crawrad_project::package_manager::PackageManagerState;

let state = PackageManagerState::default();
assert_eq!(state.installed_count(), Some(0));
```
