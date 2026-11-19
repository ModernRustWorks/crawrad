---
id: host-shellextensionhost-contributed-actions
kind: doc
target_file: src/host.rs
line_hint: 323
author: 
created: 
title: "ShellExtensionHost::contributed_actions"
tags: []
signature: "pub fn contributed_actions (& self) -> Vec < & ActionContribution >"
---

List all actions contributed across registered extension manifests.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.contributed_actions().is_empty());
```
