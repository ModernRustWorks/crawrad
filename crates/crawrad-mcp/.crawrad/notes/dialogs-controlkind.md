---
id: dialogs-controlkind
kind: doc
target_file: src/dialogs.rs
line_hint: 15
author: 
created: 
title: "ControlKind"
tags: []
signature: "pub enum ControlKind"
---

What a control is, which decides what is read from it.

# Examples

```rust
use crawrad_mcp::dialogs::ControlKind;

let kind = ControlKind::Button;
assert_eq!(kind, ControlKind::Button);
```
