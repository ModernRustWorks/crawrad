---
id: slash-commands-auto-approve-labels
kind: doc
target_file: src/slash_commands.rs
line_hint: 298
author: 
created: 
title: "auto_approve_labels"
tags: []
signature: "pub fn auto_approve_labels (on : bool) -> (& 'static str , & 'static str)"
---

`/plan auto`'s words for its state and the plan card's button.

# Examples

```rust
use crawrad_agent::slash_commands::auto_approve_labels;

let (state, button) = auto_approve_labels(true);
assert_eq!(state, "ENABLED (ON)");
assert_eq!(button, "Auto-approve: ON");
```
