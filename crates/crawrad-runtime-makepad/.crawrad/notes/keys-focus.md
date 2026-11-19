---
id: keys-focus
kind: doc
target_file: src/keys.rs
line_hint: 48
author: 
created: 
title: "Focus"
tags: []
signature: "pub enum Focus"
---

Which kind of text box holds the keyboard.

# Examples

```rust
use crawrad_runtime_makepad::keys::Focus;

let focus = Focus::Memo;
assert_eq!(focus, Focus::Memo);
```
