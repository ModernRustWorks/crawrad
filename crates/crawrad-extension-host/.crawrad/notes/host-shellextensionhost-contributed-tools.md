---
id: host-shellextensionhost-contributed-tools
kind: doc
target_file: src/host.rs
line_hint: 347
author: 
created: 
title: "ShellExtensionHost::contributed_tools"
tags: []
signature: "pub fn contributed_tools (& self) -> Vec < & ToolContribution >"
---

List all tools contributed across registered extension manifests.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.contributed_tools().is_empty());
```
