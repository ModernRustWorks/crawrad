---
id: terminalmodel
kind: doc
target_file: src/lib.rs
line_hint: 12
author: 
created: 
title: "TerminalModel"
tags: []
signature: "pub struct TerminalModel"
---

Primary data and state model for the integrated Aefos Terminal.

# Examples

```rust
use crawrad_terminal::TerminalModel;

let model = TerminalModel::new();
assert_eq!(model.sessions.len(), 1);
```
