---
id: session-launchconfig
kind: doc
target_file: src/session.rs
line_hint: 34
author: 
created: 
title: "LaunchConfig"
tags: []
signature: "pub struct LaunchConfig"
---

What to launch.

# Examples

```rust
use std::path::PathBuf;
use crawrad_dap::LaunchConfig;

let config = LaunchConfig {
    program: PathBuf::from("my_prog"),
    ..Default::default()
};
assert_eq!(config.program, PathBuf::from("my_prog"));
```
