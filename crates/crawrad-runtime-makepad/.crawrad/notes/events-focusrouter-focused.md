---
id: events-focusrouter-focused
kind: doc
target_file: src/events.rs
line_hint: 266
author: 
created: 
title: "FocusRouter::focused"
tags: []
signature: "pub fn focused (& self) -> Option < usize >"
---

The control that has the focus.

# Examples

```rust
use crawrad_runtime_makepad::events::FocusRouter;

let router = FocusRouter::default();
assert_eq!(router.focused(), None);
```
