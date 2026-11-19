---
id: typing-opens-after-dot
kind: doc
target_file: src/typing.rs
line_hint: 51
author: 
created: 
title: "opens_after_dot"
tags: []
signature: "pub fn opens_after_dot (before_caret : & str) -> bool"
---

Returns whether typing dot triggers auto-opening of the completion list.

Whether the text before the caret just typed the `.` of a member access —
`self.`, `btn_ok.`, `name().`, `"text".` — so the list opens by itself, as a modern editor's.
A number (`0.`) and a range (`..`) do not.

# Examples

```rust
use crawrad_insight::typing::opens_after_dot;

assert!(opens_after_dot("self."));
assert!(!opens_after_dot("0."));
```
