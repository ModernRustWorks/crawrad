---
id: toolbar-command-action
kind: doc
target_file: src/toolbar.rs
line_hint: 63
author: 
created: 
title: "Command::action"
tags: []
signature: "pub fn action (self) -> & 'static str"
---

The catalogue action this button runs.

Nothing is decided here beyond WHICH: the label, the shortcut, whether
it can run now and what it does are all the action's
(ADR 0005).

# Examples

```rust
use crawrad_project::toolbar::Command;

assert_eq!(Command::Undo.action(), "edit.undo");
```
