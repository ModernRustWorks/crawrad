---
id: slash-commands-plan-auto-toggled-message
kind: doc
target_file: src/slash_commands.rs
line_hint: 326
author: 
created: 
title: "plan_auto_toggled_message"
tags: []
signature: "pub fn plan_auto_toggled_message (state : & str) -> String"
---

Feedback message when `/plan auto` toggles auto-approval.

# Examples

```rust
use crawrad_agent::slash_commands::plan_auto_toggled_message;

let msg = plan_auto_toggled_message("ENABLED");
assert!(msg.contains("ENABLED"));
```
