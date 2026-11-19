---
id: dialogs-controlstate
kind: doc
target_file: src/dialogs.rs
line_hint: 104
author: 
created: 
title: "ControlState"
tags: []
signature: "pub struct ControlState"
---

What a control shows on screen, read by the shell (which owns the widgets).

# Examples

```rust
use crawrad_mcp::dialogs::ControlState;

let state = ControlState::default();
assert!(!state.visible);
```
