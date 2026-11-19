---
id: capability-capability-from-action
kind: doc
target_file: src/capability.rs
line_hint: 258
author: 
created: 
title: "Capability::from_action"
tags: []
signature: "pub fn from_action (action : & Action) -> Self"
---

Builds a native capability from a catalogue [`Action`].

# Examples

```rust
use crawrad_actions::capability::Capability;
use crawrad_actions::catalogue;

let acts = catalogue();
if let Some(act) = acts.first() {
    let cap = Capability::from_action(act);
    assert_eq!(cap.id, act.id);
}
```
