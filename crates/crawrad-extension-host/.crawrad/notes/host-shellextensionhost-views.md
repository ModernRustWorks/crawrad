---
id: host-shellextensionhost-views
kind: doc
target_file: src/host.rs
line_hint: 221
author: 
created: 
title: "ShellExtensionHost::views"
tags: []
signature: "pub fn views (& self) -> Option < Arc < Mutex < crate :: views :: ViewStore > > >"
---

Every extension's views (#398).

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.views().is_some());
```
