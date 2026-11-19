---
id: package-manager-packagemanagerstate-reload-installed
kind: doc
target_file: src/package_manager.rs
line_hint: 350
author: 
created: 
title: "PackageManagerState::reload_installed"
tags: []
signature: "pub fn reload_installed (& mut self , root : Option < & Path > , target_package : Option < & str >)"
---

Re-reads the installed list off the UI thread when a manifest changed.

Re-reads the installed list **off the UI thread**, and only when a
manifest changed since the last read ([`InstalledKey`]).

`cargo metadata` used to run here and WAIT, on the thread that draws:
measured on 30/09/2026, 55-60 ms on every show of the panel, 339 ms
cold. The old list stays on screen while the new one is read; the
answer arrives through [`Self::update_channels`].

# Examples

```rust
use crawrad_project::package_manager::PackageManagerState;

let mut state = PackageManagerState::default();
state.reload_installed(None, None);
assert!(state.installed.is_empty());
```
