---
id: host-shellextensionhost-execute-command
kind: doc
target_file: src/host.rs
line_hint: 254
author: 
created: 
title: "ShellExtensionHost::execute_command"
tags: []
signature: "pub fn execute_command (& self , action_id : & str) -> Result < () , ExtensionError >"
---

Execute a contributed command by its action ID.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.execute_command("unknown.command").is_err());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the command action ID cannot be resolved.
