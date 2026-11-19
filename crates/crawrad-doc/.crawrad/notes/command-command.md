---
id: command-command
kind: doc
target_file: src/command.rs
line_hint: 12
author: 
created: 
title: "Command"
tags: []
signature: "pub enum Command"
---

An elementary change in the document.

# Examples

```rust
use crawrad_doc::Command;

let cmd = Command::RemoveNode { node: "btn1".into() };
assert_eq!(cmd.label(), "remove btn1");
```
