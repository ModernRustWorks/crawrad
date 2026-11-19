---
id: panel-panel-from-id
kind: doc
target_file: src/panel.rs
line_hint: 126
author: 
created: 
title: "Panel::from_id"
tags: []
signature: "pub fn from_id (id : & str) -> Option < Self >"
---

The panel with this id, or `None`.

Returning `None` instead of a default is what lets the action answer
`no such panel: 'x'` — a caller on the other end of the protocol has no
screen to look at, and silently moving the wrong panel is worse than
refusing.

# Examples

```rust
use crawrad_actions::Panel;

assert_eq!(Panel::from_id("explorer"), Some(Panel::Explorer));
assert_eq!(Panel::from_id("invalid"), None);
```
