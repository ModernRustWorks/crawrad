---
id: layout-terminalpane
kind: doc
target_file: src/layout.rs
line_hint: 11
author: 
created: 
title: "TerminalPane"
tags: []
signature: "pub struct TerminalPane"
---

Information about a pane inside the terminal grid.

# Examples

```rust
use crawrad_terminal::TerminalPane;

let pane = TerminalPane {
    pane_id: 0,
    session_id: "s1".into(),
    is_focused: true,
};
assert!(pane.is_focused);
```
