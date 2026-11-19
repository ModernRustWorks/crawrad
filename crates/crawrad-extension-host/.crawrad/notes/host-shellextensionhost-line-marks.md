---
id: host-shellextensionhost-line-marks
kind: doc
target_file: src/host.rs
line_hint: 216
author: 
created: 
title: "ShellExtensionHost::line_marks"
tags: []
signature: "pub fn line_marks (& self) -> Option < Arc < Mutex < crate :: line_marks :: LineMarkStore > > >"
---

Every extension's line marks (#398).

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.line_marks().is_some());
```
