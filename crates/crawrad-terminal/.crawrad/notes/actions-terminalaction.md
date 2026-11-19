---
id: actions-terminalaction
kind: doc
target_file: src/actions.rs
line_hint: 2
author: 
created: 
title: "TerminalAction"
tags: []
signature: "pub struct TerminalAction"
---

A predefined runnable terminal action.

# Examples

```rust
use crawrad_terminal::TerminalAction;

let action = TerminalAction {
    id: "pull".into(),
    title: "Pull".into(),
    command: "git pull".into(),
    category: "Git".into(),
};
assert_eq!(action.id, "pull");
```
