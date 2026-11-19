---
id: host-shellextensionhost-contributed-menus
kind: doc
target_file: src/host.rs
line_hint: 331
author: 
created: 
title: "ShellExtensionHost::contributed_menus"
tags: []
signature: "pub fn contributed_menus (& self) -> Vec < & MenuContribution >"
---

List all menus contributed across registered extension manifests.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.contributed_menus().is_empty());
```
