---
id: agent-profile-build-platform-command
kind: doc
target_file: src/agent_profile.rs
line_hint: 772
author: 
created: 
title: "build_platform_command"
tags: []
signature: "pub fn build_platform_command (bin_path : & Path , args : & [String]) -> Command"
---

Constructs a command for native executables and batch files.

Constructs a `std::process::Command` supporting native executables and Windows batch files (.cmd/.bat).

# Examples

```rust
use crawrad_agent::agent_profile::build_platform_command;
use std::path::Path;

let cmd = build_platform_command(Path::new("agy"), &["--version".to_string()]);
assert_eq!(cmd.get_program(), "agy");
```
