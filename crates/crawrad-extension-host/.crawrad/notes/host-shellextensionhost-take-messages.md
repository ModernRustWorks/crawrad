---
id: host-shellextensionhost-take-messages
kind: doc
target_file: src/host.rs
line_hint: 307
author: 
created: 
title: "ShellExtensionHost::take_messages"
tags: []
signature: "pub fn take_messages (& self) -> Vec < (MessageLevel , String) >"
---

Retrieve and clear messages from host context and subprocesses.

Retrieve and clear messages from both in-process context and external subprocesses.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.take_messages().is_empty());
```
