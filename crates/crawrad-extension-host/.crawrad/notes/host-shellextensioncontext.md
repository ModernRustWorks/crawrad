---
id: host-shellextensioncontext
kind: doc
target_file: src/host.rs
line_hint: 21
author: 
created: 
title: "ShellExtensionContext"
tags: []
signature: "pub struct ShellExtensionContext"
---

Thread-safe host context supplied to active extensions.

Thread-safe host context supplied to active extensions during initialization and command execution.

# Examples

```rust
use crawrad_extension_host::ShellExtensionContext;

let ctx = ShellExtensionContext::new(None);
assert!(ctx.workspace_path.is_none());
```
