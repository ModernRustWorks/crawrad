---
id: need-reason
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Need::reason"
tags: []
signature: "pub fn reason (self) -> & 'static str"
---

The missing thing, as it ends the sentence *"<Action> needs …"*.

# Examples

```rust
use crawrad_actions::Need;

assert_eq!(Need::Project.reason(), "an open project");
```
