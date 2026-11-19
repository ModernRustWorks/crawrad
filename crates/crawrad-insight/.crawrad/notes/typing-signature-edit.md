---
id: typing-signature-edit
kind: doc
target_file: src/typing.rs
line_hint: 224
author: 
created: 
title: "signature_edit"
tags: []
signature: "pub fn signature_edit (before : & str , triggers : & [String] , open : bool) -> Option < SignatureEdit >"
---

Determines how an edit affects parameter hint card display.

The parameter card's answer to an edit that left `before` the caret.
`open` is whether a card is up or asked for; with none, only a trigger
typed in code asks — not one inside a string or a comment.

# Examples

```rust
use crawrad_insight::typing::{signature_edit, SignatureEdit};

let triggers = vec!["(".to_string()];
assert_eq!(signature_edit("foo(", &triggers, false), Some(SignatureEdit::Typed('(')));
```
