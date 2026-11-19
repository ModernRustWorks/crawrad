---
id: events-focus
kind: doc
target_file: src/events.rs
line_hint: 230
author: 
created: 
title: "Focus"
tags: []
signature: "pub enum Focus"
---

Which focus or keyboard event a control gets.

# Examples

```rust
use crawrad_runtime_makepad::events::Focus;

let focus = Focus::Enter;
assert_eq!(focus, Focus::Enter);
```
