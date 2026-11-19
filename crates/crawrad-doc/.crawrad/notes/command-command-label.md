---
id: command-command-label
kind: doc
target_file: src/command.rs
line_hint: 86
author: 
created: 
title: "Command::label"
tags: []
signature: "pub fn label (& self) -> String"
---

Short label for undo stack and audit log.

# Examples

```rust
use crawrad_doc::Command;

let cmd = Command::RemoveNode { node: "btn1".into() };
assert_eq!(cmd.label(), "remove btn1");
```
