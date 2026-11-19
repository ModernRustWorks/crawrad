---
id: toolbar-command
kind: doc
target_file: src/toolbar.rs
line_hint: 27
author: 
created: 
title: "Command"
tags: []
signature: "pub enum Command"
---

One button of the strip.

# Examples

```rust
use crawrad_project::toolbar::Command;

assert_eq!(Command::Run.action(), "project.run");
```
