---
id: events-focusrouter-focus
kind: doc
target_file: src/events.rs
line_hint: 271
author: 
created: 
title: "FocusRouter::focus"
tags: []
signature: "pub fn focus (& mut self , to : usize) -> Vec < (usize , Focus) >"
---

The pure core: moving the focus to `to`, as `(control, kind)`.

# Examples

```rust
use crawrad_runtime_makepad::events::{Focus, FocusRouter};

let mut router = FocusRouter::default();
let changes = router.focus(1);
assert_eq!(changes, vec![(1, Focus::Enter)]);
```
