---
id: host-shellextensionhost-execute-command-with-args
kind: doc
target_file: src/host.rs
line_hint: 279
author: 
created: 
title: "ShellExtensionHost::execute_command_with_args"
tags: []
signature: "pub fn execute_command_with_args (& self , action_id : & str , args : Option < serde_json :: Value > ,) -> Result < Option < serde_json :: Value > , ExtensionError >"
---

Execute a command with optional JSON arguments and receive structured result.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.execute_command_with_args("unknown.command", None).is_err());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the command cannot be found.
