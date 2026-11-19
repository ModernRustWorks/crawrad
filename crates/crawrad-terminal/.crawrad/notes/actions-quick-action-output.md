---
id: actions-quick-action-output
kind: doc
target_file: src/actions.rs
line_hint: 83
author: 
created: 
title: "quick_action_output"
tags: []
signature: "pub fn quick_action_output (action : & str) -> Option < & 'static str >"
---

Simulated output for sidebar quick actions in the integrated terminal.

# Examples

```rust
use crawrad_terminal::actions::quick_action_output;

let output = quick_action_output("pull");
assert!(output.is_some());
```
