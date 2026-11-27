---
id: workspace-config-bare-command
kind: doc
target_file: src/workspace_config.rs
line_hint: 13
author: 
created: 
title: "bare_command"
tags: []
signature: "pub fn bare_command () -> String"
---

The bridge's bare name, which resolves on PATH.

On a packaged install this is the execution alias Windows places in
`%LOCALAPPDATA%\Microsoft\WindowsApps\`, declared by the MSIX manifest; on every
other install it is the binary beside the IDE, found on PATH as before.

# Examples

```rust
use crawrad_mcp::workspace_config::bare_command;

let cmd = bare_command();
assert!(cmd.starts_with("crawrad-mcp-bridge"), "the name the alias is declared under");
assert!(!cmd.contains('\') && !cmd.contains('/'), "a bare name, never a path");
```
