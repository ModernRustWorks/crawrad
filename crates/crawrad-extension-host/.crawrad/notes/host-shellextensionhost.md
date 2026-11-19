---
id: host-shellextensionhost
kind: doc
target_file: src/host.rs
line_hint: 107
author: 
created: 
title: "ShellExtensionHost"
tags: []
signature: "pub struct ShellExtensionHost"
---

The IDE host runtime managing discovered manifests and plugins.

The IDE host runtime managing discovered manifests, installed plugins, and dynamic contributions.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.manifests.is_empty());
```
