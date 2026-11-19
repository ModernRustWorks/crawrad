---
id: typing-typed-trigger
kind: doc
target_file: src/typing.rs
line_hint: 168
author: 
created: 
title: "typed_trigger"
tags: []
signature: "pub fn typed_trigger (before : & str , triggers : & [String]) -> Option < (char , Opening) >"
---

Detects trigger characters and determines list opening behavior.

The trigger the text before the caret just typed, if the server lists
it (LSP `triggerCharacters`) and it is code: `.` as [`opens_after_dot`]
says, `:` only as `::`, any other trigger quietly.

# Examples

```rust
use crawrad_insight::typing::{typed_trigger, Opening};

let triggers = vec![".".to_string()];
assert_eq!(typed_trigger("self.", &triggers), Some(('.', Opening::Loud)));
```
