---
id: host-shellextensionhost-shutdown
kind: doc
target_file: src/host.rs
line_hint: 316
author: 
created: 
title: "ShellExtensionHost::shutdown"
tags: []
signature: "pub fn shutdown (& self)"
---

Gracefully shutdown all managed external extension subprocesses.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
host.shutdown();
```
