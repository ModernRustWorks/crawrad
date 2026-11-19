---
id: command-command-touches
kind: doc
target_file: src/command.rs
line_hint: 71
author: 
created: 
title: "Command::touches"
tags: []
signature: "pub fn touches (& self) -> Vec < NodeId >"
---

Affected nodes, so that observers know what to redraw.

# Examples

```rust
use crawrad_doc::Command;

let cmd = Command::RemoveNode { node: "btn1".into() };
assert_eq!(cmd.touches(), vec!["btn1"]);
```
