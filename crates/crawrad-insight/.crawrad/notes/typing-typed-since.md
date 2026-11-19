---
id: typing-typed-since
kind: doc
target_file: src/typing.rs
line_hint: 45
author: 
created: 
title: "typed_since"
tags: []
signature: "pub fn typed_since (before_caret : & str , start : usize) -> Option < & str >"
---

Returns characters typed since the completion list opened at start.

What was typed since the list opened at `start` — `None` when the caret
left the name (moved back past `start`, or typed a space, `(`, `;`…):
then the list closes.

# Examples

```rust
use crawrad_insight::typing::typed_since;

assert_eq!(typed_since("hello", 2), Some("llo"));
assert_eq!(typed_since("hello world", 2), None);
```
