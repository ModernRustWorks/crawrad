---
id: package-manager-packagemanagerstate-installed-req
kind: doc
target_file: src/package_manager.rs
line_hint: 621
author: 
created: 
title: "PackageManagerState::installed_req"
tags: []
signature: "pub fn installed_req (& self , name : & str) -> Option < & str >"
---

Returns installed version requirement for this package in manifest, if any.

Returns installed version requirement for this package in manifest (e.g. "^0.36"), if any.

# Examples

```rust
use crawrad_project::package_manager::PackageManagerState;

let state = PackageManagerState::default();
assert_eq!(state.installed_req("serde"), None);
```
