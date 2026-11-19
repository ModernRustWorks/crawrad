---
id: host-shellextensionhost-line-mark-clicked
kind: doc
target_file: src/host.rs
line_hint: 240
author: 
created: 
title: "ShellExtensionHost::line_mark_clicked"
tags: []
signature: "pub fn line_mark_clicked (& self , extension : & str , id : & str , file : & str ,) -> Result < () , ExtensionError >"
---

A line mark's icon was clicked: its extension is told.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.line_mark_clicked("unknown", "mark1", "main.rs").is_err());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the extension is unknown or fails to receive the mark click event.
